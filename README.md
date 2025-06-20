nstructions
The smart contract exposes the following instructions:

1. initialize_config()
Description: Initializes the main configuration for the stablecoin system. This is typically called once by the program authority. It sets up the Config account and the Mint account for the stablecoin.
Signer: authority
Key Accounts Involved:
authority (writable, signer): The account designated as the admin/authority for the program.
config_account (writable, PDA): Stores global settings like liquidation parameters and the authority address. Seeds: ["config"].
mint_account (writable, PDA): The SPL token mint for the stablecoin. Seeds: ["mint"].
token_program: The SPL Token program.
system_program: The Solana System program.
Arguments: None.
2. deposit_collateral_and_mint_tokens()
Description: Allows a user to deposit SOL (or other configured collateral) and mint a corresponding amount of stablecoins, based on the current collateralization ratio and price feed data.
Signer: depositor (the user)
Key Accounts Involved:
depositor (writable, signer): The user performing the action.
config_account (PDA): Read to get configuration like health factors.
mint_account (writable): The stablecoin mint, to mint new tokens from.
collateral_account (writable, PDA): Stores the user's specific collateral deposit and minted token amount. Seeds: ["collateral", mint_account] (Note: IDL shows mint_account as a seed component, might be depositor in practice for uniqueness per user).
sol_account (writable, PDA): An escrow account holding the user's deposited SOL. Seeds: ["sol", depositor].
token_account (writable, PDA): The user's Associated Token Account (ATA) where minted stablecoins will be sent. Seeds: [depositor, token_program, mint_account].
price_update: The Pyth PriceUpdateV2 account providing the current price of the collateral (e.g., SOL/USD).
token_program, associated_token_program, system_program: Required Solana programs.
Arguments:
amount_collateral (u64): The amount of collateral (e.g., lamports of SOL) to deposit.
amount_to_mint (u64): The amount of stablecoins the user wishes to mint.
3. redeem_collateral_and_burn_tokens()
Description: Allows a user to burn their stablecoins and redeem a corresponding amount of their deposited collateral.
Signer: depositor (the user)
Key Accounts Involved:
depositor (writable, signer): The user performing the action.
config_account (PDA): Read for configuration.
mint_account (writable): The stablecoin mint, to burn tokens from.
collateral_account (writable, PDA): The user's collateral account. Seeds: ["collateral", depositor].
sol_account (writable, PDA): The escrow account from which SOL will be returned to the user.
token_account (writable): The user's token account holding the stablecoins to be burned.
price_update: The Pyth PriceUpdateV2 account.
token_program, system_program: Required Solana programs.
Arguments:
amount_collateral (u64): The amount of collateral the user wishes to redeem.
amount_to_burn (u64): The amount of stablecoins the user will burn.
4. liquidate()
Description: Allows a third-party liquidator to liquidate an undercollateralized position. The liquidator burns a certain amount of stablecoins and receives a portion of the liquidated user's collateral as a bonus.
Signer: liquidator
Key Accounts Involved:
liquidator (writable, signer): The user performing the liquidation.
config_account (PDA): Read for liquidation parameters (threshold, bonus).
mint_account (writable): The stablecoin mint, to burn the liquidator's tokens.
collateral_account (writable): The collateral account of the user whose position is being liquidated.
sol_account (writable): The SOL escrow account of the user being liquidated.
token_account (writable, PDA): The liquidator's token account from which stablecoins will be burned. Seeds: [liquidator, token_program, mint_account].
price_update: The Pyth PriceUpdateV2 account.
token_program, associated_token_program, system_program: Required Solana programs.
Arguments:
amount_to_burn (u64): The amount of stablecoins the liquidator is burning to perform the liquidation.
5. update_config()
Description: Allows the program authority to update configurable parameters, such as the minimum health factor.
Signer: authority
Key Accounts Involved:
authority (writable, signer): The admin/authority account.
config_account (writable, PDA): The global configuration account to be updated.
Arguments:
min_health_factor (u64): The new minimum health factor to be set.
Account Structures (Data Stored On-Chain)
1. Config (Stored in config_account PDA)
authority (pubkey): The public key of the account with administrative privileges over the program.
mint_account (pubkey): The public key of the SPL Token mint for the stablecoin.
liquidation_threshold (u64): The collateralization ratio below which a position is eligible for liquidation.
liquidation_bonus (u64): The percentage bonus a liquidator receives from the liquidated collateral.
min_health_factor (u64): A factor representing the health of a loan; falling below this makes a position vulnerable.
bump (u8): Bump seed for the config_account PDA.
bump_mint_account (u8): Bump seed for the mint_account PDA.
2. Collateral (Stored in collateral_account PDA for each user)
depositor (pubkey): The public key of the user who owns this collateral account.
sol_account (pubkey): The public key of the PDA (sol_account) that holds this user's actual SOL collateral.
token_account (pubkey): The public key of the user's ATA for the stablecoin.
lamports_balance (u64): The amount of SOL (in lamports) deposited by the user.
amount_minted (u64): The amount of stablecoins minted by the user against their collateral.
is_initialized (bool): A flag indicating whether this collateral account has been initialized.
bump (u8): Bump seed for this collateral_account PDA.
bump_sol_account (u8): Bump seed for the associated sol_account PDA.
3. PriceUpdateV2 (External Account from Pyth Network)
This account structure is defined by the Pyth Network and is used to consume price feed data.

write_authority (pubkey): Authority that can update this price feed.
verification_level (enum VerificationLevel): Level of Wormhole guardian signature verification.
price_message (struct PriceFeedMessage): Contains the actual price data:
feed_id ([u8; 32]): Unique ID of the price feed (e.g., SOL/USD).
price (i64): The reported price.
conf (u64): The confidence interval for the price.
exponent (i32): The exponent for the price (e.g., price * 10^exponent).
publish_time (i64): Timestamp of the price update.
prev_publish_time (i64): Timestamp of the previous price update.
ema_price (i64): Exponential moving average price.
ema_conf (u64): Exponential moving average confidence interval.
posted_slot (u64): The Solana slot at which this price update was posted on-chain.
Errors
The program defines the following custom errors:

InvalidPrice (6000): "Invalid Price" - Indicates an issue with the data from the price feed.
BelowMinHealthFactor (6001): "Below Minimum Health Factor" - Triggered if an action (like minting more tokens or collateral value dropping) would result in the user's position falling below the minimum required health factor.
AboveMinHealthFactor (6002): "Cannot liquidate a Healthy Account" - Attempting to liquidate a position that is still considered healthy (i.e., above the minimum health factor or liquidation threshold).
Constants
FEED_ID: "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"
This is likely the Pyth Network feed_id for the SOL/USD price feed on the respective Solana cluster (e.g., mainnet-beta, devnet).
