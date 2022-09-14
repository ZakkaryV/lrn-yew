use js_sys;
use solana_client_wasm::{
    ClientError,
    solana_sdk::signature::{Keypair, Signer},
    WasmClient,
};
use yew::{classes, html, props, Children, Component, Context, Html, NodeRef, Properties, function_component, use_state_eq, Callback};


#[derive(Clone, Properties, PartialEq)]
pub struct SolanaConnectButtonProps {
    connected: bool,
    set_connected: Callback<bool>
}

// #[styled_component(SolanaConnectionButton)]
#[function_component(SolanaConnectButton)]
pub fn solana_connect_button(props: &SolanaConnectButtonProps) -> Html {
    if !props.connected {
        return html! { <button>{ "Connect to Solana" }</button> };
    };

    html! {
        <div>{ "Not connected to Solana mainnet." }</div> 
    }
} 


pub async fn create_solana_connection() -> Result<u64, ClientError> {
    let client = WasmClient::new("https://api.devnet.solana.com");

    let pubkey = Keypair::new().pubkey();

    let balance = client.get_balance(&pubkey).await?;

    Ok(balance)
}

#[derive(Clone, PartialEq, Properties)]
pub struct SolanaConnectionProviderProps;

#[function_component(SolanaConnectionProvider)]
pub fn solana_connection_provider(props: &SolanaConnectionProviderProps) -> Html {
   let state = use_state_eq(|| false);
   let set_state = {
       let state = state.clone();
       Callback::from(move |_| state.set(true))
   };

   return html! {
       <div>
           <SolanaConnectButton connected={*state} set_connected={&set_state} /> 
       </div>
   } 
}
