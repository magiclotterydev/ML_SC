use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    #[msg("No signer")]
    NoSigner,
    #[msg("Unsupported tree account size")]
    UnsupportedTreeAccountSize,
    #[msg("Invalid merkle tree")]
    InvalidMerkleTree,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Central authority has already been initialized")]
    AlreadyInitialized,
    #[msg("Only the upgrade authority can initialize the central authority")]
    UnauthorizedUpgradeAuthority,
    #[msg("Invalid program account")]
    InvalidProgramAccount,
    #[msg("Unauthorized wallet")]
    UnauthorizedWallet,
    #[msg("Payer must be the client, not the authorizer wallet")]
    PayerMustBeClient,
    #[msg("Invalid merkle tree address")]
    InvalidMerkleTreeAddress,
    #[msg("Invalid collection address")]
    InvalidCollectionAddress,
}