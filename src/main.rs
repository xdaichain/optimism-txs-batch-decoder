pub mod batch;
pub mod transaction_provider;
pub mod manager;

use manager::TransactionManager;

use std::str::FromStr;

use ethers::types::{ H256, U256 };

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()>
{
    let mut manager = TransactionManager::new().await;

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

    let mut paid_by_users: f64 = 0.0;

    for i in 0..decoded_txs.len()
    {
        let receipt: serde_json::Value = manager._provider._optimism
                                        .request("eth_getTransactionReceipt", [decoded_txs[i].hash]).await?;
        let fee = receipt.get("l1Fee").unwrap().as_str().unwrap();

        let fee = U256::from_str_radix(fee.trim_start_matches("0x"), 16)?;
        let fee = (fee.as_u128() as f64) / ((10 as u128).pow(18) as f64);

        paid_by_users += fee;
    }

    let receipt = manager.get_receipt();
    let gas = receipt.gas_used.unwrap();
    let gas_price = manager.get_tx().gas_price.unwrap();

    let paid_by_optimism: f64 = (gas.as_u128() as f64) * (gas_price.as_u128() as f64) / ((10 as u128).pow(18) as f64);
    let diff: f64 = (paid_by_users / paid_by_optimism - 1.0) * 100.0;

    println!("Total amount paid by users: {}", paid_by_users);
    println!("Batch submission on L1 fee: {}", paid_by_optimism);
    println!("Diff: +{:.2}%", diff);

    return Ok(());
}
