
## 📘 Stablecoin Smart Contract – Solana

A decentralized stablecoin system on Solana using SPL tokens and SOL as collateral. Powered by Anchor and Pyth price feeds.

---

### 🔧 Instructions

#### 1. `initialize_config()`

Initializes the config and mint accounts (PDA).

* **Signer:** Authority
* **Accounts:**

  * `config_account` \[PDA, "config"]
  * `mint_account` \[PDA, "mint"]
* **Programs:** `token_program`, `system_program`

---

#### 2. `deposit_collateral_and_mint_tokens(amount_collateral, amount_to_mint)`

Deposit SOL and mint stablecoins.

* **Signer:** Depositor
* **Accounts:**

  * `collateral_account` \[PDA, "collateral", depositor]
  * `sol_account` \[PDA, "sol", depositor]
  * `token_account` \[ATA for stablecoin]
  * `config_account`, `mint_account`, `price_update`
* **Programs:** `token_program`, `associated_token_program`, `system_program`

---

#### 3. `redeem_collateral_and_burn_tokens(amount_collateral, amount_to_burn)`

Burn stablecoins and redeem SOL.

* **Signer:** Depositor
* **Accounts:** Same as deposit
* **Programs:** `token_program`, `system_program`

---

#### 4. `liquidate(amount_to_burn)`

Liquidate undercollateralized positions. Burn tokens to receive SOL + bonus.

* **Signer:** Liquidator
* **Accounts:**

  * `collateral_account`, `sol_account` (target user)
  * `token_account` (liquidator)
  * `config_account`, `mint_account`, `price_update`
* **Programs:** `token_program`, `associated_token_program`, `system_program`

---

#### 5. `update_config(min_health_factor)`

Update system parameters (admin only).

* **Signer:** Authority
* **Accounts:** `config_account`

---

### 🗃️ On-Chain Accounts

#### Config (PDA: "config")

* `authority`, `mint_account`
* `liquidation_threshold`, `liquidation_bonus`
* `min_health_factor`, `bump`s

#### Collateral (PDA: "collateral", depositor)

* `depositor`, `sol_account`, `token_account`
* `lamports_balance`, `amount_minted`
* `is_initialized`, `bump`s

#### PriceUpdateV2 (Pyth)

Used for SOL/USD price via `FEED_ID`:
`7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE`

---

### ⚠️ Custom Errors

* `6000` Invalid Price
* `6001` Below Min Health Factor
* `6002` Cannot Liquidate Healthy Account

---
