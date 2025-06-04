import { mNone, mOption, mSome } from "@meshsdk/core";
import { blockchainProvider, Contract, getUtxoByTxHash, mVoid } from "../contract";

describe('Contract Deployment', () => {
  test('Happy path with datum and redeemer', async () => {
    let deployed_successfully = false;
    let validatorIndex = 0;
    const contract = new Contract("./tests/plutus.json");

    const deployTxHash = await contract.deploy(validatorIndex, 42);
    await waitUntilDeployed(deployTxHash);
    try {
      await contract.spend(validatorIndex, deployTxHash, 42);
      deployed_successfully = true;
    } catch {}

    expect(deployed_successfully).toBe(true);
  }, 100000);

  test.skip('Happy path without datum and redeemer', async () => {
    let deployed_successfully = false;
    let validatorIndex = 2;
    const contract = new Contract("./tests/plutus.json");

    const deployTxHash = await contract.deploy(validatorIndex, mVoid());
    await waitUntilDeployed(deployTxHash);
    try {
      await contract.spend(validatorIndex, deployTxHash, mVoid());
      deployed_successfully = true;
    } catch {}

    expect(deployed_successfully).toBe(true);
  }, 100000);
});

async function waitUntilDeployed(deployTxHash: string) {
  let is_deployed = false;
  while (!is_deployed) {
    try {
      await blockchainProvider.fetchUTxOs(deployTxHash);
      is_deployed = true;
    } catch {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  };
}
