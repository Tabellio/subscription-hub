import { makeCosmoshubPath } from "@cosmjs/amino";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { LedgerSigner } from "@cosmjs/ledger-amino";
import TransportNodeHid from "@ledgerhq/hw-transport-node-hid";
import { SigningArchwayClient } from "@archwayhq/arch3-core";
import dotenv from "dotenv";

dotenv.config();

const TESTNET_RPC = "https://rpc.constantine.archway.tech:443";

const PREFIX = "archway";

const ADMIN_MNEMONIC = process.env.ADMIN_MNEMONIC || "";
const ORGANIZATION_MNEMONIC = process.env.ORGANIZATION_MNEMONIC || "";
const USER_MNEMONIC = process.env.USER_MNEMONIC || "";

export const getSigner = async (mnemonic: string) => {
  const signer = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: PREFIX,
  });
  return signer;
};

export const getLedgerSigner = async () => {
  const transport = await TransportNodeHid.create();
  return new LedgerSigner(transport, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: PREFIX,
  });
};

export const getClientsAndAccounts = async () => {
  const adminSigner = await getSigner(ADMIN_MNEMONIC);
  const adminAccount = (await adminSigner.getAccounts())[0];

  const organizationSigner = await getSigner(ORGANIZATION_MNEMONIC);
  const organizationAccount = (await organizationSigner.getAccounts())[0];

  const userSigner = await getSigner(USER_MNEMONIC);
  const userAccount = (await userSigner.getAccounts())[0];

  const adminClient = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    adminSigner
  );
  const organizationClient = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    organizationSigner
  );
  const userClient = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    userSigner
  );

  return {
    adminAccount,
    adminClient,
    organizationAccount,
    organizationClient,
    userAccount,
    userClient,
  };
};
