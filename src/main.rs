pub mod batch;
pub mod transaction_provider;
pub mod manager;

use std::str::FromStr;

use ethers::types::{ H256, U256 };

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()>
{
    let mut manager = manager::TransactionManager::new().await;

    let mut batch_transaction_hash: String = String::new();
    
    println!("Please, write correct transaction hash of transaction on L1: ");

    match std::io::stdin().read_line(&mut batch_transaction_hash)
    {
        Ok(_) => {}
        Err(error) => 
        {
            println!("error while trying to read from console: {}", error);
            //This thing shouldn't return Ok(()) but for now Ok.
            return Ok(());
        }
    }
    
    let batch_transaction_hash: H256 = H256::from_str(batch_transaction_hash.as_str())?;

    manager.get_batch(batch_transaction_hash).await;
    let decoded_txs = manager.decode_batch();

    println!("Number of obtained transactions: {}", decoded_txs.len());

    let mut gas_used_by_users: U256 = U256::zero();

    for i in 0..decoded_txs.len()
    {
        
        let gas = batch::calc_calldata_gas_cost(&decoded_txs[i]);

        gas_used_by_users += gas;
    }

    let receipt = manager.get_receipt();
    let gas_used_by_sequencer = receipt.gas_used.unwrap();
    
    let diff = ((gas_used_by_users.as_u128() as f64) / (gas_used_by_sequencer.as_u128() as f64) - 1.0) * 100.0;

    println!("Gas used by users: {}", gas_used_by_users);
    println!("Gas used by sequencer: {}", gas_used_by_sequencer);
    println!("Diff: +{:.2}%", diff);

    return Ok(());
}
