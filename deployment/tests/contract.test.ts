import { blockchainProvider, Contract, getUtxoByTxHash } from "../utils";

describe('Contract Deployment', () => {
  test('Happy path', async () => {
    let deployed_successfully = false;
    let validatorIndex = 0;
    const contract = new Contract("./tests/groth16_examples.json");

    const deployTxHash = await contract.deployWithoutDatum(validatorIndex);
    await waitUntilDeployed(deployTxHash);
    try {
      await contract.spend(validatorIndex, deployTxHash);
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
