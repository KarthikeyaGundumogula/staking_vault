## **📍 DePIN Staking Vaults — Powered by Delegated Stake & Tokenized Positions**

This project enables a **staking marketplace on top of DePIN networks**, where real-world infrastructure providers (charging stations, mobility nodes, hotspots, etc.) must stake tokens to operate and earn rewards.

This protocol lets **regular users delegate stake**, allowing providers to scale while stakers earn yield. Each position is represented as a transferable **NFT (similar to UniV3 LP NFTs)**.

---

## **🌍 Why This Exists**

DePIN operators need capital to stake before operating. This protocol unlocks liquidity sources by pairing capital providers with infrastructure operators.

| Role          | Benefit                                |
| ------------- | -------------------------------------- |
| **Stakers**   | Earn yield without running hardware    |
| **Providers** | Scale operations using delegated stake |
| **Traders**   | Enter or exit yield positions via NFTs |

---

## **🔑 Core Concepts**

### **📌 Staking Vaults**

* One provider ↔ one staker
* Provider deposits rewards manually over time
* NFT represents vault ownership
* Closed only when balances are zero
* NFT burned on close

### **📌 Position NFTs**

* Minted on vault open
* Transferred on marketplace
* Represent rights to withdraw stake & rewards

### **📌 Marketplace Program**

* Escrow deposit + claim flows
* Trading positions without interacting with vault
* No staking logic inside marketplace

---

## **⚙️ Programs Overview**

| Program                     | Purpose                                                  | Program ID                                     |
| --------------------------- | -------------------------------------------------------- | ---------------------------------------------- |
| **Staking Vault Program**   | Manages staking, withdrawals, rewards, and NFT lifecycle | `DW9BXusirecGep9k5FXFDALYiY1HPtBpVWwPJ36ZD8KZ` |
| **NFT Marketplace Program** | Manages position transfers & escrow                      | `3kLob38A4tG8m3fP9ZZwSWsjdr417DjQZ4bkqxGFjaUh` |

---

## **🧑‍🤝‍🧑 User Stories**

### **<span style="color: rgb(255,165,0)">🧑‍💼 Provider Stories</span>**

* **Given** a new vault configuration
  **When** I open a vault with a staker
  **Then** delegated stake can be used to scale my DePIN operations.

* **Given** rewards are earned off-chain
  **When** I deposit rewards into the vault
  **Then** the staker receives claimable returns.

---

### **<span style="color: rgb(0,180,255)">💰 Staker Stories</span>**

* **Given** a provider-bound vault
  **When** I stake tokens
  **Then** I earn rewards without operating infrastructure.

* **Given** my vault position is represented as an NFT
  **When** I transfer or sell the NFT
  **Then** ownership transfers without unstaking.

* **Given** the staking duration has ended
  **When** I withdraw
  **Then** I receive principal + rewards.

---

### **<span style="color: rgb(140,255,0)">📈 Trader Stories</span>**

* **Given** NFTs represent vault ownership
  **When** I buy one in the marketplace
  **Then** I inherit withdrawal rights.

* **Given** NFTs are transferable
  **When** ownership changes hands
  **Then** staking and rewards remain unaffected.

---

### **<span style="color: rgb(255,0,200)">🛠 Protocol / Developer Stories</span>**

* **Given** staking logic lives in the vault program
  **When** NFTs are traded
  **Then** the staking state remains secure.

* **Given** a vault holds balances
  **When** all balances reach zero
  **Then** the vault can be closed and NFT burned.

---

## **🪙 Token & Reward Model**

| Detail        | Rule                         |
| ------------- | ---------------------------- |
| Reward source | Manual deposits by provider  |
| Recipient     | 100% to NFT Owner            |
| Token types   | Determined by DePIN network  |
| Exit method   | Withdraw + burn NFT on close |

> No automated emissions. Vaults are isolated agreements.

---

## **🧩 Design Principles**

* NFT = ownership layer, not collateral
* Vault burns on close
* Inspired by Uniswap V3 architecture
* Marketplace does not affect staking logic
* Future: store metadata on NFT using MPL attributes

---


## **⚡ Installation & Quick Start (Local Only)**

> **The protocol is still under active development.
> Currently only local validation and testing are supported.**
> Both Anchor tests and Gill scripts are available depending on your workflow.

---

### **📍 Prerequisites**

Make sure you have:

* **Node.js + Yarn**
* **Rust + Solana CLI**
* **Anchor Framework**
* **Gill CLI (optional)**

To install dependencies and build the programs:

```sh
yarn && anchor build
```

---

### **🧪 Running Anchor Tests**

For full program-level tests:

```sh
anchor test
```

> This runs against Anchor’s built-in test validator and executes everything end-to-end.

---

### **🧰 Running via Gill Scripts**

Gill scripts simulate real-world interaction flows and are useful for non-Solana devs or testers who don’t want to write Rust/Anchor code.

#### **Step 1️⃣ Start Local Validator (with MPL-Core preloaded)**

A custom script spins up a localnet with required programs already included:

```sh
anchor run localnet
```

> Leave this terminal running—it acts as the blockchain node.

---

#### **Step 2️⃣ Deploy Programs (in a new terminal)**

```sh
anchor deploy
```

---

#### **Step 3️⃣ Open a Vault**

Runs the script located at:

```
scripts/open-vault.ts
```

Execute:

```sh
anchor run open
```

After running, **copy values from logs**:

* **asset address (NFT mint)**
* **staking token address**

These will be required for the next step.

---

#### **Step 4️⃣ Stake to the Vault**

Before running, open the script:

```
scripts/stake.ts
```

Update arguments (vault, asset, staking token).

Then run:

```sh
anchor run stake
```

---


## **🔜 Roadmap**

| Feature                        | Status         |
| ------------------------------ | -------------- |
| On-NFT attribute storage       | Research phase |
| Multi-staker aggregated vaults | Planned        |
| Slashing with aggr vaults      | Planned        |
| Automated reward indexing      | Planned        |
| Cross-chain DePIN integrations | Long-term      |

---
