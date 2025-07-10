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
To unlock the funds, you must run

```npx tsx unlock_private.ts path_to_plutus script_index lock_transaction_hash```

# Flujo del milestone 2
aiken-zk build programita.ak asdasd -> programita_zk.ak
aiken-zk prove circom_path verification_key_path inputs_path output_path

# Milestone 3
aiken build programita_zk.ak --> Genera el plutus.json
```npx tsx lock.ts path_to_plutus script_index path_to_datum```

* Crear un ```ZkDatum``` como el ```ZkRedeemer```. 
  * Parametrizar los tests con el datum dependiendo de la cantidad de variables (no constantes) públicas. 

(A definir si es algo que queremos) 
Crear un comando más para aiken-zk. Este comando ```deploy``` debería usar Mesh para deployar el validator. También tiene que encontrar dentro del plutus.json el índice del script correspondiente. Eso lo puede hacer por nombre_de_archivo.ak + "spend", de esta forma sacándole la responsabilidad al usuario. Por otro lado, necesitamos poder indicarle qué valores meter en el datum. En este momento, el ```lock.ts``` tiene hardcodeado el datum para que sea un solo número, pero en la realidad va a ser algo más complejo (como un diccionario).

