use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::global_accounts::central_account::CentralStateData;
use crate::errors::errors::MyError;
use crate::clones::program_ids::*;
use mpl_bubblegum::instructions::MintToCollectionV1CpiBuilder;
use mpl_bubblegum::types::{Collection, MetadataArgs, TokenProgramVersion, TokenStandard};
use crate::CENTRAL_AUTHORITY_SEED;

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(
        constraint = authorizer_wallet.key() == central_authority.authorizer_wallet @ MyError::UnauthorizedWallet
    )]
    pub authorizer_wallet: Signer<'info>,

    #[account(
        seeds = [CENTRAL_AUTHORITY_SEED],
        bump
    )]
    pub central_authority: Account<'info, CentralStateData>,

    /// CHECK: This account is checked in the instruction
    #[account(mut)]
    pub tree_config: UncheckedAccount<'info>,

    /// CHECK: This account is neither written to nor read from.
    pub leaf_owner: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: unsafe
    pub merkle_tree: UncheckedAccount<'info>,

    // pub tree_delegate: Signer<'info>,

    /// CHECK: This account is checked in the instruction
    pub collection_mint: UncheckedAccount<'info>,

    /// CHECK:
    #[account(mut)]
    pub collection_metadata: UncheckedAccount<'info>,

    /// CHECK: This account is checked in the instruction
    pub edition_account: UncheckedAccount<'info>,
    /// CHECK: This is the PDA that will receive the payment
    #[account(mut)]
    pub payment_receiver: UncheckedAccount<'info>,

    /// CHECK: This is just used as a signing PDA.
    pub bubblegum_signer: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub token_metadata_program: Program<'info, MplTokenMetadata>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
    
}

/// Buys a ticket for the lottery by minting a compressed NFT to the specified collection.
///
/// # Arguments
///
/// * `ctx` - The context of the instruction, containing all necessary accounts.
/// * `name` - The name of the NFT ticket.
/// * `symbol` - The symbol of the NFT ticket.
/// * `uri` - The URI pointing to the metadata of the NFT ticket.
/// * `seller_fee_basis_points` - The royalty fee in basis points (100 = 1%).
/// * `payment_amount` - The amount of SOL to transfer.
/// * `payment_receiver_key` - The key of the PDA that will receive the payment.
///
/// # Returns
///
/// Returns `Ok(())` if the ticket is successfully purchased, or an error otherwise.
pub fn buy_ticket<'info>(ctx: Context<'_, '_, '_, 'info, BuyTicket<'info>>,
        name: String,
        symbol: String,
        uri: String,
        seller_fee_basis_points: u16,
        deposit_amount: u64
    ) -> Result<()> {
        msg!("buying ticket for collection");

        // Transfer SOL from payer to payment receiver PDA
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.payment_receiver.to_account_info(),
                },
            ),
            deposit_amount,
        )?;

        // Prepare the signer seeds for the CPI call
        let bump_seed = [ctx.bumps.central_authority];
        let signer_seeds: &[&[&[u8]]] = &[&[
            CENTRAL_AUTHORITY_SEED,
            &bump_seed,
        ]];
        
        // Build and invoke the MintToCollectionV1 CPI with the prepared builder and signer seeds
        MintToCollectionV1CpiBuilder::new(
            &ctx.accounts.bubblegum_program.to_account_info(),
        )
            .tree_config(&ctx.accounts.tree_config.to_account_info())
            .leaf_owner(&ctx.accounts.leaf_owner.to_account_info())
            .leaf_delegate(&ctx.accounts.leaf_owner.to_account_info())
            .merkle_tree(&ctx.accounts.merkle_tree.to_account_info())
            .payer(&ctx.accounts.payer.to_account_info())
            .tree_creator_or_delegate(&ctx.accounts.central_authority.to_account_info())
            .collection_authority(&ctx.accounts.central_authority.to_account_info())
            .collection_authority_record_pda(Some(&ctx.accounts.bubblegum_program.to_account_info()))
            .collection_mint(&ctx.accounts.collection_mint.to_account_info())
            .collection_metadata(&ctx.accounts.collection_metadata.to_account_info())
            .collection_edition(&ctx.accounts.edition_account.to_account_info())
            .bubblegum_signer(&ctx.accounts.bubblegum_signer.to_account_info())
            .log_wrapper(&ctx.accounts.log_wrapper.to_account_info())
            .compression_program(&ctx.accounts.compression_program.to_account_info())
            .token_metadata_program(&ctx.accounts.token_metadata_program.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .metadata(
                MetadataArgs {
                    name,
                    symbol,
                    uri,
                    creators: vec![],
                    seller_fee_basis_points,
                    primary_sale_happened: false,
                    is_mutable: false,
                    edition_nonce: Some(0),
                    uses: None,
                    collection: Some(Collection {
                        verified: true,
                        key: ctx.accounts.collection_mint.key(),
                    }),
                    token_program_version: TokenProgramVersion::Original,
                    token_standard: Some(TokenStandard::NonFungible),
                }
            )
            .invoke_signed(signer_seeds)?;
        Ok(())
    }