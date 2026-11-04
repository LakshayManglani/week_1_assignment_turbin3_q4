use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_token_interface::{id as token_program_id, instruction::initialize_mint, state::Mint};

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

    let create_mint_account_ix = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &token_program_id(),
    );

    let init_mint_account_ix = initialize_mint(
        &token_program_id(),
        &mint.pubkey(),
        &payer.pubkey(),
        Some(&payer.pubkey()),
        decimals,
    )?;

    let latest_blockhash = client.get_latest_blockhash()?;

    let tx_create_mint = Transaction::new_signed_with_payer(
        &[create_mint_account_ix, init_mint_account_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        latest_blockhash,
    );

    let transaction_signature = client.send_and_confirm_transaction(&tx_create_mint)?;
    let mint_account = client.get_account(&mint.pubkey())?;
    let mint_data = Mint::unpack(&mint_account.data)?;

    println!("Mint Address: {}", mint.pubkey());
    println!("Mint Account: {:#?}", mint_data);
    println!("\nTransaction Signature: {}", transaction_signature);

    Ok(())
}
