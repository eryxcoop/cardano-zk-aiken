use crate::compiler::token_zk::TokenZK as Token;
use crate::zk_examples::{InputVisibility, InputZK, ZkExample};

pub struct ComponentCreator {
    pub token: Token,
}

impl ComponentCreator {
    pub fn from_token(token: Token) -> Self {
        Self { token }
    }
    const USED_CIRCOM_VERSION: &'static str = "2.1.9";

    fn process_component_inputs_and_template_parameters<const N: usize>(template_file_name: &str, template_name: &str, input_to_identifiers: [(&InputZK, &str); N], template_parameters: &[&str]) -> String {
        let public_inputs_identifiers = Self::process_inputs_visibility(input_to_identifiers);

        Self::generate_circom_component(
            Self::USED_CIRCOM_VERSION,
            template_file_name,
            template_name,
            public_inputs_identifiers,
            template_parameters,
        )
    }

    fn generate_circom_component(
        circom_version: &str,
        template_file_name: &str,
        template_name: &str,
        public_inputs_identifiers: Vec<String>,
        circuit_template_parameters: &[&str],
    ) -> String {
        let circom_version_line = format!("pragma circom {};", circom_version);
        let import_line = format!("include \"templates/{}.circom\";", template_file_name);

        let visibility_line = Self::generate_inputs_visibility(public_inputs_identifiers);

        let component_parameters = Self::process_and_generate_template_arguments(circuit_template_parameters);

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
            ZkExample::Addition { lhs, rhs, res } => {
                let template_file_name = "addition";
                let template_name = "Addition";

                let input_to_identifiers = [(lhs, "first_addend"), (rhs, "second_addend"), (res, "sum")];
                let template_parameters = &[];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, input_to_identifiers, template_parameters)
            }
            ZkExample::Subtraction { lhs, rhs, res } => {
                let template_file_name= "subtraction";
                let template_name= "Subtraction";
                
                let input_to_identifiers = [(lhs, "minuend"), (rhs, "subtrahend"), (res, "difference")];
                let template_parameters = &[];
                
                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, input_to_identifiers, template_parameters)
            }
            ZkExample::Multiplication { lhs, rhs, res } => {
                let template_file_name = "multiplication";
                let template_name = "Multiplication";

                let input_to_identifiers = [(lhs, "multiplicand"), (rhs, "multiplier"), (res, "product")];
                let template_parameters = &[];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, input_to_identifiers, template_parameters)
            }
            ZkExample::Fibonacci {
                fib_0,
                fib_1,
                n,
                res,
            } => {
                let Token::Int { value, base: _ } = *n.token.clone() else {
                    panic!("Not expected kind of token")
                };

                let template_file_name = "fibonacci";
                let template_name = "Fibonacci";

                let inputs_to_identifiers = [(fib_0, "first_fibonacci"), (fib_1, "second_fibonacci"), (res, "nth_fibonacci")];
                let template_parameters = &[value.as_str()];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
            ZkExample::If {
                condition,
                assigned,
                true_branch,
                false_branch,
            } => {
                let template_file_name = "if";
                let template_name = "If";

                let inputs_to_identifiers = [
                    (condition, "condition"),
                    (assigned, "assigned"),
                    (true_branch, "true_branch"),
                    (false_branch, "false_branch"),
                ];
                let template_parameters = &[];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
            ZkExample::AssertEq { lhs, rhs } => {
                let template_file_name = "assert_eq";
                let template_name = "AssertEq";

                let inputs_to_identifiers = [(lhs, "lhs"), (rhs, "rhs")];
                let template_parameters = &[];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
            ZkExample::CustomCircom { .. } => {
                panic!("You shouldn't be here")
            }
            ZkExample::Sha256 { n_bits, r#in, out } => {
                let Token::Int { value, base: _ } = *n_bits.token.clone() else {
                    panic!("Not expected kind of token")
                };
                let template_file_name = "hash";
                let template_name = "Sha256";

                let inputs_to_identifiers = [(r#in, "in"), (out, "out")];
                let template_parameters = &[value.as_str()];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
            ZkExample::Poseidon {
                n_inputs,
                r#in,
                out,
            } => {
                let Token::Int { value, base: _ } = *n_inputs.token.clone() else {
                    panic!("Not expected kind of token")
                };
                let template_file_name = "hash";
                let template_name = "Poseidon";

                let inputs_to_identifiers = [(r#in, "in"), (out, "out")];
                let template_parametrs = &[value.as_str()];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parametrs)
            }
            ZkExample::MerkleTreeChecker {
                levels,
                leaf,
                root,
                path_elements,
                path_indices,
            } => {
                let Token::Int { value, base: _ } = *levels.token.clone() else {
                    panic!("Not expected kind of token")
                };
                let template_file_name = "merkle_tree_checker";
                let template_name = "MerkleTreeChecker";

                let inputs_to_identifiers = [
                    (leaf, "leaf"),
                    (root, "root"),
                    (path_elements, "pathElements"),
                    (path_indices, "pathIndices"),
                ];
                let template_parameters = &[value.as_str()];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
            ZkExample::PolynomialEvaluations {
                grade,
                coefficients,
                amount_of_evaluations,
                domain,
                evaluations,
            } => {
                let Token::Int { value: polynomial_grade, base: _ } = *grade.token.clone() else {
                    panic!("Not expected kind of token")
                };
                let Token::Int { value: amount_of_evaluations, base: _ } = *amount_of_evaluations.token.clone() else {
                    panic!("Not expected kind of token")
                };
                let template_file_name = "polynomials";
                let template_name = "PolynomialEvaluations";

                let inputs_to_identifiers = [
                    (coefficients, "coefficients"),
                    (domain, "domain"),
                    (evaluations, "evaluations"),
                ];
                let template_parameters = &[polynomial_grade.as_str(), amount_of_evaluations.as_str()];

                Self::process_component_inputs_and_template_parameters(template_file_name, template_name, inputs_to_identifiers, template_parameters)
            }
        }
    }

    fn process_inputs_visibility<const N: usize>(
        public_inputs_identifiers: [(&InputZK, &str); N],
    ) -> Vec<String> {
        public_inputs_identifiers
            .iter()
            .fold(vec![], |mut acc, (input, var_name)| {
                match input.visibility {
                    InputVisibility::Private => acc,
                    InputVisibility::Public => {
                        acc.push(var_name.to_string());
                        acc
                    }
                }
            })
    }
}
