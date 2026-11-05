use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    hash::Hash,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signature, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_interface::{
    id as token_program_id,
    instruction::{initialize_mint, mint_to_checked, transfer_checked},
    state::Mint,
};

fn main() -> Result<()> {
    // setup rpc client
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // setup payer
    let payer =
        read_keypair_file("id.json").expect("Failed to find file id.json in the root directory!");

    // setup mint
    let decimals: u8 = 6;

    let latest_blockhash = client.get_latest_blockhash()?;

    // Create Mint
    let (mint, signature_mint) = create_token_mint(&client, &payer, decimals, latest_blockhash)?;
    println!(
        "\nMint Created\n\tAddress: {}\n\tSig: {}",
        mint.pubkey(),
        signature_mint
    );

    // Create sender ATA
    let (sender_address, signature_sender) = create_ata(
        &client,
        &payer,
        &payer.pubkey(),
        &mint.pubkey(),
        latest_blockhash,
    )?;
    println!(
        "\nSender ATA Created\n\tAddress: {}\n\tSig: {}",
        sender_address, signature_sender
    );

    // Mint tokens to Sender
    let signature_mint_tokens = mint_tokens_to_ata(
        &client,
        &payer,
        &mint.pubkey(),
        &sender_address,
        latest_blockhash,
        100,
        decimals,
    )?;
    println!(
        "\nMinted 100 Tokens\n\tAddress: {}\n\tSig: {}",
        sender_address, signature_mint_tokens
    );

    // create recipient ata
    let recipient = Keypair::new();
    let (recipient_address, signature_recipient) = create_ata(
        &client,
        &payer,
        &recipient.pubkey(),
        &mint.pubkey(),
        latest_blockhash,
    )?;
    println!(
        "\nRecipient ATA Created\n\tAddress: {}\n\tSig: {}",
        recipient_address, signature_recipient
    );

    // Transfer Tokens
    let signature_transfer = transfer_tokens_between_ata(
        &client,
        &payer,
        &sender_address,
        &mint.pubkey(),
        &recipient_address,
        latest_blockhash,
        25,
        decimals,
    )?;
    println!("\nTransfered 25 tokens\n\tSig: {}", signature_transfer);
    println!(
        "\tFrom ATA: {} --------> To ATA: {}",
        sender_address, recipient_address
    );

    Ok(())
}

fn create_token_mint(
    client: &RpcClient,
    payer: &Keypair,
    decimals: u8,
    latest_blockhash: Hash,
) -> Result<(Keypair, Signature)> {
    let mint = Keypair::new();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account(
                &payer.pubkey(),
                &mint.pubkey(),
                mint_rent,
                Mint::LEN as u64,
                &token_program_id(),
            ),
            initialize_mint(
                &token_program_id(),
                &mint.pubkey(),
                &payer.pubkey(),
                Some(&payer.pubkey()),
                decimals,
            )?,
        ],
        Some(&payer.pubkey()),
        &[payer, &mint],
        latest_blockhash,
    );
    let signature = client.send_and_confirm_transaction(&transaction)?;

    Ok((mint, signature))
}

fn create_ata(
    client: &RpcClient,
    payer: &Keypair,
    wallet: &Pubkey,
    mint: &Pubkey,
    latest_blockhash: Hash,
) -> Result<(Pubkey, Signature)> {
    let transaction = Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &payer.pubkey(),
            wallet,
            mint,
            &token_program_id(),
        )],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    let address = get_associated_token_address(&payer.pubkey(), mint);

    Ok((address, signature))
}

fn mint_tokens_to_ata(
    client: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    destination_ata: &Pubkey,
    latest_blockhash: Hash,
    amount: u64,
    decimals: u8,
) -> Result<Signature> {
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_checked(
            &token_program_id(),
            mint,
            destination_ata,
            &payer.pubkey(),
            &[],
            amount,
            decimals,
        )?],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    Ok(signature)
}

fn transfer_tokens_between_ata(
    client: &RpcClient,
    payer: &Keypair,
    source_ata: &Pubkey,
    mint: &Pubkey,
    destination_ata: &Pubkey,
    latest_blockhash: Hash,
    amount: u64,
    decimals: u8,
) -> Result<Signature> {
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_checked(
            &token_program_id(),
            source_ata,
            mint,
            destination_ata,
            &payer.pubkey(),
            &[],
            amount,
            decimals,
        )?],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    Ok(signature)
}
