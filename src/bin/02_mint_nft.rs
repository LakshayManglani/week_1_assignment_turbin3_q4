use anyhow::Result;
use mpl_core::instructions::CreateV1Builder;
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

    let transaction = Transaction::new_signed_with_payer(
        &[CreateV1Builder::new()
            .asset(asset.pubkey())
            .payer(payer.pubkey())
            .name("My Asset".into())
            .uri("https://4r3jajvbhenppwwxyuxtagvbz5vkke43ucecsbekp3emjnrefloa.devnet-1.datasprite-cdn.com/GNpp9YzTvKm3QyRzkMBZt3KVNmhPrJTdCJvucn2LCWRH/".into())
            .instruction()],
        Some(&payer.pubkey()),
        &[&asset, &payer],
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;

    println!(
        "\nMint NFT\n\tAddress: {}\n\tSig: {}",
        asset.pubkey(),
        signature
    );

    Ok(())
}
