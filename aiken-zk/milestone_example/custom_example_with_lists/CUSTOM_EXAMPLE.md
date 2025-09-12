# Custom example using list inputs
We'll walk through a use case of this tool using a custom circom circuit that takes a list as an input. 

The following explanation assumes that you have installed all the dependencies as dictated in the root ```README.md``` file. 

## The source code
The first step is creating the custom circom file. This can be found in the ```compare_head.circom``` file. As you can see, there are 2 input signals, one of which is an array. Remember that your custom circom file must contain a ```main``` component for the tool to work. 

The next step is to reference the circom file within our aiken code. The file we're using for this is the ```compare_head.ak```, and the line is

```rust
expect _redeemer = offchain custom("./compare_head.circom", [@l, val])
```

Notice that we're using an extra ```@``` to indicate that the variable ```l``` is, in fact, a variable containing a list. We're not using this in the variable ```val``` since it's a single number. In general, you should make sure that the amount of public inputs in the aiken file matches with the amount in the main circom component.  

## Building the source code with the Tool

The next step is to build the aiken code in ```compare_head.ak```. Right now, the tool depends on external typescript code to compress the Groth16 verification key in a format usefull for Aiken. This code is located in the ```curve_compress/``` directory. You should go to that directory, run ```npm i``` and then come back out. 

Then, run the command

```bash
cargo run -- build compare_head.ak validators/compare_head_final.ak
```

This will create a new aiken file in ```validators/compare_head_final.ak``` which has an embedded Groth16 verification key for the circom code we defined in ```compare_head.circom```. You can check it out if you want. 

## Building with Aiken
Now we're going to use the original Aiken tool to build the automatically generated code. For this, you just need to run 
```bash
aiken build
```
without changing your directory (meaning, in the same directory that the ```aiken.toml``` file is located, which should be the same one that contains the ```validators``` directory). If the build is successfull, we can move to the next steps.

## Generate the proof
The CompareHead parameters used to generate the proof are in the ```input_compare_head.json``` file:

```json
{
  "l": [1, 2],
  "val": 1
}
```

### Aiken testing

Execute the following command to generate the proof to use in the Aiken test:

```bash
cargo run -- prove aiken compare_head.circom verification_key.zkey inputs_compare_head.json proof.ak
```
You could use the generated proof ```proof.ak``` on the Aiken test. Then, running an ```aiken check``` should execute successfully.

### MeshJS unlocking

Now that you've your aiken code built, you can enter to the subdirectory  ```deployment``` and run:

```shell
npx tsx lock_complex_token.ts
```

Save the output transaction hash for the next step and go back to the ```custom_example_with_list``` directory.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs compare_head.circom verification_key.zkey inputs_compare_head.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_complex_token.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
```

Use the exported function to wrap the redeemer. The spend should look like:

```javascript
await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(mVoid()))
```

Finally, unlock the contract running the following command. Replace `lockTxHash` with the hash that you copied in the lock step:

```shell
npx tsx unlock_complex_token.ts lockTxHash
```
