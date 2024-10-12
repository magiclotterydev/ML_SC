use anchor_lang::prelude::*;
#[account]
pub struct CentralStateData {
    pub is_initialized: bool,
    pub authorizer_wallet: Pubkey,
}

impl CentralStateData {
    pub const MAX_SIZE: usize = 32 * 3;
}