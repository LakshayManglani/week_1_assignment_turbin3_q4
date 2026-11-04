use std::{env, str::FromStr};

use anyhow::{Context, Result};
use mpl_core::{accounts::BaseAssetV1, instructions::UpdateV1Builder};
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    pubkey::Pubkey, signature::read_keypair_file, signer::Signer, transaction::Transaction,
};

fn main() -> Result<()> {
    // setup rpc client
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let latest_blockhash = client.get_latest_blockhash()?;

    // setup payer
    let payer =
        read_keypair_file("id.json").expect("Failed to find file id.json in the root directory!");

    // setup asset
    let args: Vec<String> = env::args().collect();

    let asset = Pubkey::from_str(&args[1])
        .context("Usage: cargo run --bin 03_update_nft -- <ASSET_PUBKEY>")?;

    let new_name = "Updated Asset";
    let new_uri = "https://rvvguosnyhmjkrwy35m32gry673ppzchiv6bq4hizkztgfdhgc3q.devnet-1.datasprite-cdn.com/AX2bm55bJ38PZaX4jJFTxmSyj5kQicvBf2bBe1zw74J2/";

    let transaction_mint_asset = Transaction::new_signed_with_payer(
        &[UpdateV1Builder::new()
            .asset(asset)
            .payer(payer.pubkey())
            .authority(Some(payer.pubkey()))
            .new_name(new_name.into())
            .new_uri(new_uri.into())
            .instruction()],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let transaction_signature_update_asset =
        client.send_and_confirm_transaction(&transaction_mint_asset)?;
    let asset_account = client.get_account(&asset)?;
    let asset_data = BaseAssetV1::from_bytes(&asset_account.data)?;

    println!(
        "\nTransaction Signature: {}",
        transaction_signature_update_asset
    );
    println!("Asset Address: {}", asset);
    println!("Asset Account: {:#?}", asset_data);

    Ok(())
}
