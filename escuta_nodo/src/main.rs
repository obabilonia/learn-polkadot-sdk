use subxt::{OnlineClient, PolkadotConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9933").await?;
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;
    
    while let Some(block) = blocks_sub.next().await {
        let new_block = block?; 
        let block_number = new_block.header().number;
        
        println!("New block #{block_number} created! ✨");

        let extrinsics = new_block.extrinsics().await?;

        for extrinsic in extrinsics.iter() {
            match extrinsic {
                Ok(extrinsic_details) => {
                    let idx = extrinsic_details.index();
                    // here we get all the events for the current extrinsic we're looping over :D 
                    let events = extrinsic_details.events().await?;

                    println!("    Extrinsic #{idx}:");
                    println!("      Events:");
                
                    for evt in events.iter() {
                        let evt = evt?;
                
                        let pallet_name = evt.pallet_name();
                        // I've used variant_name here, can you figure out what it is? 
                        let event_name = evt.variant_name();
                
                        println!("        {pallet_name}_{event_name}");
                    }
                },
                Err(e) => {
                    println!("Encountered an error: {}", e);
                },
            }
        }
    }

    Ok(())
}

