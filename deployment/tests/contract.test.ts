import { Data } from "@meshsdk/core";
import { Contract, mVoid } from "../contract";
import { BlockchainProvider } from "../blockchain_provider";

describe('Contract Deployment', () => {
  let scriptPath: string;
  beforeEach(() => {
    scriptPath = "./tests/plutus.json";
  })

  test('Happy path with datum and redeemer', async () => {
    const datum = 42;
    const redeemer = 42;
    await testDeploymentWith(scriptPath, 0, datum, redeemer);
  }, 100000);

  test.skip('Happy path without datum and redeemer', async () => {
    const datum = mVoid();
    const redeemer = mVoid();
    await testDeploymentWith(scriptPath, 2, datum, redeemer);
  }, 100000);
});

async function testDeploymentWith(scriptPath: string, validatorIndex: number, datum: Data, redeemer: Data) {
  let deployed_successfully = false;
  const contract = new Contract(scriptPath);

  const deployTxHash = await contract.deploy(validatorIndex, datum);
  await waitUntilDeployed(deployTxHash);
  try {
    await contract.spend(validatorIndex, deployTxHash, redeemer);
    deployed_successfully = true;
  } catch { }

  expect(deployed_successfully).toBe(true);
}

async function waitUntilDeployed(deployTxHash: string) {
  const blockchainProvider = new BlockchainProvider();
  while (!await blockchainProvider.isTxDeployed(deployTxHash)) {
    await new Promise(resolve => setTimeout(resolve, 1000));
  };
}
