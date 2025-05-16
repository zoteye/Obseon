use anchor_lang::prelude::*;

declare_id!("VAuLtObseon11111111111111111111111111111111111");

#[program]
pub mod obseon_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, ritual_phrase: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.creator = *ctx.accounts.creator.key;
        vault.ritual_phrase = ritual_phrase;
        vault.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        // Placeholder logic — vault closing might be tied to geo-location, sound file hash, or on-chain event
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = creator, space = 8 + 32 + 64 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub creator: Signer<'info>,
}

#[account]
pub struct Vault {
    pub creator: Pubkey,
    pub ritual_phrase: String,
    pub timestamp: i64,
}
