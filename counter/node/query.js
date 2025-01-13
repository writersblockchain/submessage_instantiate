import { SecretNetworkClient, Wallet } from "secretjs";
import dotenv from "dotenv";
dotenv.config();

const wallet = new Wallet("desk pigeon hammer sleep only mistake stool december offer patrol once vacant");

const secretjs = new SecretNetworkClient({
  chainId: "pulsar-3",
  url: "https://api.pulsar3.scrttestnet.com",
  wallet: wallet,
  walletAddress: wallet.address,
});

const contractAddress = "secret1fusjc28f3wk5wy2fumel9dqg0jk25uzay6wvmz";
const contractCodeHash =
"138c4984186458a1ea7887dd271295f8f2547fc381d8991c780fdd2eb2f6ee73";

let query_count = async () => {
  const count = await secretjs.query.compute.queryContract({
    contract_address: contractAddress,
    query: {
      get_count: {},
    },
    code_hash: contractCodeHash,
  });

  console.log(count);
};
query_count();
