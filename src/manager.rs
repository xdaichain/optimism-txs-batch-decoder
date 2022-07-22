use crate::transaction_provider::TransactionProvider;
use crate::batch::Batch;

use ethers::types::{ Transaction,  TransactionReceipt, H256 };

pub struct TransactionManager
{
    pub _provider: TransactionProvider,
    _batch: Option<Batch>
}

impl TransactionManager
{
    pub async fn new() -> Self
    {
        let provider = TransactionProvider::new().await;

        return TransactionManager { _provider: provider, _batch: None };
    }

    pub async fn get_batch(&mut self, tx: H256) -> &Self
    {
        self._batch = Some(self._provider.get_batch(tx).await.unwrap());
        
        self
    }

    pub fn decode_batch(&self) -> Vec<Transaction>
    {
        return self._batch.as_ref().unwrap().decode();
    }

    pub fn get_receipt(&self) -> TransactionReceipt
    {
        return self._batch.as_ref().unwrap().get_receipt();
    }

    pub fn get_tx(&self) -> Transaction
    {
        return self._batch.as_ref().unwrap().get_tx();
    }
}
