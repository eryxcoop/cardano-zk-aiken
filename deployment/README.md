## Generate credentials
```npm install```
```npx tsx generate-credentials.ts``` --> Crea los archivos 
* ```me.addr``` --> Dirección generada
* ```me.sk``` --> Private key de la dirección

Run ```export BLOCKFROST_PROJECT_ID=preview...```

To obtain such a key you must create an account on [blockfrost](https://blockfrost.io) and create a project. 

## Deposit funds from a faucet 
Go to https://docs.cardano.org/cardano-testnets/tools/faucet, copy the address from ```me.addr``` and receive test ADA.
You'll have to wait until the transaction takes place. You can check that on [CardanoScan](https://preview.cardanoscan.io/).

## Lock
The logic for deploying a locking contract is in lock.ts. You must provide the script with:
* a compiled contract path (path to some ```plutus.json```)
* an index inside the ```plutus.json``` that refers to the desired script, you'll have to check the plutus file.

Run ```npx tsx lock.ts path_to_plutus script_index```.
Wait for the transaction to take place on [CardanoScan](https://preview.cardanoscan.io/).

#### Where to get the plutus.json from?
Compile an aiken file. That should generate a ```plutus.json```. 

## Unlock

