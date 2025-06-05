import { Contract, mVoid } from "./contract";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  
  const contract = new Contract(compiledContractPath);
  contract.getWalletCollateral();
}
 
main();
