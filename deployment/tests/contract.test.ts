import { Data, mConStr0, MeshWallet, UTxO } from "@meshsdk/core";
import { Contract, mVoid } from "../contract";
import { BlockchainProvider } from "../blockchain_provider";

describe('Contract Deployment', () => {
  let scriptPath: string;
  let blockchainProvider: BlockchainProvider;
  let wallet: MeshWallet;
  beforeEach(() => {
    scriptPath = "./tests/plutus.json";
    blockchainProvider = new BlockchainProvider();
    wallet = blockchainProvider.getWalletUsing('me.sk');
  })

  test.skip('Happy path with datum and redeemer', async () => {
    const datum = 42;
    const redeemer = 42;
    await testDeploymentWith(scriptPath, 0, datum, redeemer);
  }, 150000);

  test.skip('Happy path without datum and redeemer', async () => {
    const datum = mVoid();
    const redeemer = mVoid();

    await testDeploymentWith(scriptPath, 2, datum, redeemer);
  }, 150000);

  test('Happy path with manual budget', async () => {
    const datum = mVoid();
    const redeemer = mVoid();
    const redeemerBudget = { mem: 50000, steps: 25000000 };
    await testDeploymentWith(scriptPath, 2, datum, redeemer, redeemerBudget);
  }, 150000);

  async function testDeploymentWith(scriptPath: string, validatorIndex: number, datum: Data, redeemer: Data, redeemerBudget?: {mem: number, steps: number}) {
    const contract = new Contract(scriptPath, blockchainProvider, wallet);

    const deployedTxHash = await contract.deploy(validatorIndex, datum);

    await waitUntilDeployed(deployedTxHash);
    await waitUntilWalletUTxOsHaveBeenUpdated(deployedTxHash);

    const deployed_successfully = await try_execution(async () => {
      const spendTxHash = await contract.spend(validatorIndex, deployedTxHash, redeemer, redeemerBudget);
      await waitUntilDeployed(spendTxHash);
    });

    expect(deployed_successfully).toBe(true);
  }

  async function try_execution(closure: () => Promise<void>): Promise<boolean> {
    try {
      await closure();
      return true;
    } catch {
      return false;
    }
  }

  async function waitUntilWalletUTxOsHaveBeenUpdated(deployTxHash: string) {
    while (!await hasCollateralTxBeenSynchronizedTo(deployTxHash)) {
      await oneSecondTimer();
    }
  }

  async function waitUntilDeployed(deployTxHash: string) {
    while (!await blockchainProvider.hasTxBeenImpactedOnBlockchain(deployTxHash)) {
      await oneSecondTimer();
    };
  }

  async function oneSecondTimer() {
    await new Promise(resolve => setTimeout(resolve, 1000));
  }

  async function hasCollateralTxBeenSynchronizedTo(scriptTxHash: string): Promise<boolean> {
    const collaterals: UTxO[] = await wallet.getCollateral();
    return collaterals.some((collateral: UTxO) => collateral.input.txHash === scriptTxHash)
  }
});
