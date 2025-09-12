# Walkthrough the example

## Structure

The directories are:

* curve compress: use by `aiken-zk`
* deployment: scripts for locking and unlocking the example validator
* templates: use by `aiken-zk`
* validators with offchain: contains the example validator with offchain code

The example itself is an Aiken project (see aiken.toml).

## Dependencies

First, enter the subdirectory ```curve_compress``` and run ```npm install```. These files are used by the aiken-zk tool.

Then go back and enter the subdirectory ```deployment``` and run another ```npm install```. This installs the
dependencies needed for meshJS deployment.

## Running the fibonacci example (simple token)

### Conversion to compilable Aiken

First, run the following command:

```shell
cargo run -- build validators_with_offchain/example_fibonacci.ak validators/output.ak
```

The command will generate a modified aiken file in ```validators/output.ak```. This is the final source code, over which
you can use the main aiken compiler.

The generated source code includes a test that missing a valid proof to success. So,
trying an ```aiken check``` at this point will fail.

### Generate proof

The fibonacci parameters used to generate the proof are in the input_fibonacci.json file:

```json
{
  "a": "2",
  "b": "3",
  "c": "13"
}
```

This means `a = fibonacci(1) = 2`, `b = fibonacci(2) = 3`, then `c = fibonacci(5) = 13`.

#### Aiken testing (Milestone 2)

Execute the following command to generate a proof ready to use in the mentioned Aiken test:

```shell
cargo run -- prove aiken output.circom verification_key.zkey inputs_fibonacci.json proof.ak
```

The ```proof.ak``` file will look like:

```
Proof {
    piA: #"...",
    piB: #"...",
    piC: #"...",
}
```

Copy the file content and replace the placeholder proof on the generated ```output.ak```:

```
test test_example() {
    let proof: Proof = Proof {
        piA: #"...",
        piB: #"...",
        piC: #"...",
    }

    test_proof_is_valid(proof)
}
```

Then, running an ```aiken check``` should execute successfully.

#### MeshJS unlocking (Milestone 3)

##### Prerequisites

In order to deploy, you need a wallet with funds and a Blockfrost project.

The scripts expect that a file ```me.sk``` exists on the deployment folder. This file should be your wallet key.

If you don't have a wallet, you can use the ```deployment/generate-crendentials.ts``` script to generate one:

```shell
npx tsx generate-credentials.ts
```

Then use https://docs.cardano.org/cardano-testnet/tools/faucet to get some funds.

As the deployment script uses Blockfrost, you need to define an environment variable BLOCKFROST_ID with your blockfrost
project id:

```shell
export BLOCKFROST_PROJECT_ID=preview...
```

##### Steps

Although the example provides scripts for locking and unlocking the contract, the tool is useful only for the unlocking
step.

To lock the contract, first compile the Aiken code with:

```shell
aiken build
```

Then, enter the subdirectory ```deployment``` and run:

```shell
npx tsx lock_fibonacci.ts
```

This will output the transaction hash, save it for the next step and go back to the ```milestone_example``` directory.

Now, it's time to unlock. For this task we provide the ```unlock_fibonacci.ts``` file, but it lacks the proof on the redeemer yet.

To generate this proof, you can use the aiken-zk tool.

To accomplish this task, go back and run:

```shell
cargo run -- prove meshjs output.circom verification_key.zkey inputs_fibonacci.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_fibonacci.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
 ```

As the last step before unlocking, use the exported function to wrap the redeemer.
The spend should look like:

```javascript
await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(second_fibonacci))
```

This function will wrap your redeemer with additional information about the proof.

Now you have all you need to deploy the script into the blockchain.

Finally, to unlock the contract run the following command. Replace `lockTxHash` with the hash that you copied in the
lock step:

```shell
npx tsx unlock_fibonacci.ts lockTxHash
```

Now you can program, deploy and spend a validator with offchain capabilities!

For more information, read the README.

## Running the sha256 example (complex token)

For this tutorial we assume you have run the fibonacci example.

### Conversion to compilable Aiken

Run the following command:

```shell
cargo run -- build validators_with_offchain/example_sha256.ak validators/output.ak
```

The compilable aiken file is in ```validators/output.ak```.

### Generate proof

The sha256 parameters used to generate the proof are in the input_sha256.json file:

```json
{
  "in": ["0","0", ... ,"1"],
  "out": ["0xBFB424E48235A63C27A22610243DC4E0", "0xB217B0B604358A93072E1FBA35637435"]
}
```

This means `sha256("Coffe") = "0xBFB424E48235A63C27A22610243DC4E0B217B0B604358A93072E1FBA35637435"`.

#### Aiken testing

Execute the following command to generate the proof to use in the Aiken test:

```shell
cargo run -- prove aiken output.circom verification_key.zkey inputs_sha256.json proof.ak
```

You could use the generated proof ```proof.ak``` on the Aiken test. Then, running an ```aiken check``` should execute
successfully.

#### MeshJS unlocking

Compile the Aiken code with:

```shell
aiken build
```

Then, enter the subdirectory ```deployment``` and run:

```shell
npx tsx lock_complex_token.ts
```

Save the output transaction hash for the next step and go back to the ```milestone_example``` directory.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs output.circom verification_key.zkey inputs_sha256.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_complex_token.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
```

Use the exported function to wrap the redeemer. The spend should look like:

```javascript
await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(mVoid()))
```

Finally, unlock the contract running the following command. Replace `lockTxHash` with the hash that you copied in the
lock step:

```shell
npx tsx unlock_complex_token.ts lockTxHash
```

## Running the MerkleTreeChecker example (complex token)

For this tutorial we assume you have run the fibonacci example.

### Conversion to compilable Aiken

Run the following command:

```shell
cargo run -- build validators_with_offchain/example_merkle_tree_checker.ak validators/output.ak
```

The compilable aiken file is in ```validators/output.ak```.

### Generate proof

The MerkleTreeChecker parameters used to generate the proof are in the input_merkle_tree_checker.json file:

```json
{
  "leaf": 0,
  "root": 79,
  "pathElements": [1, 10],
  "pathIndices": [0, 1]
}
```

#### Aiken testing

Execute the following command to generate the proof to use in the Aiken test:

```shell
cargo run -- prove aiken output.circom verification_key.zkey inputs_merkle_tree_checker.json proof.ak
```

You could use the generated proof ```proof.ak``` on the Aiken test. Then, running an ```aiken check``` should execute
successfully.

#### MeshJS unlocking

Compile the Aiken code with:

```shell
aiken build
```

Then, enter the subdirectory ```deployment``` and run:

```shell
npx tsx lock_complex_token.ts
```

Save the output transaction hash for the next step and go back to the ```milestone_example``` directory.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs output.circom verification_key.zkey inputs_merkle_tree_checker.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_complex_token.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
```

Use the exported function to wrap the redeemer. The spend should look like:

```javascript
await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(mVoid()))
```

Finally, unlock the contract running the following command. Replace `lockTxHash` with the hash that you copied in the
lock step:

```shell
npx tsx unlock_complex_token.ts lockTxHash
```

## Running the CustomCircuit example (complex token)

We assume you have run the fibonacci example.

In order to show how a custom circuit works, a my_custom_circuit.circom is provided. Note that it is a circom component (not a
circom template), this means it includes a line ```component main {public [f2, hashedFn]} = MyCustomCircuit(5);```
declaring the main component with their public inputs.

### What this custom circuit does?

Given a generalization of fibonacci sequence being the elements fibonacci_1 and fibonacci_2 arbitrary integers instead of
0 and 1 like the original fibonacci sequence. Then the fibonacci_n is the nth element being n the sequence length.
This example uses an $n = 5$.

For example a fibonacci like sequence with fibonacci_1 = 10 and fibonacci_2 = 11 and a length of 5, has this look: 
[10,11,21,32,53]

Then, the circuit checks that a user knows a fibonacci sequence. It has 4 inputs: fibonacci_1, fibonacci_2, fibonacci_n
and a hash of fibonacci_n. The circuit checks that the hash matches with fibonacci_n.

As fibonacci_2 and the hashed element are private, the idea is that the locker published the fibonacci_2 and the hashed 
element. So the unlocker has to provide fibonacci_1 and fibonacci_n that are private in order to unlock the validator.

### Conversion to compilable Aiken

Run the following command:

```shell
cargo run -- build validators_with_offchain/example_custom_circuit.ak validators/output.ak
```

The compilable aiken file is in ```validators/output.ak```.

### Generate proof

The CustomCircuit parameters used to generate the proof are in the input_custom_circuit.json file:

```json
{
  "f1": "10",
  "f2": "11",
  "fn": "53",
  "hashedFn": "12890501568230843208428202504271607409819721466300370964598773433007502645712"
}
```

#### Aiken testing

Execute the following command to generate the proof to use in the Aiken test:

```shell
cargo run -- prove aiken my_custom_circuit.circom verification_key.zkey inputs_custom_circuit.json proof.ak
```

You could use the generated proof ```proof.ak``` on the Aiken test. Then, running an ```aiken check``` should execute
successfully.

#### MeshJS unlocking

Compile the Aiken code with:

```shell
aiken build
```

Then, enter the subdirectory ```deployment``` and run:

```shell
npx tsx lock_complex_token.ts
```

Save the output transaction hash for the next step and go back to the ```milestone_example``` directory.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs my_custom_circuit.circom verification_key.zkey inputs_custom_circuit.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_complex_token.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

```javascript
 import {mZKRedeemer} from "./zk_redeemer";
```

Use the exported function to wrap the redeemer. The spend should look like:

```javascript
await contract.spend(validatorScriptIndex, txHashFromDeposit, mZKRedeemer(mVoid()))
```

Finally, unlock the contract running the following command. Replace `lockTxHash` with the hash that you copied in the
lock step:

```shell
npx tsx unlock_complex_token.ts lockTxHash
```

