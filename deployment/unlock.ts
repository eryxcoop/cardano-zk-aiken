import { Contract } from "./utils";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  
  const contract = new Contract(compiledContractPath);
  // get utxo, collateral and address from wallet
  await contract.spend(validatorIndex, txHashFromDeposit);
}
 
main();
