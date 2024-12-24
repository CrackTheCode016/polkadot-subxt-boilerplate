use subxt::{ PolkadotConfig, OnlineClient, SubstrateConfig };

#[subxt::subxt(runtime_metadata_path = "./metadata/polkadot_relay_chain.scale")]
pub mod polkadot {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<PolkadotConfig>::from_url(
        "wss://polkadot-rpc.dwellir.com"
    ).await?;

    // Build a balance transfer extrinsic.
    let runtime_upgrade_query = polkadot::storage().system().last_runtime_upgrade();

    // Perform the query
    let result = api.storage().at_latest().await?.fetch(&runtime_upgrade_query).await?;

    // Parse and display the result
    if let Some(last_upgrade) = result {
        println!("Spec Name: {:?}", last_upgrade.spec_name);
        println!("Spec Version: {:?}", last_upgrade.spec_version);
    }

    Ok(())
}
