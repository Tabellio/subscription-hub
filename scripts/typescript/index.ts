import { getClientsAndAccounts } from "./wallet";
import dotenv from "dotenv";

dotenv.config();

interface Response {
  id: number;
  data: Record<string, string>;
}

const SUBSCRIPTION_HUB_CODE_ID = process.env.SUBSCRIPTION_HUB_CODE_ID || "";

(async () => {
  const {
    adminClient,
    adminAccount,
    organizationClient,
    organizationAccount,
    userClient,
    userAccount,
  } = await getClientsAndAccounts();

  const createNewSubscriptionHub = async () => {
    const res = await adminClient.instantiate(
      adminAccount.address,
      Number(SUBSCRIPTION_HUB_CODE_ID),
      {},
      "Subscription Hub",
      "auto",
      {
        admin: adminAccount.address,
      }
    );

    console.log(
      "\n🟠 Subscription Hub Contract Address: ",
      res.contractAddress,
      "\n"
    );

    return res.contractAddress;
  };

  const createNewOrganization = async (contractAddress: string) => {
    let res = await organizationClient.execute(
      organizationAccount.address,
      contractAddress,
      {
        create_organization: {
          name: "My New Organization",
          description: "This is my new organization",
          website: "https://my-organization.com",
          metadata: {
            foo: "bar",
            baz: "qux",
          },
        },
      },
      "auto"
    );

    console.log(
      "\n🟠 Create New Organization TxHash: ",
      res.transactionHash,
      "\n"
    );

    let organizations: Response[] = await organizationClient.queryContractSmart(
      contractAddress,
      {
        user_organizations: {
          user_address: organizationAccount.address,
        },
      }
    );

    console.log(
      "🟠 Organization: ",
      JSON.stringify(organizations[organizations.length - 1], null, 2),
      "\n"
    );

    return organizations[organizations.length - 1];
  };

  const createNewSubscriptionPlan = async (
    contractAddress: string,
    organizationId: number
  ) => {
    let res = await organizationClient.execute(
      organizationAccount.address,
      contractAddress,
      {
        create_subscription_plan: {
          organization_id: organizationId,
          name: "My New Subscription Plan",
          description: "This is my new subscription plan",
          price: "500000", // 0.5
          duration: 1,
          duration_unit: "month",
          cancelable: true,
          refundable: false,
        },
      },
      "auto"
    );

    console.log(
      "\n🟠 Create New Subscription Plan TxHash: ",
      res.transactionHash,
      "\n"
    );

    let subscriptionPlans: Response[] =
      await organizationClient.queryContractSmart(contractAddress, {
        organization_subscription_plans: {
          organization_id: organizationId,
        },
      });

    console.log(
      "🟠 Subscription Plan: ",
      JSON.stringify(subscriptionPlans[subscriptionPlans.length - 1], null, 2),
      "\n"
    );

    return subscriptionPlans[subscriptionPlans.length - 1];
  };

  const subscribeToPlan = async (contractAddress: string, planId: number) => {
    let res = await userClient.execute(
      userAccount.address,
      contractAddress,
      {
        subscribe_plan: {
          plan_id: planId,
        },
      },
      "auto"
    );

    console.log("🟠 Subscribe Plan TxHash: ", res.transactionHash, "\n");

    let subscriptions: Response[] = await userClient.queryContractSmart(
      contractAddress,
      {
        user_subscriptions: {
          user_address: userAccount.address,
        },
      }
    );

    console.log(
      "🟠 Subscriptions: ",
      JSON.stringify(subscriptions, null, 2),
      "\n"
    );
  };

  // Create a new subscription hub
  let subscriptionHubContractAddress = await createNewSubscriptionHub();

  // Create a new organization
  let organization = await createNewOrganization(
    subscriptionHubContractAddress
  );

  // Create a new subscription plan
  let subscriptionPlan = await createNewSubscriptionPlan(
    subscriptionHubContractAddress,
    Number(organization.id)
  );

  await subscribeToPlan(
    subscriptionHubContractAddress,
    Number(subscriptionPlan.id)
  );
})();
