use subxt::{OnlineClient, PolkadotConfig};
use subxt::dynamic::{At, Value};
use subxt_signer::sr25519::dev;

#[subxt::subxt(runtime_metadata_path = "local_metadata.scale")]  // Usa os metadados do nó local
pub mod local {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9933").await?;


    // Criar a chave de Alice usando subxt_signer::sr25519::dev
    let alice_key = dev::alice();

    // Obter o endereço de Alice
    let alice_address = alice_key.public_key().to_account_id();

    // Exibir o endereço de Alice
    println!("Endereço de Alice: {}", alice_address.to_string());

    // Consultar o saldo de Alice
    let balance_query = local::storage().system().account(alice_address);
    let account_info = api.storage().at_latest().await?.fetch(&balance_query).await?;

    // Exibir o saldo
    if let Some(info) = account_info {
        println!("Saldo de Alice: {}", info.data.free);
    } else {
        println!("Conta não encontrada!");
    }

    Ok(())
}
