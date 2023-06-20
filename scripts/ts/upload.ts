import fs from "fs";
import { getClientsAndAccounts } from "./wallet";
import dotenv from "dotenv";

(async () => {
  dotenv.config();

  const { adminClient, adminAccount } = await getClientsAndAccounts();

  const WASM_FILE_PATH = process.env.WASM_FILE_PATH || "";

  // Read the file contents as a Buffer
  const fileBuffer = fs.readFileSync(WASM_FILE_PATH);

  // Convert the Buffer to Uint8Array
  const wasmArray = new Uint8Array(fileBuffer.buffer);

  let res = await adminClient.upload(adminAccount.address, wasmArray, "auto");

  console.log("\n🟠 Subscription Hub Code ID: ", res.codeId, "\n");
})();
