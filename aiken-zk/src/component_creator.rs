use crate::token_zk::TokenZK as Token;
use crate::zk_examples::{InputVisibility, InputZK, ZkExample};

pub struct ComponentCreator {
    pub token: Token,
}

impl ComponentCreator {
    pub fn from_token(token: Token) -> Self {
        Self { token }
    }

    fn process_three_inputs_for_template(
        &self,
        lhs: &InputZK,
        rhs: &InputZK,
        res: &InputZK,
        template_file: &str,
        template: &str,
    ) -> String {
        let public_inputs_identifiers = [(lhs, "a"), (rhs, "b"), (res, "c")].iter().fold(
            vec![],
            |mut acc, (input, var_name)| match input.visibility.clone() {
                Some(visibility) => match visibility {
                    InputVisibility::Public => {
                        acc.push(*var_name);
                        acc
                    }
                    _ => acc,
                },
                None => {
                    acc.push(*var_name);
                    acc
                }
            },
        );

        let import = format!("include \"templates/{}.circom\";", template_file);
        let visibility = if public_inputs_identifiers.len() == 0 {
            "".to_string()
        } else {
            format!("{{ public [{}] }} ", public_inputs_identifiers.join(","))
        };
        let instantiation =
            "component main ".to_string() + &visibility + &format!("= {}();", template);
        import.to_string() + "\n" + &instantiation
    }

    pub fn create(&self) -> String {
        let Token::Offchain { example } = &self.token else {
            panic!("Not expected kind of token")
        };
        match example {
            ZkExample::Addition { lhs, rhs, res } => {
                self.process_three_inputs_for_template(lhs, rhs, res, "addition", "Addition")
            }
            ZkExample::Subtraction { lhs, rhs, res } => {
                self.process_three_inputs_for_template(lhs, rhs, res, "subtraction", "Subtraction")
            }
            ZkExample::Multiplication { lhs, rhs, res } => self.process_three_inputs_for_template(
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

                let public_inputs_identifiers = Self::process_inputs_visibility(inputs_to_identifiers);

                let import = "include \"templates/fibonacci.circom\";";
                let visibility = if public_inputs_identifiers.len() == 0 {
                    "".to_string()
                } else {
                    format!("{{ public [{}] }} ", public_inputs_identifiers.join(","))
                };
                let instantiation = "component main ".to_string()
                    + &visibility
                    + &format!("= Fibonacci({});", value);
                import.to_string() + "\n" + &instantiation
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
                let public_inputs_identifiers = Self::process_inputs_visibility(inputs_to_identifiers);

                let import = "include \"templates/if.circom\";";
                let visibility = if public_inputs_identifiers.len() == 0 {
                    "".to_string()
                } else {
                    format!("{{ public [{}] }} ", public_inputs_identifiers.join(","))
                };
                let instantiation = "component main ".to_string() + &visibility + "= If();";
                import.to_string() + "\n" + &instantiation
            },
            ZkExample::AssertEq {
                lhs,rhs
            } => {String::from("")}
        }
    }

    fn process_inputs_visibility<const N:usize>(public_inputs_identifiers: [(&InputZK, &str); N]) -> Vec<String> {
        public_inputs_identifiers.iter()
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
