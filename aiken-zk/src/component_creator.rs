use crate::token_zk::TokenZK as Token;
use crate::zk_examples::{InputVisibility, InputZK, ZkExample};

pub struct ComponentCreator {
    pub token: Token,
}

impl ComponentCreator {
    pub fn from_token(token: Token) -> Self {
        Self { token }
    }
    const USED_CIRCOM_VERSION: &'static str = "2.1.9";

    fn process_components_with_3_inputs_and_no_template_parameters(
        &self,
        lhs: &InputZK,
        rhs: &InputZK,
        res: &InputZK,
        template_file: &str,
        template: &str,
    ) -> String {
        let input_to_identifiers = [(lhs, "a"), (rhs, "b"), (res, "c")];
        let public_inputs_identifiers = Self::process_inputs_visibility(input_to_identifiers);

        Self::generate_circom_component(
            Self::USED_CIRCOM_VERSION,
            template_file,
            template,
            public_inputs_identifiers,
            &[],
        )
    }

    fn generate_circom_component(
        circom_version: &str,
        template_file_name: &str,
        template_name: &str,
        public_inputs_identifiers: Vec<String>,
        parameters: &[&str],
    ) -> String {
        let circom_version_line = format!("pragma circom {};", circom_version);
        let import_line = format!("include \"templates/{}.circom\";", template_file_name);

        let visibility_line = Self::generate_inputs_visibility(public_inputs_identifiers);

        let component_parameters = Self::process_and_generate_template_arguments(parameters);

        let instantiation = format!(
            "component main {}= {}{};",
            visibility_line, template_name, component_parameters
        );

        format!(
            "{}\n{}\n{}",
            circom_version_line, import_line, instantiation
        )
    }

    fn process_and_generate_template_arguments(parameters: &[&str]) -> String {
        if parameters.is_empty() {
            "()".to_string()
        } else {
            format!("({})", parameters.join(", "))
        }
    }

    fn generate_inputs_visibility(public_inputs_identifiers: Vec<String>) -> String {
        if public_inputs_identifiers.is_empty() {
            "".to_string()
        } else {
            format!("{{ public [{}] }} ", public_inputs_identifiers.join(","))
        }
    }

    pub fn create(&self) -> String {
        let Token::Offchain { example } = &self.token else {
            panic!("Not expected kind of token")
        };
        match example {
            ZkExample::Addition { lhs, rhs, res } => self
                .process_components_with_3_inputs_and_no_template_parameters(
                    lhs, rhs, res, "addition", "Addition",
                ),
            ZkExample::Subtraction { lhs, rhs, res } => self
                .process_components_with_3_inputs_and_no_template_parameters(
                    lhs,
                    rhs,
                    res,
                    "subtraction",
                    "Subtraction",
                ),
            ZkExample::Multiplication { lhs, rhs, res } => self
                .process_components_with_3_inputs_and_no_template_parameters(
                    lhs,
                    rhs,
                    res,
                    "multiplication",
                    "Multiplication",
                ),
            ZkExample::Fibonacci {
                fib_0,
                fib_1,
                n,
                res,
            } => {
                let Token::Int { value, base: _ } = &*n.token else {
                    panic!("Not expected kind of token")
                };
                let inputs_to_identifiers = [(fib_0, "a"), (fib_1, "b"), (res, "c")];
                let public_inputs_identifiers =
                    Self::process_inputs_visibility(inputs_to_identifiers);
                Self::generate_circom_component(
                    Self::USED_CIRCOM_VERSION,
                    "fibonacci",
                    "Fibonacci",
                    public_inputs_identifiers,
                    &[&value],
                )
            }
            ZkExample::If {
                condition,
                assigned,
                true_branch,
                false_branch,
            } => {
                let inputs_to_identifiers = [
                    (condition, "condition"),
                    (assigned, "assigned"),
                    (true_branch, "true_branch"),
                    (false_branch, "false_branch"),
                ];
                let public_inputs_identifiers =
                    Self::process_inputs_visibility(inputs_to_identifiers);

                Self::generate_circom_component(
                    Self::USED_CIRCOM_VERSION,
                    "if",
                    "If",
                    public_inputs_identifiers,
                    &[],
                )
            }
            ZkExample::AssertEq { lhs, rhs } => {
                let inputs_to_identifiers = [(lhs, "a"), (rhs, "b")];
                let public_inputs_identifiers =
                    Self::process_inputs_visibility(inputs_to_identifiers);
                Self::generate_circom_component(
                    Self::USED_CIRCOM_VERSION,
                    "assert_eq",
                    "AssertEq",
                    public_inputs_identifiers,
                    &[],
                )
            }
            ZkExample::CustomCircom { path, public_input_identifiers} => {
                panic!("Not implemented")
            }
        }
    }

    fn process_inputs_visibility<const N: usize>(
        public_inputs_identifiers: [(&InputZK, &str); N],
    ) -> Vec<String> {
        public_inputs_identifiers
            .iter()
            .fold(vec![], |mut acc, (input, var_name)| {
                match input.visibility.clone() {
                    Some(InputVisibility::Private) => acc,
                    _ => {
                        acc.push(var_name.to_string());
                        acc
                    }
                }
            })
    }
}
