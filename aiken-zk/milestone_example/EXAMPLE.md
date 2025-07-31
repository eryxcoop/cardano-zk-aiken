# Walkthrough the example

First, enter the sub-directory ```curve_compress``` and run ```npm install```. Then go back and run the following:

```cargo run -- build validators_with_offchain/example.ak validators/output.ak```

The command will generate a modified aiken file in ```validators/output.ak```. This is the final source code, over which
you can use the main aiken compiler

The previous generated source code (validators/output.ak) includes a test that missing a valid proof to success.

Then, running an ```aiken check``` should execute successfully.
