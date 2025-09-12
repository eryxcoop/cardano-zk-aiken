# Custom example using list inputs
We'll walk through an use case of this tool using a custom circom circuit that takes a list as an input. 

## The source code
The first step is creating the custom circom file. This can be found in the ```compare_head.circom``` file. As you can see, there are 2 input signals, one of which is an array. Remember that your custom circom file must contain a ```main``` component for the tool to work. 

The next step is to reference the circom file within our aiken code. The file we're using for this is the ```compare_head.ak```, and the line is

```rust
expect _redeemer = offchain custom("./compare_head.circom", [@l, val])
```

Notice that we're using an extra ```@``` to indicate that the variable ```l``` is, in fact, a variable containing a list. We're not using this in the variable ```val``` since it's a single number. In general, you should make sure that the amount of public inputs in the aiken file matches with the amount in the main circom component.  

## Building the source code with the Tool

The next step is to build the aiken code in ```compare_head.ak```. Here, run the command

```bash
cargo run -- build compare_head.ak validators/compare_head_final.ak
```

This will create a new aiken file in ```validators/compare_head_final.ak``` which has an embedded Groth16 verification key for the circom code we defined in ```compare_head.circom```. You can check it out if you want. 

## Building with Aiken
Now we're going to use the original aiken tool to build the automatically generated code. For this, you just need to run 
```bash
aiken build
```
without changing your directory (meaning, in the same directory that the ```aiken.toml``` file is located, which should be the same one that contains the ```validators``` directory). If the build is successfull, we can move to the next steps. 

## Locking funds using the 