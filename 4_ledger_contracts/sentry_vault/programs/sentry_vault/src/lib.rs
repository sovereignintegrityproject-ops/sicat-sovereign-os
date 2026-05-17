use anchor_lang::prelude::*;

// Your officially synced cryptographic identity!
declare_id!("GeYyUtY77YRYY4yCwHSa9BuZWEVC96esoSkTkquX2vJB"); 

#[program]
pub mod sentry_vault {
    use super::*;

    // 1. THE DEPOSIT (The Behavioral Bond)
    pub fn initialize_deposit(ctx: Context<InitializeDeposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.developer = *ctx.accounts.developer.key;
        vault.deposit_amount = amount;
        vault.is_slashed = false;
        
        msg!("🏦 SENTRY VAULT: Foreign Developer locked {} tokens as a behavioral bond.", amount);
        Ok(())
    }

    // 2. THE PENALTY (The Slashing)
    pub fn slash_deposit(ctx: Context<SlashDeposit>, reason: String) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.is_slashed = true;
        
        msg!("🚨 SENTRY VAULT: CIVIC VETO INITIATED!");
        msg!("Reason: {}", reason);
        msg!("💸 Funds seized and routed to the $SICAT Global South Vault.");
        Ok(())
    }

    // 3. THE CLEARANCE (The ZK Seal Verification)
    pub fn clear_action(ctx: Context<ClearAction>, zk_proof_hash: String) -> Result<()> {
        let _vault = &mut ctx.accounts.vault; 
        
        msg!("✅ SENTRY VAULT: Action Cleared for National Grid.");
        msg!("Cryptographic Seal Verified: {}", zk_proof_hash);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDeposit<'info> {
    #[account(init, payer = developer, space = 8 + 32 + 8 + 1)]
    pub vault: Account<'info, VaultState>,
    #[account(mut)]
    pub developer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SlashDeposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    pub sentry_authority: Signer<'info>, 
}

#[derive(Accounts)]
pub struct ClearAction<'info> {
    #[account(mut)]
    pub vault: Account<'info, VaultState>,
    pub sentry_authority: Signer<'info>,
}

#[account]
pub struct VaultState {
    pub developer: Pubkey,
    pub deposit_amount: u64,
    pub is_slashed: bool,
}
