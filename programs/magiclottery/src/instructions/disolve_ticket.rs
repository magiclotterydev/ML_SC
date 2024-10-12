use anchor_lang::prelude::*;
use crate::global_accounts::central_account::CentralStateData;
use crate::errors::errors::MyError;
use crate::clones::program_ids::*;
use mpl_bubblegum::instructions::BurnCpiBuilder;
use crate::CENTRAL_AUTHORITY_SEED;

#[derive(Accounts)]
pub struct DisolveTicket<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(
        constraint = authorizer_wallet.key() == central_authority.authorizer_wallet @ MyError::UnauthorizedWallet,
        constraint = payer.key() != authorizer_wallet.key() @ MyError::PayerMustBeClient
    )]
    pub authorizer_wallet: Signer<'info>,

    #[account(
        seeds = [CENTRAL_AUTHORITY_SEED],
        bump
    )]
    pub central_authority: Account<'info, CentralStateData>,
    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub leaf_owner: Signer<'info>,
    /// CHECK: This account is modified in the downstream program
    #[account(mut)]
    pub merkle_tree: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the instruction
    pub tree_config: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}



/// Dissolves (burns) a ticket NFT from the collection
///
/// This function burns a compressed NFT ticket using the Bubblegum program.
/// It verifies the merkle tree and other constraints before proceeding with the burn operation.
///
/// # Arguments
///
/// * `ctx` - The context struct containing the accounts required for the operation
/// * `root` - The root hash of the merkle tree
/// * `data_hash` - The hash of the NFT's data
/// * `creator_hash` - The hash of the NFT's creator
/// * `nonce` - A unique number used to prevent replay attacks
/// * `index` - The index of the leaf in the merkle tree
///
/// # Returns
///
/// * `Result<()>` - Returns Ok(()) if the operation is successful, otherwise returns an error
pub fn disolve_ticket<'info>(
    ctx: Context<'_, '_, '_, 'info, DisolveTicket<'info>>,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32
) -> Result<()> {
    msg!("buying ticket from collection");
    
    // Prepare the remaining accounts for the CPI call
    let remaining_accounts: Vec<(&AccountInfo, bool, bool)> = ctx.remaining_accounts
        .iter()
        .map(|account| (account, account.is_signer, account.is_writable))
        .collect();

    // Build and invoke the Burn CPI to the Bubblegum program
    BurnCpiBuilder::new(
        &ctx.accounts.bubblegum_program.to_account_info(),
    )
        .tree_config(&ctx.accounts.tree_config.to_account_info())
        .leaf_owner(&ctx.accounts.leaf_owner.to_account_info(), true)
        .leaf_delegate(&ctx.accounts.leaf_owner.to_account_info(), true)
        .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
        .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
        .compression_program(&ctx.accounts.compression_program.to_account_info())
        .system_program(&ctx.accounts.system_program.to_account_info())
        .add_remaining_accounts(&remaining_accounts)
        .root(root)
        .data_hash(data_hash)
        .creator_hash(creator_hash)
        .nonce(nonce)
        .index(index)
        .invoke()?;

    Ok(())
}