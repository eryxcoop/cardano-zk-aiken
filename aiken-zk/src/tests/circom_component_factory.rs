pub fn addition_custom_circom_template_and_component() -> String {
    r#"
pragma circom 2.1.9;

template Addition() {
    signal input a;
    signal input b;
    signal input c;
    c === a + b;
}

component main {public [a, b]} = Addition();
"#
    .to_string()
}
