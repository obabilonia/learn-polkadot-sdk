use subxt::{OnlineClient, PolkadotConfig, utils::{AccountId32, MultiAddress}};
use subxt::dynamic::{At, Value};
use subxt_signer::sr25519::dev;  // Usa as chaves de Alice e Bob

#[subxt::subxt(runtime_metadata_path = "local_metadata.scale")]  // Usa os metadados do nó local
pub mod local {}

//use local::balances::calls::types::TransferKeepAlive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Conectar ao nó local
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9933").await?;

    // Criar as chaves de Alice e Bob
    let alice_key = dev::alice();
    let bob_key = dev::bob();

    // Obter os endereços
    let alice_address = alice_key.public_key().to_account_id();
    let bob_address = bob_key.public_key().to_account_id();

    println!("Alice: {}", alice_address);
    println!("Bob: {}", bob_address);

    // Definir o valor a ser transferido (exemplo: 1_000_000_000 unidades)
    let amount = 10;

    let transfer = local::tx().balances().transfer_allow_death(bob_key.public_key().into(), amount);
    // Montar a transação de transferência

    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&transfer, &alice_key)
        .await?
        .wait_for_finalized_success()
        .await?;

    // Find a Transfer event and print it.
    let transfer_event = events.find_first::<local::balances::events::Transfer>()?;
    if let Some(event) = transfer_event {
        println!("Balance transfer success: {event:?}");
    }

    println!("Transação enviada com sucesso!");










    // Criar transação dinâmica usando `Value`
//    let transfer_tx = subxt::dynamic::tx(
//        "Balances",
//        "transfer_allow_death",
//        vec![Value::from_bytes(bob_address), Value::u128(amount)],
//    );

//    println!("passou aqui");

    // Enviar a transação de transferência de Alice para Bob
//    let call = local::balances::call::transfer {
//        dest: bob_key.public_key(),
//        value: amount,
//    };

    // Cria a transação assinada por Alice
//    let extrinsic: Extrinsic<PolkadotConfig> = client.tx().call(&transfer_tx).sign(&alice_key).await?;

    // Enviar a transação para o nó
//    let hash = client.rpc().send_extrinsic(&extrinsic).await?;
//    println!("Transação enviada com hash: {:?}", hash);


    // Enviar a transação
//    let tx_progress = api.tx().sign_and_submit_then_watch_default(&transfer_tx, &alice_key).await?;

    // Criar uma variável para consumir o fluxo de eventos
//    let mut tx_progress = tx_progress;

    // Monitorar eventos da transação
//    while let Some(event) = tx_progress.next().await {
//        match event {
//            Ok(e) => println!("Evento da transação: {:?}", e),
//            Err(e) => eprintln!("Erro no evento: {:?}", e),
//        }
//    }

    println!("Transação concluída com sucesso!");


    Ok(())
}


