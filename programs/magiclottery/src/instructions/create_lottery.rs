use anchor_lang::prelude::*;
use crate::global_accounts::central_account::CentralStateData;
use crate::errors::errors::MyError;
use crate::CENTRAL_AUTHORITY_SEED;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
        mpl_token_metadata::types::{CollectionDetails, DataV2},
    },
    token::{Mint, mint_to, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{ MasterEdition, Metadata as MetadataAccount };

#[derive(Accounts)]
pub struct CreateLottery<'info> {
    #[account(mut, signer)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [CENTRAL_AUTHORITY_SEED],
        bump
    )]
    pub central_authority: Account<'info, CentralStateData>,

    #[account(
        constraint = authorizer_wallet.key() == central_authority.authorizer_wallet @ MyError::UnauthorizedWallet,
        constraint = payer.key() != authorizer_wallet.key() @ MyError::PayerMustBeClient
    )]
    pub authorizer_wallet: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = central_authority.key(),
        mint::freeze_authority = central_authority.key(),
    )]
    pub collection_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = collection_mint,
        associated_token::authority = central_authority
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// CHECK - address
    #[account(
        mut,
        address = MetadataAccount::find_pda(&collection_mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: address
    #[account(
        mut,
        address = MasterEdition::find_pda(&collection_mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}    
       
/// Creates a new lottery by minting an NFT collection
///
/// This function initializes a new lottery by creating an NFT collection.
/// It mints a single NFT, creates metadata and master edition accounts,
/// and sets up the collection for future ticket minting.
///
/// # Arguments
///
/// * `ctx` - The context struct containing the accounts required for the operation
/// * `name` - The name of the lottery collection
/// * `symbol` - The symbol for the lottery collection
/// * `uri` - The URI for the lottery collection metadata
///
/// # Returns
///
/// * `Result<()>` - Returns Ok(()) if the operation is successful, otherwise returns an error
pub fn create_lottery<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateLottery<'info>>,
        name: String,
        symbol: String,
        uri: String,
) -> Result<()> {
        msg!("initializing collection");
        
        // Prepare the signer seeds for CPI calls
        let bump_seed = [ctx.bumps.central_authority];
        let signer_seeds: &[&[&[u8]]] = &[&[
            CENTRAL_AUTHORITY_SEED,
            &bump_seed,
        ]];
        
        // Create mint account and mint one token
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.collection_mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.central_authority.to_account_info(),
            },
            signer_seeds,
        );

        mint_to(cpi_context, 1)?;

        // Create metadata account
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                mint_authority: ctx.accounts.central_authority.to_account_info(),
                update_authority: ctx.accounts.central_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        );

        let data_v2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(
            cpi_context,
            data_v2,
            true,
            true,
            Some(CollectionDetails::V1 { size: 1 }),
        )?;

        // Create master edition account
        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: ctx.accounts.master_edition_account.to_account_info(),
                mint: ctx.accounts.collection_mint.to_account_info(),
                update_authority: ctx.accounts.central_authority.to_account_info(),
                mint_authority: ctx.accounts.central_authority.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        );

        create_master_edition_v3(cpi_context, Some(0))?;

        // Store the collection address in the central authority
        //ctx.accounts.central_authority.collection_address = ctx.accounts.mint.key();
        Ok(())
    }