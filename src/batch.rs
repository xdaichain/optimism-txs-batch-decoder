use ethers::types::{ Transaction, TransactionReceipt, U256, U64, H160, Bytes, TxHash };
use ethers::utils::rlp;

use byteorder::{ BigEndian, ByteOrder };

use flate2::write::ZlibDecoder;

use std::io::Write;
use std::str::FromStr;

#[derive(Clone)]
pub struct Batch
{
    _tx: Option<Transaction>,
    _receipt: Option<TransactionReceipt>,
}

impl Batch
{
    pub fn get_tx(&self) -> Transaction
    {
        return self._tx.clone().unwrap();
    }

    pub fn get_receipt(&self) -> TransactionReceipt
    {
        return self._receipt.clone().unwrap();
    }

    pub fn new(tx: Option<Transaction>, receipt: Option<TransactionReceipt>) -> Self
    {
        return Batch { _tx: tx, _receipt: receipt };
    }

    pub fn decode(&self) -> Vec<Transaction>
    {
        let mut txs = Vec::<Transaction>::new();
        let calldata = self._tx.as_ref().unwrap().input.to_vec();
        let mut contexts: [u8; 3] = [0, 0, 0];

        contexts.copy_from_slice(&calldata[12..15]);
        let contexts: usize = BigEndian::read_u24(contexts.as_ref()) as usize;

        let tx_list_index: usize = 15 + 16 * contexts;
        let mut tx_list: Vec<u8> = Vec::<u8>::new();

        tx_list.resize(calldata.len() - tx_list_index, 0);
        tx_list.clone_from_slice(&calldata[tx_list_index..calldata.len()]);

        self.decode_batch(tx_list.as_ref(), txs.as_mut());

        return txs;
    }

    fn decode_batch(&self, tx_list: &Vec<u8>, out: &mut Vec<Transaction>)
    {
        let decompressed_txs = self.decompress_batch(tx_list);

        let mut begin: usize = 0;
        let mut tx_len: usize = BigEndian::read_u24(&decompressed_txs[begin..(begin + 3)]) as usize;

        while begin < decompressed_txs.len()
        {  
            let rlp = ethers::utils::rlp::Rlp::new(&decompressed_txs[(begin + 3)..(begin + 3 + tx_len)]);
            let mut tx: Vec<u8> = Vec::<u8>::new();
            tx.resize(tx_len, 0);
            tx.clone_from_slice(&decompressed_txs[begin..(begin + tx_len)]);

            let _nonce: Vec<u8> = rlp::decode(rlp.at(0).unwrap().as_raw()).unwrap();
            let _gas_price: Vec<u8> = rlp::decode(rlp.at(1).unwrap().as_raw()).unwrap();
            let _gas: Vec<u8> = rlp::decode(rlp.at(2).unwrap().as_raw()).unwrap();
            let _to: Vec<u8> = rlp::decode(rlp.at(3).unwrap().as_raw()).unwrap();
            let _value: Vec<u8> = rlp::decode(rlp.at(4).unwrap().as_raw()).unwrap();
            let _input: Vec<u8> = rlp::decode(rlp.at(5).unwrap().as_raw()).unwrap();
            let _v: Vec<u8> = rlp::decode(rlp.at(6).unwrap().as_raw()).unwrap();
            let _r: Vec<u8> = rlp::decode(rlp.at(7).unwrap().as_raw()).unwrap();
            let _s: Vec<u8> = rlp::decode(rlp.at(8).unwrap().as_raw()).unwrap();
            
            let _nonce: U256 = U256::from_str(Bytes::from(_nonce).to_string().as_str()).unwrap();
            let _gas_price: Option<U256> = Some(U256::from_str(Bytes::from(_gas_price).to_string().as_str()).unwrap());
            let _gas: U256 = U256::from_str(Bytes::from(_gas).to_string().as_str()).unwrap();
            let _to: Option<H160> = Some(H160::from_str(Bytes::from(_to).to_string().as_str()).unwrap());
            let _value: U256 = U256::from_str(Bytes::from(_value).to_string().as_str()).unwrap();
            let _input: Bytes = Bytes::from(_input);
            let _v: U64 = U64::from_str(Bytes::from(_v).to_string().as_str()).unwrap();
            let _r: U256 = U256::from_str(Bytes::from(_r).to_string().as_str()).unwrap();
            let _s: U256 = U256::from_str(Bytes::from(_s).to_string().as_str()).unwrap();

            let mut transaction = Transaction 
            { 
                hash: TxHash::zero(), 
                nonce: _nonce, 
                block_hash: None, 
                block_number: None, 
                transaction_index: None, 
                from: H160::zero(), 
                to: _to, 
                value: _value, 
                gas_price: _gas_price,
                gas: _gas, 
                input: _input, 
                v: _v, 
                r: _r, 
                s: _s, 
                transaction_type: None, 
                access_list: None, 
                max_priority_fee_per_gas: None, 
                max_fee_per_gas: None, 
                chain_id: None 
            };

            transaction.from = transaction.recover_from().unwrap(); 
            transaction.hash = transaction.hash();
            out.push(transaction);



            begin = begin + 3 + tx_len;
            if begin < decompressed_txs.len()
            {
                tx_len = BigEndian::read_u24(&decompressed_txs[begin..(begin + 3)]) as usize;
            }
        }
    }

    fn decompress_batch(&self, tx_list: &Vec<u8>) -> Vec<u8>
    {
        let mut decoded_txs = Vec::<u8>::new();
        let mut decoder = ZlibDecoder::new(decoded_txs);
    
        decoder.write_all(tx_list).expect("Zlib decoding error.");
        decoded_txs = decoder.finish().unwrap();
    
        return decoded_txs;
    }
}
