# User Stories for Capital Layer Protocol Staking Vault

## **Initialization**

* As an admin, when I initialize a vault with valid configuration, the vault is created and the formation phase begins. ✅
* As an admin, when I initialize a vault with invalid shares or min/max cap mismatch, the transaction fails with "Invalid Configuration". ✅
* As an admin, when I initialize a vault a new MPL-Core collection is created. ✅

---

## **Capital Formation**

* As a capital provider, when I deposit capital during Formation phase, a Position NFT is minted and my deposit is recorded in the vault. ✅
* As a capital provider, when I try to deposit after Active phase has started, the transaction fails with "Vault Not Accepting Deposits".✅
* As a capital provider, when I withdraw early in Formation phase, my capital is returned minus the fee and my Position NFT is burned. ✅
* As a capital provider, when I try to withdraw early after the Active phase has begun, the transaction fails with "Capital Locked". ✅
* As a capital provider, I can close my position after fundraise period if the vault didn't reach the min_cap, and withdraw my funds. ✅
* As a node operator, I can clsoe the vault after fundraise period if the vault didn't reach the min_cap and its ata is empty. ✅
  
---

## **Active Phase Behavior**

* As a position holder, when rewards are deposited by the reward distributor, my claimable rewards increase proportionally to my stake. ✅
* As a position holder, when I claim rewards, only the accumulated rewards are transferred. ✅
* As a position holder, When I try to Unlock prinicipal, the transaction failes with "Active Phase, Rewards Locked" error. ✅
* As a position holder, when I list my Position NFT, the marketplace marks it as listed. 
* As a buyer, when I purchase a listed Position NFT, I become the new owner of the locked position and rewards.
* As a seller, when I try to list a Position NFT I do not own, the transaction fails with "Unauthorized".

---

## **Reward Deposit Validation**

* As a reward distributor, when I deposit rewards during the Active phase, rewards are added to the vault. ✅
* As a reward distributor, when I try to deposit rewards using a non-authorized wallet, the transaction fails with "Invalid Reward Distributor". ✅
* As a reward distributor, when I deposit rewards with the wrong token mint, the transaction fails with "Invalid Reward Token". ✅

---

## **Slashing & Dispute Window**

* As an agent, when I raise a slashing request during the Active phase, a dispute window opens and slashing amount is recorded. ✅
* As a agent, when I submit a slashing request exceeds max_slash_bps, the transaction fails with "Slash Amount Exceeds Limit". ✅
* As an agent, when I try to raise a slashing request outside the Active phase, the transaction fails with "Invalid Phase". ✅
* As an agent, when I submit slashing proof before the dispute window expires, the slash amount is approved. ✅
* As an agent, when I fail to submit proof within the dispute window, the vault dismisses the slash request automatically. ✅
* As a node operator, when I continue depositing rewards during the dispute window, deposits succeed. ✅
* As a position holder, when I try to claim rewards during a dispute, the transaction fails with "Vault in Dispute". ✅

---

## **Closure Phase**

* As a vault, when the Active phase duration ends, the vault transitions to Closed phase.
* As a position holder, when I withdraw my principal in Closed phase, I receive my pro-rata capital and my Position NFT is burned.
* As a position holder, when I try to withdraw principal before closure, the transaction fails with "Vault Not Closed".

---

## **Admin Safety**

* As an admin, when I rescue foreign tokens, only non-locking and non-reward tokens can be withdrawn.
* As an admin, when I pause the vault, all state-changing instructions fail with "Vault Paused".
* As an admin, when I resume the vault, normal operations continue.

---
