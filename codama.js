import { createCodamaConfig } from "gill";
 
export default createCodamaConfig({
  idl: "target/idl/staking_vault.json",
  clientJs: "scripts/codama/generated",
});