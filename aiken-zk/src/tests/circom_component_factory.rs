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

pub fn indexing_custom_circom_template_and_component() -> String {
    r#"
pragma circom 2.1.9;

template Addition() {
    signal input l[2];
    signal input idx;
    signal input val;
    l[idx] === val;
}

component main {public [l, idx, val]} = Addition();
"#
    .to_string()
}


