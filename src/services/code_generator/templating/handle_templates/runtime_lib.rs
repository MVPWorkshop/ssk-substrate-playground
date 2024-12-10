use std::collections::HashSet;

use crate::services::code_generator::types::PalletConfig;
use dyn_fmt::AsStrFormatExt;
use serde::Serialize;

use super::{render_handlebars_template_to_bytes, TemplateRenderError};

#[derive(Debug, Serialize)]
pub struct RuntimeImplBlocks {
    pub additional_pallet_impl_code: Option<String>,
    pub pallet_name: String,
    pub pallet_traits: Option<Vec<String>>,
    pub configurable_parameter_types: Option<Vec<String>>,
    pub is_instance: bool,
    pub alias_name: String,
    pub instance_counter: u8,
}

#[derive(Debug, Serialize)]
pub struct RuntimeLibAggregate {
    pub additional_runtime_lib_code: Vec<String>, // needs to be distinct
    pub impl_blocks: Vec<RuntimeImplBlocks>,
    pub construct_runtime: Vec<String>,
}

fn transform_name(input: &str) -> (String, String, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() == 2 {
        let part1 = parts[0].to_string(); // "GeneralCouncil"
        let part2 = format!("pallet_{}", parts[1].to_lowercase()); // "pallet_collective"
        let combined = format!("{}{}", part1, parts[1]); // "GeneralCouncilCollective"
        (part1, part2, combined)
    } else {
        (String::new(), String::new(), String::new())
    }
}

impl From<Vec<PalletConfig>> for RuntimeLibAggregate {
    fn from(pallets: Vec<PalletConfig>) -> Self {
        let mut additional_runtime_lib_code = HashSet::new(); // makes sure that the code is distinct
        let mut impl_blocks = vec![];
        let mut construct_runtime = vec![];

        let mut instance_counter = 0;
        for (index, pallet) in pallets.iter().enumerate() {
            // add the pallet runtime code
            if let Some(code) = pallet.runtime.additional_runtime_lib_code.clone() {
                for line in code {
                    additional_runtime_lib_code.insert(line);
                }
            };

            let is_instance = pallet.metadata.is_instance.unwrap_or_default();
            let (part1, pallet_name, together) = transform_name(&pallet.name);
            if is_instance {
                instance_counter += 1;
                construct_runtime.push(format!(
                    "\n\t#[runtime::pallet_index({})]\n\tpub type {} = {}<Instance{}>;",
                    index + 20,
                    part1,
                    pallet_name,
                    instance_counter
                ));
            } else {
                construct_runtime.push(format!(
                    "\n\t#[runtime::pallet_index({})]\n\tpub type {} = {};",
                    index + 20,
                    pallet.runtime.construct_runtime.runtime[0],
                    pallet.runtime.construct_runtime.runtime[1],
                ));
            }
            let additional_pallet_impl_code = pallet.runtime.additional_pallet_impl_code.clone();
            let pallet_name = pallet
                .dependencies
                .pallet
                .alias
                .clone()
                .to_lowercase()
                .replace(" ", "_");
            let pallet_traits = pallet
                .runtime
                .pallet_traits
                .iter()
                .map(|(key, value)| format!("\ttype {} = {};\n", key, value))
                .collect::<Vec<_>>();
            let pallet_traits = if pallet_traits.is_empty() {
                None
            } else {
                Some(pallet_traits)
            };

            let configurable_parameter_types = if let Some(optional_parameter_types) =
                pallet.runtime.optional_parameter_types.clone()
            {
                let mut temp = vec![];
                for (_, pt) in optional_parameter_types {
                    let m = match pt.expression.configured_multiplier {
                        Some(v) => v.to_string(),
                        None => match pt.expression.default_multiiplier {
                            Some(v) => v.to_string(),
                            None => "".to_string(),
                        },
                    };
                    let u = match pt.expression.configured_unit {
                        Some(u) => u,
                        None => pt.expression.default_unit,
                    };
                    let s = format!(
                        "    pub{}{}: {} = {};",
                        pt.prefix,
                        pt.name,
                        pt.p_type,
                        pt.expression.format.format(&[u, m])
                    );
                    temp.push(s);
                }
                Some(temp)
            } else {
                None
            };
            impl_blocks.push(RuntimeImplBlocks {
                // param_types!
                additional_pallet_impl_code,
                // pallet_name
                pallet_name,
                // Lines
                pallet_traits,
                // configurable traits
                configurable_parameter_types,
                // is_instance
                is_instance,
                // type alias_name = pallet_collective::Instance1
                alias_name: together,
                // index of pallet
                instance_counter,
            });
        }
        RuntimeLibAggregate {
            additional_runtime_lib_code: additional_runtime_lib_code.into_iter().collect(),
            impl_blocks,
            construct_runtime,
        }
    }
}

pub fn generate_runtime_lib_file_bytes(
    runtime_lib_file_path: &str,
    pallet_configs: &[PalletConfig],
) -> Result<Vec<u8>, TemplateRenderError> {
    let runtime_lib_aggregate: RuntimeLibAggregate = pallet_configs.to_vec().into();
    render_handlebars_template_to_bytes(runtime_lib_file_path, &runtime_lib_aggregate)
}
