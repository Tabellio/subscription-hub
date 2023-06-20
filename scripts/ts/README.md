# Subscription Hub Scripts

These scripts are used to help manage the Subscription Hub.

## Getting Started

- Copy the `.env.example` file to `.env` and update the variables.

```bash
cp .env.example .env
```

- You need 3 mnemonics for each actor. **Admin**, **Organization** and **User**. Update the `ADMIN_MNEMONIC`, `ORGANIZATION_MNEMONIC` and `USER_MNEMONIC` variables in the `.env` file.

Now you are ready to run the scripts!

### Upload Script

> **Note**: You need to compile your contract before uploading it to the blockchain. Refer to the [Compile Script](../../README.md#getting-started) section for more information.

After getting your wasm file, update the `WASM_FILE_PATH` variable in the `.env` file.

To upload the contract to the blockchain, use the following command:

```bash
./scripts/upload.sh
```

After the script is executed, you will see the contract code ID in the terminal. Copy the code ID and update the `SUBSCRIPTION_HUB_CODE_ID` variable in the `.env` file.

You are now ready to use the start script!

### Start Script

> **Note**: You first need to upload your contract to the blockchain. Refer to the [Upload Script](#upload-script) section for more information.

Start script is used to execute multiple operations at once. It is used to try out the Subscription Hub.

1. It creates a new subscription hub with an admin wallet.
2. It creates a new organization with an organization wallet.
3. It creates a new subscription plan with the same organization wallet.
4. It subscribes to the subscription plan with the user wallet.

To run the script, use the following command:

```bash
./scripts/start.sh
```
