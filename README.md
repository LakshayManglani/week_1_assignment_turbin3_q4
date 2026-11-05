# Solana Week 1 Assignment

This repository contains Rust scripts demonstrating core Solana operations on Devnet: creating and transferring SPL tokens, and minting and updating NFTs using Metaplex Core (`mpl-core`).

---

## Tasks Overview

1. **Mint and transfer your own SPL token** (`01_spl_token`)
2. **Mint an NFT using MPL Core** (`02_mint_nft`)
3. **Update the NFT's name and metadata as the update authority** (`03_update_nft`)

---

## Prerequisites

- **Rust & Cargo**: Rust 1.85+ (2024 edition supported)
- **Solana CLI** (recommended): To manage your keypair and request airdrops.
- **Devnet SOL**: You will need some Devnet SOL in your wallet to pay for transactions and rent exemption.

---

## Setup

### 1. Copy your Solana Keypair

Each binary looks for an `id.json` keypair file in the root directory of this project:

```bash
cp ~/.config/solana/id.json ./id.json
```

> **Note:** If you do not already have a keypair generated, you can create one using:
>
> ```bash
> solana-keygen new --outfile ./id.json
> ```

### 2. Fund your Wallet on Devnet

Ensure your wallet has sufficient SOL on Devnet:

```bash
solana airdrop 2 -k ./id.json -u devnet
```

You can check your balance with:

```bash
solana balance -k ./id.json -u devnet
```

---

## Usage

Each task is implemented as a standalone binary in `src/bin/`. You can execute each task using Cargo:

### Task 1: Mint and Transfer an SPL Token

Creates a new token mint, initializes an Associated Token Account (ATA) for your payer wallet, mints tokens to it, creates a recipient account, and transfers tokens.

```bash
cargo run --bin 01_spl_token
```

**What happens:**

- Connects to Solana Devnet.
- Generates and initializes a new Token Mint (6 decimals).
- Creates an Associated Token Account (ATA) for your payer wallet.
- Mints 100 units to your ATA.
- Creates a recipient keypair and associated token account.
- Transfers 25 units to the recipient ATA.
- Outputs transaction signatures and account details.

---

### Task 2: Mint an NFT using MPL Core

Mints a new NFT on Devnet using Metaplex Core (`mpl-core`).

```bash
cargo run --bin 02_mint_nft
```

**What happens:**

- Creates a new MPL Core asset with name `"My Asset"` and initial metadata URI.
- Confirms the transaction on Devnet.
- Prints the transaction signature and the **Asset Address**.

> 💡 **Important:** Copy the printed **Asset Address** (public key) from the output. You will need it for Task 3.

---

### Task 3: Update NFT Name and Metadata

Updates the metadata (name and URI) of an existing MPL Core asset using your wallet as the update authority.

Pass the **Asset Address** obtained from Task 2 as an argument:

```bash
cargo run --bin 03_update_nft -- <ASSET_PUBKEY>
```

**Example:**

```bash
cargo run --bin 03_update_nft -- 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
```

**What happens:**

- Reads the asset public key from command line arguments.
- Calls `UpdateV1Builder` from `mpl-core` to update:
  - **Name:** `"Updated Asset"`
  - **URI:** Updated metadata JSON endpoint
- Verifies and prints the updated asset state on-chain.
