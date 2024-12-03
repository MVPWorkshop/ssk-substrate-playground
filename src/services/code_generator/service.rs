use std::path::Path;
use std::{collections::HashMap, sync::Arc};

use super::load_configs::load_configs;
use super::load_templates::load_templates;
use super::templating::handle_templates::manifest::generate_manifest_file_to_bytes;
use super::templating::handle_templates::runtime_lib::generate_runtime_lib_file_bytes;
use super::templating::handle_templates::HBS_SUFFIX;
use super::types::TemplateType;
use super::{CodeGeneratorServiceError, Result};
use crate::api::handlers::generate_project_handler::ParameterConfiguration;
use crate::services::archiver::ArchiverService;
use async_trait::async_trait;

use super::{types::PalletConfig, CodeGenerator};

pub struct CodeGeneratorService<ZB: 'static> {
    #[allow(dead_code)]
    config_directory: String,
    templates_directory: String,
    pallet_configs: HashMap<String, PalletConfig>,
    // TODO: change this to Vec<TemplateType>
    templates:Vec<TemplateType>,
    archiver_service: Arc<dyn ArchiverService<ZippedBuffer = ZB>>,
}

impl<ZB: 'static + Send> CodeGeneratorService<ZB> {
    pub async fn try_new(
        archiver_service: Arc<dyn ArchiverService<ZippedBuffer = ZB>>,
    ) -> Result<Self> {
        let config_directory = std::env::var("CONFIG_DIRECTORY")?;
        let templates_directory = std::env::var("TEMPLATES_DIRECTORY")?;
        let pallet_configs = load_configs(Path::new(&config_directory).to_path_buf()).await?;
        let templates = load_templates(Path::new(&templates_directory).to_path_buf()).await?;

        Ok(Self {
            config_directory,
            templates_directory,
            pallet_configs,
            templates,
            archiver_service,
        })
    }
    fn filter_configs(&self, filter: Vec<String>,) -> Result<HashMap<String, PalletConfig>> {
        // Check if the pallets are supported
        for pallet_name in filter.iter() {
            if !self.pallet_configs.contains_key(pallet_name) {
                return Err(CodeGeneratorServiceError::PalletNotFoundError(
                    pallet_name.clone(),
                ));
            }
        }

        // Get the required pallets for the pallets in the list
        let  filtered: Vec<String> = self
            .pallet_configs
            .iter()
            // Get the pallets that are in the list of pallet names
            .filter(|(name, _)| filter.contains(*name))
            // Get the required pallets for each pallet
            .flat_map(|(pallet_name, pallet)| {
                let mut pallet_with_reqs = vec![pallet_name.clone()];
                if let Some(required_pallets) = pallet.dependencies.required_pallets.clone() {
                    pallet_with_reqs.extend(required_pallets);
                }
                pallet_with_reqs
            })
            .collect::<Vec<String>>();

            // let essential = self
            // .pallet_configs
            // .iter()
            // .filter(|(_, config)| {
            //     if let Some(essential_templates) = &config.metadata.is_essential {
            //         essential_templates.contains(&template_type)
            //     } else {
            //         false
            //     }
            // })
            // .map(|(pallet_name, _)| pallet_name.clone())
            // .collect::<Vec<_>>();
        

        // create local coppy of the pallets
        Ok(self
            .pallet_configs
            .iter()
            .filter(|(name, _)| filtered.contains(*name))
            .map(|(name, config)| (name.clone(), config.clone()))
            .collect::<HashMap<_, _>>())
    }
    pub async fn add_pallets_to_archive(
        &self,
        zipper_buffer: ZB,
        pallet_configs: Vec<PalletConfig>,
        template_type: TemplateType, 
    ) -> Result<ZB>
    where
        ZB: 'static + Send,
    {
        let manifest_file_path = format!(
            "templates/{}/runtime/Cargo.toml.hbs",
            template_type
        );
        let manifest_file_content =
            generate_manifest_file_to_bytes(&manifest_file_path, &pallet_configs).unwrap();
        let zipper_buffer = self
            .archiver_service
            .add_content_to_archive(
                zipper_buffer,
                &manifest_file_content,
                Path::new("runtime/Cargo.toml"),
            )
            .await?;
    
        let runtime_lib_file_path = format!(
            "templates/{}/runtime/src/lib.rs.hbs",
            template_type
        );
        let runtime_lib_file_content =
            generate_runtime_lib_file_bytes(&runtime_lib_file_path, &pallet_configs).unwrap();
        let zipper_buffer = self
            .archiver_service
            .add_content_to_archive(
                zipper_buffer,
                &runtime_lib_file_content,
                Path::new("runtime/src/lib.rs"),
            )
            .await?;
    
        Ok(zipper_buffer)
    }
    
    fn apply_configs(
        &self,
        parameter_configs: &HashMap<String, Option<HashMap<String, ParameterConfiguration>>>
    ) -> Result<Vec<PalletConfig>> {
        let mut filtered_configs =
            self.filter_configs(parameter_configs.keys().cloned().collect())?;
        parameter_configs
            .iter()
            .filter_map(|(name, config)| config.clone().map(|config| (name, config)))
            .for_each(|(name, config)| {
                let pallet_to_configure = filtered_configs.get_mut(name).unwrap();
                for input in config {
                    if let Some(parameter) = pallet_to_configure
                        .runtime
                        .optional_parameter_types
                        .as_mut()
                        .unwrap()
                        .get_mut(&input.0)
                    {
                        parameter.expression.configured_multiplier = input.1.multiplier;
                        parameter.expression.configured_unit = input.1.unit;
                    }
                }
            });
        Ok(filtered_configs.values().cloned().collect::<Vec<_>>())
    }
}
#[async_trait]
impl<ZB: 'static + Send> CodeGenerator for CodeGeneratorService<ZB> {
    fn pallet_configs(&self) -> &HashMap<String, PalletConfig> {
        &self.pallet_configs
    }

    fn templates(&self) -> &Vec<TemplateType> {
        &self.templates
    }

    async fn generate_project_archive(
        &self,
        pallets: &HashMap<String, Option<HashMap<String, ParameterConfiguration>>>,
        template_type: TemplateType,
    ) -> Result<Vec<u8>> {
        let pallets = self.apply_configs(pallets)?;
    
        if !self.templates.contains(&template_type) {
            return Err(CodeGeneratorServiceError::InvalidTemplateType(format!(
                "{:?}",
                template_type
            )));
        }
    
        let template_path = Path::new(&self.templates_directory)
            .join(template_type.to_string());
    
        let zipped_buffer = self
            .archiver_service
            .archive_folder(template_path.as_path(), HBS_SUFFIX)
            .await?;
    
        let zipped_buffer = self
            .add_pallets_to_archive(zipped_buffer, pallets, template_type)
            .await?;
        let zipped_data = self.archiver_service.close_archive(zipped_buffer).await?;
        Ok(zipped_data)
    }
    
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::services::{
//         archiver::async_zip::AsyncZipArchiverService,
//         code_generator::{
//             service::CodeGeneratorService, templating::handle_templates::HBS_SUFFIX, CodeGenerator,
//         },
//     };
//     #[tokio::test]
//     async fn test_add_archived_pallets() {
//         dotenv::from_filename(".env.local").ok();
//         let archiver = Arc::new(AsyncZipArchiverService);
//         let cg = CodeGeneratorService::try_new(archiver.clone()).await;
//         assert!(cg.is_ok());
//         let cg = cg.unwrap();
//         let pallets = cg.pallet_configs();
//         let pallets = pallets.iter().map(|(_, v)| v.clone()).collect();
//         let zipper_buffer = archiver
//             .archive_folder(Path::new("templates/solochain/basic"), HBS_SUFFIX)
//             .await;
//         let cg = CodeGeneratorService::try_new(archiver.clone()).await;
//         assert!(cg.is_ok());
//         let cg = cg.unwrap();
//         let zipper_buffer = cg
//             .add_pallets_to_archive(zipper_buffer.unwrap(), pallets)
//             .await;
//         // save zipper_buffer to file test.zip in root directory
//         let bytes = archiver.close_archive(zipper_buffer.unwrap()).await;
//         assert!(bytes.is_ok());
//     }
//     #[tokio::test]
//     async fn test_filter_configs() {
//         dotenv::from_filename(".env.local").ok();
//         let archiver = Arc::new(AsyncZipArchiverService);
//         let cg = CodeGeneratorService::try_new(archiver.clone()).await;
//         assert!(cg.is_ok());
//         let cg = cg.unwrap();
//         let filtered = cg.filter_configs(vec!["Pallet Bounties".to_string()]);
//         assert!(filtered.is_ok());
//         let filtered = filtered.unwrap();
//         assert_eq!(filtered.len(), 9);
//     }
// }
