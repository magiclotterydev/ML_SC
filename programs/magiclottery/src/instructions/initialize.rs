use anchor_lang::prelude::*;
use crate::global_accounts::central_account::CentralStateData;
use crate::errors::errors::MyError;
use crate::CENTRAL_AUTHORITY_SEED;

#[derive(Accounts)]
pub struct InitializeCentralAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + CentralStateData::MAX_SIZE,
        seeds = [CENTRAL_AUTHORITY_SEED],
        bump
    )]
    pub central_authority: Account<'info, CentralStateData>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the upgrade authority of the program
    pub upgrade_authority: Signer<'info>,
    /// CHECK: This is the program data account
    #[account(
        constraint = program_data.upgrade_authority_address == Some(upgrade_authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    /// CHECK: This is the program account
    pub program: AccountInfo<'info>,
}

pub fn initialize_central_authority(ctx: Context<InitializeCentralAuthority>, authorizer_wallet: Pubkey) -> Result<()> {
    // Remove this check as we're initializing the account
    // require_eq!(ctx.accounts.central_authority.is_initialized, false, MyError::AlreadyInitialized);

    // Check if the signer is the upgrade authority of the program
    let upgrade_authority_info = &ctx.accounts.upgrade_authority;
    let program_data = &ctx.accounts.program_data;

    require!(
        program_data.upgrade_authority_address == Some(upgrade_authority_info.key()),
        MyError::UnauthorizedUpgradeAuthority
    );

    // Verify that the provided program account matches the program ID
    require_keys_eq!(
        ctx.accounts.program.key(),
        *ctx.program_id,
        MyError::InvalidProgramAccount
    );

    // Initialize the central authority
    ctx.accounts.central_authority.authorizer_wallet = authorizer_wallet;
    ctx.accounts.central_authority.is_initialized = true;
    // Initialize other fields as needed

    Ok(())
}