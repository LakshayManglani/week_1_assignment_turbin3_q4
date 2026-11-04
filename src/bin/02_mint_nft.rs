use anyhow::Result;
use mpl_core::{accounts::BaseAssetV1, instructions::CreateV1Builder};
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    signature::{Keypair, read_keypair_file},
    signer::Signer,
    transaction::Transaction,
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
    let asset = Keypair::new();

    let transaction_mint_asset = Transaction::new_signed_with_payer(
        &[CreateV1Builder::new()
            .asset(asset.pubkey())
            .payer(payer.pubkey())
            .name("My Asset".into())
            .uri("https://example.com/my-asset.json".into())
            .instruction()],
        Some(&payer.pubkey()),
        &[&asset, &payer],
        latest_blockhash,
    );

    let transaction_signature_mint_asset =
        client.send_and_confirm_transaction(&transaction_mint_asset)?;
    let asset_account = client.get_account(&asset.pubkey())?;
    let asset_data = BaseAssetV1::from_bytes(&asset_account.data)?;

    println!(
        "\nTransaction Signature: {}",
        transaction_signature_mint_asset
    );
    println!("Asset Address: {}", asset.pubkey());
    println!("Asset Account: {:#?}", asset_data);

    Ok(())
}
