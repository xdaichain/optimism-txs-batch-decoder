use ethers::prelude::Middleware;
use ethers::providers::{ Provider, ProviderError, Http };
use ethers::types::H256;

use std::env;

use crate::batch::Batch;

pub struct TransactionProvider
{
    pub _optimism: Provider<Http>,
    pub _mainnet: Provider<Http>
}

impl TransactionProvider
{
    pub async fn new() -> Self
    {
        dotenv::dotenv().ok();

        let mainnet: Provider<Http> = Provider::<Http>::try_from(&env::var("MAINNET_URL").unwrap())
                                        .expect("could not instantiate HTTP MAINNET Provider");
        let optimism: Provider<Http> = Provider::<Http>::try_from(&env::var("OPTIMISM_URL").unwrap())
                                        .expect("could not instantiate HTTP OPTIMISM Provider");

        return Self { _mainnet: mainnet, _optimism: optimism };
    }

    pub async fn get_batch(&self, hash: H256) -> Result<Batch, ProviderError>
    {
        let tx = self._mainnet.get_transaction(hash).await?;
        let receipt = self._mainnet.get_transaction_receipt(hash).await?;

        return Ok(Batch::new(tx, receipt));
    }
}
