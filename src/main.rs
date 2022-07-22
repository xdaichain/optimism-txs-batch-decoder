pub mod batch;
pub mod manager;
pub mod transaction_provider;

use std::str::FromStr;

use ethers::types::{H256, U256};

use eyre::Result;

fn print_info(
    batch_tx_hash: H256,
    num_batch_txs: usize,
    gas_used_by_users: U256,
    gas_used_by_sequencer: U256,
) {
    println!("–––––––––––––––––––––––––––––––––––––");
    println!("Batch tx hash: {}", batch_tx_hash);
    println!("Number of transactions in batch: {}", num_batch_txs);
    println!("Gas used by users: {}", gas_used_by_users);
    println!("Gas used by sequencer: {}", gas_used_by_sequencer);
    let diff = ((gas_used_by_users.as_u128() as f64) / (gas_used_by_sequencer.as_u128() as f64)
        - 1.0)
        * 100.0;
    println!("Diff: +{:.2}%", diff);
    println!("–––––––––––––––––––––––––––––––––––––");
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = manager::TransactionManager::new().await;

    // read the batch transaction hash from the command line
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app

    // handle each batch transaction hash passed to the app
    for batch_transaction_hash in args.iter().skip(1) {
        // decode the batch transaction hash
        let batch_transaction_hash_h256: H256 = H256::from_str(batch_transaction_hash.as_str())?;

        manager.get_batch(batch_transaction_hash_h256).await;
        let decoded_txs = manager.decode_batch();

        let mut gas_used_by_users: U256 = U256::zero();

        for decoded_tx in &decoded_txs {
            let gas = batch::calc_calldata_gas_cost(decoded_tx);

            gas_used_by_users += gas;
        }

        let receipt = manager.get_receipt();
        let gas_used_by_sequencer = receipt.gas_used.unwrap();

        print_info(
            batch_transaction_hash_h256,
            decoded_txs.len(),
            gas_used_by_users,
            gas_used_by_sequencer,
        );
    }

    Ok(())
}
