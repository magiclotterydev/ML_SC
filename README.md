# Magic Lottery Smart Contract

🌐 https://magic-lottery.fun/

📝 https://github.com/magiclottery/ML_SC

📘 https://magic-lottery.gitbook.io/magic-lottery-docs

This Solana smart contract implements a lottery system using compressed NFTs (cNFTs) as tickets. The contract leverages the Metaplex Bubblegum program for handling compressed NFTs and includes functionality for creating and managing lotteries.

## Key Features

- Central authority management
- Lottery creation with collection NFTs
- Merkle tree initialization for compressed NFTs
- Ticket purchasing (minting cNFTs)
- Ticket dissolution (burning cNFTs)

## Contract Functions

### 1. initialize_central_authority

Initializes the central authority that manages the lottery. This function must be called before any other instructions.

**Parameters:**

- `authorizer_wallet`: Public key of the wallet authorized to manage the lottery

### 2. create_lottery

Creates a new lottery by initializing the collection master NFT that stores the metadata for the lottery.

**Parameters:**

- `name`: Name of the lottery collection
- `symbol`: Symbol for the lottery collection
- `uri`: URI for the lottery collection metadata

### 3. create_lottery_tree

Initializes the Merkle tree that will be used to store the lottery cNFTs (tickets).

**Parameters:**

- `max_depth`: Maximum depth of the Merkle tree
- `max_buffer_size`: Maximum buffer size for the Merkle tree
- `required_tree_account_size`: Required size of the tree account

### 4. buy_ticket

Allows a user to purchase a ticket for the lottery by minting a cNFT.

**Parameters:**

- `name`: Name of the ticket NFT
- `symbol`: Symbol of the ticket NFT
- `uri`: URI for the ticket NFT metadata
- `seller_fee_basis_points`: Seller fee in basis points

### 5. disolve_ticket

Allows a user to dissolve (burn) a ticket and potentially withdraw their stake from the lottery pool.

**Parameters:**

- `root`: Merkle root
- `data_hash`: Hash of the NFT data
- `creator_hash`: Hash of the NFT creator
- `nonce`: Nonce value
- `index`: Index of the leaf in the Merkle tree

## Account Structures

The contract defines several account structures for managing the lottery state and operations:

- `CentralStateData`: Stores the central authority state
- `CreateLottery`: Accounts required for creating a lottery
- `CreateLotteryTree`: Accounts required for creating a lottery Merkle tree
- `BuyTicket`: Accounts required for purchasing a ticket
- `DisolveTicket`: Accounts required for dissolving a ticket

## Error Handling

The contract includes custom error types to handle various failure scenarios, such as unauthorized access, invalid accounts, and initialization issues.

## Dependencies

This contract relies on several external programs and libraries:

- Anchor Framework
- Metaplex Bubblegum Program
- Metaplex Token Metadata Program
- SPL Token Program
- SPL Associated Token Account Program
- SPL Account Compression Program

## Security Considerations

- The contract includes checks to ensure that only authorized wallets can perform certain actions.
- Proper validation is implemented to verify account ownership and relationships.
- Note that in a production environment, additional security measures and thorough auditing would be necessary.
