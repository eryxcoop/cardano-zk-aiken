import { mConStr0, mStringToPlutusBSArray } from "@meshsdk/core";
import { BlockchainProvider } from "./blockchain_provider";
import { Contract } from "./contract";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');
  
  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
  const factors = mConStr0([5, 7]);
  const proof = mConStr0([
    "a66840faf8af87a974d1bcf96820f3893870463bfce0140f265bbe2fdf699d7c3e8822c1400d7b1a30390ef5c1e021af",
    "897d7b5074b813ff043efa7b5f62a028ead87b8576e8700cbb138ef9d209b4a3365ccd1ffd5eb30f8bbaeac75977a1ca06a9e54ef326e5b60000065406dfb24f64db99e7e38672f21e8a1c67d4ca61db45e9e5887d5db6a3ed7cb40c96256b5e",
    "ad6a3410175b686c3982e6cafdcc68414fcda8a769f9f5c06c0b5130146a6c6885ee2a8bffce7e0ce3a5c9cf813ad534"
  ]);
  const proofs = [proof];
  const redeemer = mConStr0([factors, proofs]);
  contract.spend(validatorScriptIndex, txHashFromDeposit, redeemer, {mem: 154984, steps: 4284159244})
}

main();
