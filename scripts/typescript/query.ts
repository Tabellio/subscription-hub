import { getClientsAndAccounts } from "./wallet";
import dotenv from "dotenv";

dotenv.config();

(async () => {
  const { adminClient, adminAccount } = await getClientsAndAccounts();

  let res = await adminClient.getContractMetadata(
    "archway1yl88z864ufynrtafdgl842zegvh26zlph7j8cw9zer7s96hgrzqs89wqjt"
  );

  console.log("\n🟠 Rewards Records: ", res, "\n");
})();
