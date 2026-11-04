use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};
use spl_token_interface::{
    id as token_program_id,
    instruction::{initialize_mint, mint_to_checked},
    state::{Account, Mint},
};

fn main() -> Result<()> {
    // setup rpc client
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // setup payer
    let payer =
        read_keypair_file("id.json").expect("Failed to find file id.json in the root directory!");

    // setup mint
    let mint = Keypair::new();
    let mint_rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let decimals: u8 = 6;

    let latest_blockhash = client.get_latest_blockhash()?;

    let transaction_create_and_init_mint_account = Transaction::new_signed_with_payer(
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
        &[&payer, &mint],
        latest_blockhash,
    );
    let transaction_signature_mint =
        client.send_and_confirm_transaction(&transaction_create_and_init_mint_account)?;

    let mint_account = client.get_account(&mint.pubkey())?;
    let mint_data = Mint::unpack(&mint_account.data)?;

    println!("\nTransaction Signature: {}", transaction_signature_mint);
    println!("Mint Address: {}", mint.pubkey());
    println!("Mint Account: {:#?}", mint_data);

    // setup sender ATA
    let transaction_create_sender_ata = Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &payer.pubkey(),
            &payer.pubkey(),
            &mint.pubkey(),
            &token_program_id(),
        )],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );
    let transaction_signature_sender =
        client.send_and_confirm_transaction(&transaction_create_sender_ata)?;
    let sender_account_address = get_associated_token_address(&payer.pubkey(), &mint.pubkey());
    let sender_token_account = client.get_account(&sender_account_address)?;
    let sender_token_data = Account::unpack(&sender_token_account.data)?;

    println!("\nTransaction Signature: {}", transaction_signature_sender);
    println!("Sender Address: {}", sender_account_address);
    println!("Sender Account: {:#?}", sender_token_data);

    // Mint tokens
    let mint_amount = 100;

    let transaction_mint_token = Transaction::new_signed_with_payer(
        &[mint_to_checked(
            &token_program_id(),
            &mint.pubkey(),
            &sender_account_address,
            &payer.pubkey(),
            &[],
            mint_amount,
            decimals,
        )?],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );
    let transaction_signature_mint_token =
        client.send_and_confirm_transaction(&transaction_mint_token)?;
    let mint_account = client.get_account(&mint.pubkey())?;
    let mint_data = Mint::unpack(&mint_account.data)?;
    let sender_token_account = client.get_account(&sender_account_address)?;
    let sender_token_data = Account::unpack(&sender_token_account.data)?;
    println!("\n Mint Mined");
    println!(
        "\nTransaction Signature: {}",
        transaction_signature_mint_token
    );
    println!("Mint Account: {:#?}", mint_data);
    println!("Sender Account: {:#?}", sender_token_data);

    Ok(())
}
