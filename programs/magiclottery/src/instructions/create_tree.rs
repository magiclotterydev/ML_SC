use anchor_lang::prelude::*;
use crate::global_accounts::central_account::CentralStateData;
use crate::errors::errors::MyError;
use crate::clones::program_ids::*;
use crate::CENTRAL_AUTHORITY_SEED;

use mpl_bubblegum::instructions::CreateTreeConfigCpiBuilder;

#[derive(Accounts)]
pub struct CreateLotteryTree<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(
        constraint = payer.key() != authorizer_wallet.key() @ MyError::PayerMustBeClient
    )]
    pub authorizer_wallet: Signer<'info>,

    #[account(
    seeds = [CENTRAL_AUTHORITY_SEED],
    bump,
    mut
    )]
    pub central_authority: Account<'info, CentralStateData>,

    /// CHECK: This account must be all zeros
    #[account(
    zero,
    signer
    )]
    pub merkle_tree: AccountInfo<'info>,

    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    // program
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
}

/// Creates a new lottery tree (merkle tree) for storing compressed NFT tickets
///
/// This function initializes a new merkle tree using the Bubblegum program.
/// It sets up the tree configuration and stores the merkle tree address in the central authority.
///
/// # Arguments
///
/// * `ctx` - The context struct containing the accounts required for the operation
/// * `max_depth` - The maximum depth of the merkle tree
/// * `max_buffer_size` - The maximum buffer size for the merkle tree
/// * `required_tree_account_size` - The required size of the tree account in bytes
///
/// # Returns
///
/// * `Result<()>` - Returns Ok(()) if the operation is successful, otherwise returns an error
pub fn create_lottery_tree<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateLotteryTree<'info>>,
        max_depth: u32,
        max_buffer_size: u32,
) -> Result<()> {
        msg!("initializing collection merkle tree");

        // Prepare the signer seeds for the CPI call
        let bump_seed = [ctx.bumps.central_authority];
        let signer_seeds: &[&[&[u8]]] = &[&[
            CENTRAL_AUTHORITY_SEED,
            &bump_seed,
        ]];

        // Build and invoke the CreateTreeConfig CPI to the Bubblegum program
        CreateTreeConfigCpiBuilder::new(
            &ctx.accounts.bubblegum_program.to_account_info(),
        )
            .tree_config(&ctx.accounts.tree_config.to_account_info())
            .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .tree_creator(&ctx.accounts.central_authority.to_account_info())
            .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
            .compression_program(&ctx.accounts.compression_program.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .max_depth(max_depth)
            .max_buffer_size(max_buffer_size)
            .public(false)
            .invoke_signed(signer_seeds)?;
        
        

        // Remove the line that stores the merkle tree address in the central authority
        // ctx.accounts.central_authority.merkle_tree_address = Some(ctx.accounts.merkle_tree.key());
        
        Ok(())
    }