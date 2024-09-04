use crate::types::*;
use regex::Regex;

pub struct GeneratedRuntime {
    pub updated_runtime_code: String,
    pub updated_chain_spec_code: String,
}
pub struct SubstrateRuntimeUtil {
    pallet_config: PalletConfig,
    runtime_code: String,
    chain_spec_code: String,
    regex: RegexCollection,
}

#[allow(unused)]
struct RegexCollection {
    pallet_trait: Regex,
    construct_runtime: Regex,
    additional_code: Regex,
    additional_genesis_variables: Regex,
    genesis_config: Regex,
    runtime_api: Regex,
}

impl SubstrateRuntimeUtil {
    pub fn new(pallet_config: PalletConfig, runtime_code: String, chain_spec_code: String) -> Self {
        let regex = RegexCollection {
            pallet_trait: Regex::new(&format!(
                r"impl\s+{}::Config\s+for\s+Runtime\s*\{{[\s\S]+?\}}",
                pallet_config.dependencies.pallet.alias
            ))
            .unwrap(),
            construct_runtime: Regex::new(r"mod\s+runtime\s*\{[\s\S]+?\}").unwrap(),
            additional_code: Regex::new(r"\A(?:.*\n){4}").unwrap(),
            additional_genesis_variables: Regex::new(r"(?s)(^.*\n?){3}").unwrap(),
            genesis_config: Regex::new(r"serde_json::json!\(\{(?:[^{}]|\{[^{}]*\})*\}\)").unwrap(),
            runtime_api: Regex::new(r"(?s)impl_runtime_apis!.*").unwrap(),
        };

        if !regex.construct_runtime.is_match(&runtime_code) {
            panic!("Couldn't find construct_runtime inside the provided code");
        }

        SubstrateRuntimeUtil {
            pallet_config,
            runtime_code,
            chain_spec_code,
            regex,
        }
    }

    pub fn pallet_name(&self) -> &String {
        &self.pallet_config.name
    }

    pub fn pallet_alias(&self) -> &String {
        &self.pallet_config.dependencies.pallet.alias
    }

    pub fn check_if_pallet_implemented(&self) -> bool {
        self.regex.pallet_trait.is_match(&self.runtime_code)
    }

    fn update_construct_runtime(&mut self, current_code: String, new_code: String) {
        self.runtime_code = self.runtime_code.replace(&current_code, &new_code);
    }

    fn add_pallet_traits(&mut self) {
        let mut trait_implementation = format!(
            "\n\nimpl {}::Config for Runtime {{\n",
            to_snake_case(&self.pallet_alias())
        );
        // let mut parameter_types = String::from("parameter_types! {\n");

        let mut custom_parameter_counter = 0;

        for (trait_name, trait_value) in &self.pallet_config.runtime.pallet_traits {
            if trait_value.is_empty() {
                trait_implementation.push_str(&format!(
                    "{}type {} = {};\n",
                    tabs(1),
                    trait_name,
                    trait_value
                ));
            } else {
                let param_name = trait_value.clone();

                // Commented the code for parameter types for pallet trait configs.
                // parameter_types.push_str(&format!(
                //     "{}pub {} {}: {} = {};\n",
                //     tabs(1),
                //     if trait_value.is_empty() { "" } else { "const" },
                //     param_name,
                //     trait_value.clone(),
                //     trait_value.clone(),
                // ));

                trait_implementation.push_str(&format!(
                    "{}type {} = {};\n",
                    tabs(1),
                    trait_name,
                    param_name
                ));

                custom_parameter_counter += 1;
            }
        }

        trait_implementation.push_str("}\n\n");
        // parameter_types.push_str("}\n\n");

        let current_construct_runtime = self
            .regex
            .construct_runtime
            .find(&self.runtime_code)
            .unwrap()
            .as_str()
            .to_string();

        let mut construct_runtime = current_construct_runtime.clone() + &trait_implementation;

        if custom_parameter_counter > 0 {
            // construct_runtime = parameter_types + &construct_runtime;
            construct_runtime = construct_runtime;
        }

        self.update_construct_runtime(current_construct_runtime, construct_runtime);
    }

    fn add_pallet_to_construct_runtime(&mut self) {
        let construct_runtime_module = format!(
            "{} #[runtime::pallet_index({})]\n {}pub type {} = {};",
            tabs(1),
            self.pallet_config.runtime.construct_runtime.index.unwrap(),
            tabs(1),
            self.pallet_config.runtime.construct_runtime.runtime.0,
            self.pallet_config.runtime.construct_runtime.runtime.1,
        );

        let construct_runtime = self
            .regex
            .construct_runtime
            .find(&self.runtime_code)
            .unwrap()
            .as_str()
            .to_string();
        let update = format!(
            "{}\n\n{}{}",
            &construct_runtime[..construct_runtime.len() - 2],
            construct_runtime_module,
            &construct_runtime[construct_runtime.len() - 2..],
        );

        self.update_construct_runtime(construct_runtime, update);
    }

    fn add_additional_code(
        &mut self,
        existing_code: String,
        additional_code: &Option<Vec<String>>,
        test_regex: &Regex,
    ) -> String {
        if additional_code.is_none() {
            return existing_code;
        }

        let mut additional_runtime_code = String::new();

        for code in additional_code.as_ref().unwrap() {
            additional_runtime_code.push_str(&format!("{}\n", code));
        }

        if let Some(additional_code_regex) = test_regex.find(&existing_code) {
            let position_of_additional_code = additional_code_regex.end();
            return format!(
                "{}\n\n{}{}",
                &existing_code[..position_of_additional_code],
                additional_runtime_code,
                &existing_code[position_of_additional_code..],
            );
        } else {
            existing_code
        }
    }

    fn add_runtime_api_code(
        &mut self,
        existing_code: String,
        runtime_api: &Option<String>,
        test_regex: &Regex,
    ) -> String {
        if runtime_api.is_none() {
            return existing_code;
        }

        let mut add_runtime_api = String::new();
        add_runtime_api.push_str(&format!("{}\n", runtime_api.clone().unwrap()));

        if let Some(additional_code_regex) = test_regex.find(&existing_code) {
            let position_of_additional_code = additional_code_regex.end() - 2;
            return format!(
                "{}\n{}{}",
                &existing_code[..position_of_additional_code],
                add_runtime_api,
                &existing_code[position_of_additional_code..],
            );
        } else {
            existing_code
        }
    }
    fn replace_existing_genesis_field_value(
        &mut self,
        struct_field_name: &str,
        genesis_field_name: &str,
        new_value: &str,
    ) {
        let find_genesis_struct = Regex::new(&format!(
            r"{}:[\s\S]+?(?P<fieldLine>{}: \s(?P<fieldValue>[\S ]+),)",
            struct_field_name, genesis_field_name
        ))
        .unwrap();

        if let Some(genesis_struct_test) = find_genesis_struct.find(&self.chain_spec_code) {
            let struct_ = genesis_struct_test.as_str().to_string();
            let struct_field_line = find_genesis_struct
                .captures(&struct_)
                .unwrap()
                .name("fieldLine")
                .unwrap()
                .as_str();
            let struct_field_value = find_genesis_struct
                .captures(&struct_)
                .unwrap()
                .name("fieldValue")
                .unwrap()
                .as_str();

            let new_struct_field_line = struct_field_line.replace(struct_field_value, new_value);
            let new_struct = struct_.replace(struct_field_line, &new_struct_field_line);

            self.chain_spec_code = self.chain_spec_code.replace(&struct_, &new_struct);
        }
    }

    fn add_chain_spec_code(&mut self) {
        if let Some(genesis_config) = &self.pallet_config.runtime.genesis_config {
            if let Some(genesis_config_match) =
                self.regex.genesis_config.find(&self.chain_spec_code)
            {
                let mut genesis_config_str = format!("{}: ", genesis_config.config_struct_name);
                genesis_config_str.push_str("{\n");

                genesis_config_str.push_str(&format!(
                    "{} \n {}",
                    genesis_config
                        .struct_fields
                        .iter()
                        .map(|(k, v)| format!("{}{}: {}", tabs(7), k, v))
                        .collect::<String>(),
                    tabs(5)
                ));
                genesis_config_str.push_str("},\n");

                let genesis_config_match = genesis_config_match.as_str().to_string();

                let updated_genesis = format!(
                    "{}{}{}{}",
                    &genesis_config_match[..genesis_config_match.len() - 3],
                    tabs(5),
                    genesis_config_str,
                    &genesis_config_match[genesis_config_match.len() - 3..],
                );

                if self.pallet_name() == "PalletSession" {
                    self.replace_existing_genesis_field_value("grandpa", "authorities", "vec![]");

                    self.replace_existing_genesis_field_value("aura", "authorities", "vec![]");
                }

                let genesis_struct_block = self
                    .regex
                    .genesis_config
                    .find(&self.chain_spec_code)
                    .unwrap()
                    .as_str();

                self.chain_spec_code = self.chain_spec_code.replace(
                    genesis_struct_block,
                    &format!("{}{}", genesis_struct_block, updated_genesis),
                );
            }
        }
    }

    pub fn generate_runtime_code(&mut self) -> GeneratedRuntime {
        if !self.check_if_pallet_implemented() {
            // self.check_if_pallet_implemented();
            self.add_pallet_traits();
            self.add_pallet_to_construct_runtime();
            self.add_chain_spec_code();

            // Extract the additional code and chain spec code for modification
            let additional_runtime_lib_code = self
                .pallet_config
                .runtime
                .additional_runtime_lib_code
                .clone();
            let additional_chain_spec_code = self
                .pallet_config
                .runtime
                .additional_chain_spec_code
                .clone();
            let runtime_api_code = self.pallet_config.runtime.runtime_api_code.clone();

            // Apply additional code modifications
            self.runtime_code = self.add_additional_code(
                self.runtime_code.clone(),
                &additional_runtime_lib_code,
                &self.regex.additional_code.clone(),
            );

            // Apply additional genesis variables modifications
            self.chain_spec_code = self.add_additional_code(
                self.chain_spec_code.clone(),
                &additional_chain_spec_code,
                &self.regex.additional_code.clone(),
            );

            // Apply runtime apis modifications
            self.runtime_code = self.add_runtime_api_code(
                self.runtime_code.clone(),
                &runtime_api_code,
                &self.regex.runtime_api.clone(),
            );
        }

        GeneratedRuntime {
            updated_runtime_code: self.runtime_code.clone(),
            updated_chain_spec_code: self.chain_spec_code.clone(),
        }
    }
}

fn to_snake_case(s: &str) -> String {
    // Convert to snake case
    s.to_lowercase().replace(" ", "_")
}

#[allow(unused)]
fn to_pascal_case(s: &str) -> String {
    // Convert to pascal case
    s.split('_')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f
                    .to_uppercase()
                    .chain(c.flat_map(|l| l.to_lowercase()))
                    .collect(),
            }
        })
        .collect()
}

fn tabs(n: usize) -> String {
    "   ".repeat(n)
}
