import { BlockchainProvider } from "./blockchain_provider";
import { Contract } from "./contract";
import {mConStr0} from "@meshsdk/core";
import {datum} from "./datum";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');
  console.log(datum);

  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
  await contract.deploy(validatorScriptIndex, mConStr0([35]));
}

main();