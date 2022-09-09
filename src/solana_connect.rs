use js_sys;
use solana_client_wasm::{
    ClientError,
    solana_sdk::signature::{Keypair, Signer},
    WasmClient,
};

pub async fn create_solana_connection() -> Result<u64, ClientError> {
    let client = WasmClient::new("https://api.devnet.solana.com");

    let pubkey = Keypair::new().pubkey();

    let balance = client.get_balance(&pubkey).await?;

    Ok(balance)
}
