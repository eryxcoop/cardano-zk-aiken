import { mConStr0 } from "@meshsdk/core";
import { BlockchainProvider } from "./blockchain_provider";
import { Contract, mVoid } from "./contract";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');
  
  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
  const proof = mConStr0([
    "a6ff73ac3b7e373996ad4e45f4392c70de3da99db07933b44f504d230d2cf36b7f6f6e50761cda8d1297b36c5b216ed4",
    "91ef5fea8f068dbeb1893af96fd52e2d9b7e25cad9a0639c040e45e0a273a54856c0b37d00fc4601340e7028502da6de07079fe9d07c06122124750b63626e5150716a3bd601ea80420860151e5894c3a8c581636dedda8e0071fddd5aa93b9c",
    "a241a69c5c35aa559c9e79e5ee537f56c7fe7d083f7796cce1f8b6d119a1f893739af4a93d7d5743d31201c709cf9b4f"
  ]);
  const proofs = [proof];
  const redeemer = mConStr0([mVoid(), proofs]);
  contract.spend(validatorScriptIndex, txHashFromDeposit, redeemer, {mem: 154984, steps: 4284159244})
}

main();
