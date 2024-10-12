//! Magic Lottery Program: Implements a lottery system on Solana blockchain.

use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};
mod errors;
mod clones;
mod global_accounts;
mod instructions;
use crate::instructions::*;

pub const CENTRAL_AUTHORITY_SEED: &[u8] = b"B_central_authority";
declare_id!("ML1sC4hP2dtyNHSL4QP4KUkqjyqL3cFJWHLmQZdZ3yN");

/// Main program module for Magic Lottery
#[program]
pub mod magic_lottery {
    use super::*;

    /// Initialize central authority
    pub fn initialize_central_authority(ctx: Context<InitializeCentralAuthority>, authorizer_wallet: Pubkey) -> Result<()> {
        initialize::initialize_central_authority(ctx, authorizer_wallet)
    }

    /// Create a new lottery
    pub fn create_lottery<'info>(ctx: Context<'_, '_, '_, 'info, CreateLottery<'info>>, name: String, symbol: String, uri: String) -> Result<()> { 
        create_lottery::create_lottery(ctx, name, symbol, uri)
    }   

    /// Buy a lottery ticket
    pub fn buy_ticket<'info>(ctx: Context<'_, '_, '_, 'info, BuyTicket<'info>>, name: String, symbol: String, uri: String, seller_fee_basis_points: u16, payment_amount: u64) -> Result<()> { 
        buy_ticket::buy_ticket(ctx, name, symbol, uri, seller_fee_basis_points, payment_amount)
    } 
    
    /// Dissolve a ticket
    pub fn disolve_ticket<'info>(ctx: Context<'_, '_, '_, 'info, DisolveTicket<'info>>, root: [u8; 32], data_hash: [u8; 32], creator_hash: [u8; 32], nonce: u64, index: u32) -> Result<()> { 
        disolve_ticket::disolve_ticket(ctx, root, data_hash, creator_hash, nonce, index)
    } 

    /// Create a lottery tree
    pub fn create_tree<'info>(ctx: Context<'_, '_, '_, 'info, CreateLotteryTree<'info>>, max_depth: u32, max_buffer_size: u32) -> Result<()> { 
        create_tree::create_lottery_tree(ctx, max_depth, max_buffer_size)
    } 
}

