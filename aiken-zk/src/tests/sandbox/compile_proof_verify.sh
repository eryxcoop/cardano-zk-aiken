#!/bin/bash

CIRCUIT_PATH="$2"

CIRCUIT_NAME=$(basename -s ".circom" "${CIRCUIT_PATH}")

INPUT_PATH="$3"

OUTPUT_PATH="build/"
mkdir -p ${OUTPUT_PATH}

CEREMONY_PATH="$4"

RAND_1="$5"
RAND_2="$6"

function compile() {

    # Compiles circuit to wasm
    circom "${CIRCUIT_PATH}" --r1cs --wasm --sym -p bls12381 -o ${OUTPUT_PATH}

    # Groth16 setup
    snarkjs groth16 setup ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs ${CEREMONY_PATH} ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey

    echo $RAND_1 | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey --name="1st Contributor Name" -v

    echo $RAND_2 | snarkjs zkey contribute ${OUTPUT_PATH}${CIRCUIT_NAME}_0001.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_0002.zkey --name="Second contribution Name" -v

    snarkjs zkey beacon ${OUTPUT_PATH}${CIRCUIT_NAME}_0000.zkey ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"

    snarkjs zkey export verificationkey ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey ${OUTPUT_PATH}verification_key.json

}

function prove() {
  node ${OUTPUT_PATH}${CIRCUIT_NAME}_js/generate_witness.js ${OUTPUT_PATH}${CIRCUIT_NAME}_js/${CIRCUIT_NAME}.wasm ${INPUT_PATH}  ${OUTPUT_PATH}witness.wtns
  snarkjs groth16 prove ${OUTPUT_PATH}${CIRCUIT_NAME}_final.zkey ${OUTPUT_PATH}witness.wtns ${OUTPUT_PATH}proof.json ${OUTPUT_PATH}public.json
}

function verify() {
  snarkjs groth16 verify ${OUTPUT_PATH}verification_key.json ${OUTPUT_PATH}public.json ${OUTPUT_PATH}proof.json
}

case "$1" in
    -c )
      compile
      ;;
    -p )
      prove
      ;;
    -v )
      verify
      ;;
    -cpv )
      compile
      prove
      verify
      ;;
esac

# Serialize circuit information
# snarkjs r1cs export json ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs ${OUTPUT_PATH}${CIRCUIT_NAME}.r1cs.json

