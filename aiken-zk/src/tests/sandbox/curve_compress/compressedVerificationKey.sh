#!/bin/bash

VERIFICATION_KEY_PATH="$1"
VERIFICATION_KEY_NAME=$(basename -s ".zkey" "${VERIFICATION_KEY_PATH}")

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
OUTPUT_PATH="build/"

snarkjs zkey export verificationkey "${VERIFICATION_KEY_PATH}" "${OUTPUT_PATH}${VERIFICATION_KEY_NAME}.json" &> /dev/null
node "${SCRIPT_DIR}/compressedVerificationKey.js" "${OUTPUT_PATH}${VERIFICATION_KEY_NAME}.json"
