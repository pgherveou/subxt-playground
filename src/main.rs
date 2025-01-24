use std::time::Duration;
use subxt::config::PolkadotConfig;
use subxt::utils::H256;
use subxt::{
    backend::rpc::reconnecting_rpc_client::{
        ExponentialBackoff, RpcClient as ReconnectingRpcClient,
    },
    OnlineClient,
};

//  subxt metadata --url wss://westend-asset-hub-rpc.polkadot.io > revive_chain.metadata
#[subxt::subxt(runtime_metadata_path = "revive_chain.metadata")]
mod subxt_client {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let rpc_client = ReconnectingRpcClient::builder()
        .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
        .build("wss://westend-asset-hub-rpc.polkadot.io".to_string())
        .await?;

    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;

    let hash = H256::from(hex_literal::hex!(
        "67f9723393ef76214df0118c34bbbd3dbebc8ed46a10973a8c969d48fe7598c9"
    ));

    log::info!("Calling block_gas_limit");
    let runtime_api = api.runtime_api().at(hash);
    let payload = subxt_client::apis().revive_api().block_gas_limit();
    let gas_limit = runtime_api.call(payload).await?;
    log::info!("gas limit result: {gas_limit:?}");
    Ok(())
}
