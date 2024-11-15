use std::collections::HashSet;

use dyn_fmt::AsStrFormatExt;
use serde::Serialize;

use crate::types::PalletConfig;

use super::{render_handlebars_template, TemplateRenderError};

#[derive(Debug, Serialize)]
pub struct RuntimeImplBlocks {
    pub additional_pallet_impl_code: Option<String>,
    pub pallet_name: String,
    pub pallet_traits: Option<Vec<String>>,
    pub configurable_parameter_types: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct RuntimeLibAggregate {
    pub additional_runtime_lib_code: Vec<String>, // needs to be distinct
    pub impl_blocks: Vec<RuntimeImplBlocks>,
    pub construct_runtime: Vec<String>,
}

impl From<Vec<PalletConfig>> for RuntimeLibAggregate {
    fn from(pallets: Vec<PalletConfig>) -> Self {
        let mut additional_runtime_lib_code = HashSet::new(); // makes sure that the code is distinct
        let mut impl_blocks = vec![];
        let mut construct_runtime = vec![];

        for (index, pallet) in pallets.iter().enumerate() {
            // add the pallet runtime code
            if let Some(code) = pallet.runtime.additional_runtime_lib_code.clone() {
                for line in code {
                    additional_runtime_lib_code.insert(line);
                }
            };
            construct_runtime.push(format!(
                "\n\t#[runtime::pallet_index({})]\n\tpub type {} = {};",
                index + 9,
                pallet.runtime.construct_runtime.runtime[0],
                pallet.runtime.construct_runtime.runtime[1],
            ));
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
                for pt in optional_parameter_types {
                    let v = match pt.expression.default_multiiplier {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    };
                    let s = format!(
                        "    pub{}{}: {} = {};",
                        pt.prefix,
                        pt.name,
                        pt.p_type,
                        pt.expression
                            .format
                            .format(&[pt.expression.default_unit, v])
                    );
                    temp.push(s);
                }
                Some(temp)
            } else {
                None
            };
            impl_blocks.push(RuntimeImplBlocks {
                additional_pallet_impl_code,
                pallet_name,
                pallet_traits,
                configurable_parameter_types,
            });
        }
        RuntimeLibAggregate {
            additional_runtime_lib_code: additional_runtime_lib_code.into_iter().collect(),
            impl_blocks,
            construct_runtime,
        }
    }
}

pub fn generate_runtime_lib_file(
    runtime_lib_file_path: &str,
    pallet_configs: &[PalletConfig],
) -> Result<(), TemplateRenderError> {
    let runtime_lib_aggregate: RuntimeLibAggregate = pallet_configs.to_vec().into();
    render_handlebars_template(runtime_lib_file_path, &runtime_lib_aggregate)
}
