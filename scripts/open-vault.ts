import { Client, createClient } from "./helpers/client";
import { getAccounts, Token_Accounts } from "./helpers/accounts";
import { createFungibleToken, mintFT } from "./helpers/token-ops";

async function main() {
  const client: Client = await createClient();
  const ft = await createFungibleToken(client, { decimals: 8 });
  const accounts: Token_Accounts = await getAccounts(ft);
  await mintFT(
    client,
    accounts.provider_ata,
    accounts.provider_acc.address,
    ft.address
  );
  console.log(accounts.provider_ata);
  console.log(accounts);
}

main();
