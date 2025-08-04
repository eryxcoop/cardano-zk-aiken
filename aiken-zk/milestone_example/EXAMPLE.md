# Walkthrough the example

## Structure

* curve compress
* deployment
* templates
* validators with offchain

The example itself is an Aiken project (see aiken.toml)

## Dependencies

First, enter the subdirectory ```curve_compress``` and run ```npm install```. These files are used by the aiken-zk tool.

Then go back and enter the subdirectory ```deployment``` and run another ```npm install```. This installs the
dependencies
needed for meshJS deployment.

## Steps

### Conversion to Aiken

First, run the following command:

```cargo run -- build validators_with_offchain/example.ak validators/output.ak```

The command will generate a modified aiken file in ```validators/output.ak```. This is the final source code, over which
you can use the main aiken compiler.

The previous generated source code (validators/output.ak) includes a test that missing a valid proof to success. So,
trying an ```aiken check``` at this point will fail.

### Generate proof

#### Aiken testing (Milestone 2)

Execute the following command to generate a proof ready to use in the mentioned Aiken test:

```cargo run -- prove aiken output.circom verification_key.zkey inputs.json proof.ak```

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

Asumimos que tenes credenciales nombradas como me.addr y me.sk, sino las podes generar con generate-credentials.ts

Although the example provides scripts for locking and unlocking the contract, the tool is useful only for the unlocking
step.

To lock the contract, first compile the Aiken code with:

```aiken build```

Then, enter the subdirectory ```deployment``` and run:

```npx tsx lock.ts```

This will output the transaction hash, save it for the next step.

Now, it's time to unlock. For this task we provide the ```unlock.ts``` file, but it lacks the proof on the redeemer yet.

To generate this proof, you can use the aiken-zk tool.

To accomplish this task, go back and run:

```cargo run -- prove meshjs output.circom verification_key.zkey inputs.json deployment/zk_redeemer.ts```

Go to ```deployment/unlock.ts``` and import the exported function ```mZKRedeemer``` from the generated library:

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

Finally, to unlock the contract run the following command (use the hash that you copied in the lock step):

```npx tsx unlock.ts lockTxHash```

Now you can program, deploy and spend a validator with offchain capabilities!

For more information, read the README.


