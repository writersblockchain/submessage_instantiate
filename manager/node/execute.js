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

const contractAddress = "secret1em240awdxwxkwywk6u48re735ua2g8u3vw9awh";
const contractCodeHash =
  "d62cf696a17fbbb76bab55196e731711ac3feb7d282250c27ec6726030b3c7c9";

let increase_count = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        increment: {
          contract: "secret1cldglt6wvueva2akly4x3wvzzlevk2hxzv0cvq",
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

// increase_count();

let instantiate_contract = async () => {
  const tx = await secretjs.tx.compute.executeContract(
    {
      sender: wallet.address,
      contract_address: contractAddress,
      msg: {
        instantiate: {
          code_id: 12982,
          code_hash: "138c4984186458a1ea7887dd271295f8f2547fc381d8991c780fdd2eb2f6ee73"
        },
      },
      code_hash: contractCodeHash,
    },
    { gasLimit: 100_000 }
  );

  console.log(tx);
};

instantiate_contract();
