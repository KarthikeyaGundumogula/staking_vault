```yaml

/** VaultConfig represents the overall configuration and state of the vault */

VaultConfig {
    locking_token_mint: Pubkey,
    reward_token_mint: Pubkey,

    min_cap: u64,
    max_cap: u64,
    total_capital_collected: u64,

    beneficiaries: Vec<Pubkey>,
    beneficiary_shares_bps: Vec<u16>,
    investor_bps: u16,

    max_slash_bps: u16,
    nft_collection: Pubkey,

    reward_distributor: Pubkey,
    node_operator: Pubkey,
    attestor: Pubkey,

    vault_phase: u8,
    
    // Slashing
    is_dispute_active: bool,
    dispute_ends_at: i64,
    pending_slash_amount: u64,
    slash_claimant: Pubkey,

}
/** PositionState represents an individual user's position in the vault */

PositionState {
    vault: Pubkey,
    
    total_value_locked: u64,

    total_rewards_claimed: u64,
    last_reward_timestamp: i64,

    is_listed: bool,
}

```


| Instruction                   | Who Can Call       | Phase            |
| ----------------------------- | ------------------ | ---------------- |
| initialize_vault              | Admin              | -                |
| deposit_capital               | Capital Provider   | Formation        |
| withdraw_capital_early        | Capital Provider   | Formation        |
| mint_position_nft             | Vault (internal)   | Formation        |
| start_active_phase            | Vault (internal)   | Auto-trigger     |
| deposit_rewards               | Reward Distributor | Active           |
| claim_rewards                 | NFT Holder         | Active&&!dispute |
| list_position                 | NFT Holder         | Active           |
| delist_position               | NFT Holder         | Active           |
| Buy_position                  | Market Participant | Active           |
| raise_slash_request           | Attestor           | Active           |
| submit_slash_proof            | Attestor           | Dispute          |
| execute_slash                 | Vault (internal)   | Dispute          |
| resolve_dispute_without_slash | Vault (internal)   | Dispute          |
| start_closure                 | Vault (internal)   | End of Active    |
| withdraw_principal            | NFT Holder         | Closed           |
| burn_position_nft             | Vault (internal)   | Closed           |



A. initialization/

initialize_vault

update_vault_config

B. capital/

deposit_capital

withdraw_capital_early

start_active_phase (internal)

C. nft/

list_position

delist_position

update_position_after_transfer (internal)

burn_position_nft (internal)

D. rewards/

deposit_rewards

claim_rewards

E. slashing/

raise_slash_request

submit_slash_proof

transition_to_dispute_window (internal)

execute_slash (internal)

resolve_dispute_without_slash (internal)

F. lifecycle/

transition_to_active_phase (internal)

transition_to_closed_phase (internal)

withdraw_principal

G. admin/

admin_rescue_tokens

admin_pause_vault

admin_resume_vault


### clients

```javascript
// Creating a vault with beneficiaries
const beneficiaries = [
  {
    address: beneficiary1.publicKey,
    shareBps: 2000, // 20%
    totalClaimed: new BN(0),
  },
  {
    address: beneficiary2.publicKey,
    shareBps: 3000, // 30%
    totalClaimed: new BN(0),
  },
];

await program.methods
  .createVault({
    minCap: new BN(1000),
    maxCap: new BN(10000),
    beneficiaries: beneficiaries,
    investorBps: 5000, // 50%
    // ... other params
  })
  .accounts({...})
  .rpc();
```