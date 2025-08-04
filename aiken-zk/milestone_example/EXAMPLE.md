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

...


