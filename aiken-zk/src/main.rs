use aiken_zk::create_zk_src;

fn main() {
    let src = "offchain addition(3,4,5)";
    println!("Original code: \n{}\n\n", src);
    let src_zk = create_zk_src(&src);
    println!("New code:\n{}\n\n", src_zk);

    // let res = argument_parser().parse(String::from("(4,4,4)"));
    // println!("{:?}", res);
}
