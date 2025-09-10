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

This will output the transaction hash, save it for the next step.

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
npx tsx lock_sha256.ts
```

Save the output transaction hash for the next step.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs output.circom verification_key.zkey inputs_sha256.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_sha256.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

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
npx tsx unlock_sha256.ts lockTxHash
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
npx tsx lock_merkle_tree_checker.ts
```

Save the output transaction hash for the next step.

Time to unlock. Run the following command to generate the typescript library that contains the proof:

```shell
cargo run -- prove meshjs output.circom verification_key.zkey inputs_merkle_tree_checker.json deployment/zk_redeemer.ts
```

Go to ```deployment/unlock_merkle_tree_checker.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

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
npx tsx unlock_merkle_tree_checker.ts lockTxHash
    ```
