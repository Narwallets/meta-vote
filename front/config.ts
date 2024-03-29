export const getConfig = (env: string)  => {
  const CONTRACT_NAME =
    process.env.CONTRACT_NAME || "metavote.testnet";
  switch (env) {
    case "production":
    case "mainnet":
      return {
        networkId: "mainnet",
        nodeUrl: "https://rpc.mainnet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.near.org",
        helperUrl: "https://helper.mainnet.near.org",
        explorerUrl: "https://explorer.mainnet.near.org",
        metapoolUrl: "https://metapool.app/dapp/mainnet/meta",
        refFinance: "https://app.ref.finance/#meta-pool.near%7Cmeta-token.near",
        metayieldUrl: "https://metayield.app" 

      };
    case "development":
    case "testnet":
    case "preview":
      return {
        networkId: "testnet",
        nodeUrl: "https://rpc.testnet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
        metapoolUrl: "https://metapool.app/dapp/testnet/meta",
        refFinance: "https://app.ref.finance/#meta-pool.near%7Cmeta-token.near",
        metayieldUrl: "https://metayield.app" 
      };
    case "betanet":
      return {
        networkId: "betanet",
        nodeUrl: "https://rpc.betanet.near.org",
        contractName: CONTRACT_NAME,
        walletUrl: "https://wallet.betanet.near.org",
        helperUrl: "https://helper.betanet.near.org",
        explorerUrl: "https://explorer.betanet.near.org",
        metapoolUrl: "https://metapool.app/dapp/testnet/meta",
        refFinance: "https://app.ref.finance/#meta-pool.near%7Cmeta-token.near",
        metayieldUrl: "https://metayield.app" 
      };
    case "local":
      return {
        networkId: "local",
        nodeUrl: "http://localhost:3030",
        keyPath: `${process.env.HOME}/.near/validator_key.json`,
        walletUrl: "http://localhost:4000/wallet",
        metapoolUrl: "https://metapool.app/dapp/testnet/meta",
        refFinance: "https://app.ref.finance/#meta-pool.near%7Cmeta-token.near",
        contractName: CONTRACT_NAME,
        metayieldUrl: "https://metayield.app" 
      };
    case "test":
    case "ci":
      return {
        networkId: "shared-test",
        nodeUrl: "https://rpc.ci-testnet.near.org",
        contractName: CONTRACT_NAME,
        masterAccount: "test.near",
      };
    case "ci-betanet":
      return {
        networkId: "shared-test-staging",
        nodeUrl: "https://rpc.ci-betanet.near.org",
        contractName: CONTRACT_NAME,
        masterAccount: "test.near",
      };
    default:
      throw Error(
        `Unconfigured environment '${env}'. Can be configured in src/config.js.`
      );
  }
}