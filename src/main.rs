use std::hash::Hash;
use alloy_primitives::{Address, address};
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::{H256, Bytes};
use secp256k1::SecretKey;
use std::str::FromStr;

use mini_redis::Result;
use web3::contract::tokens::Tokenizable;

const COUNTER_CONTRACT_ADDRESS: Address = address!("b4B46bdAA835F8E4b4d8e208B6559cD267851051");
const STATE_ROOT_CONTRACT_ADDRESS: &str = "0xb4B46bdAA835F8E4b4d8e208B6559cD267851051";
const L1_RPC_ADDREESS: &str = "http://localhost:32773";
const PREFUNDED_SECRET: &str = "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31";

#[derive(Debug)]
pub struct StateRootContract(Contract<Http>);

impl StateRootContract {
    pub async fn new(web3: &web3::Web3<Http>, address: String) -> Self {
        let address = web3::types::Address::from_str(&address).unwrap();
        let contract =
            Contract::from_json(web3.eth(), address, include_bytes!("./state_root_abi.json")).unwrap();
        StateRootContract(contract)
    }

    pub async fn update_state_root(&self, account: &SecretKey, state_root: Bytes) -> H256 {
        let result =  self
            .0
            .signed_call(
                "changeStateRoot",
                state_root,
                Options {
                    gas: Some(5_000_000.into()),
                    ..Default::default()
                },
                SecretKeyRef::new(account),
            )
            .await;
        match &result {
            Ok(r) => println!("tx_id: {}", r),
            Err(e) => println!("err: {}", e)
        }
        result.unwrap()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world! I live at {}", COUNTER_CONTRACT_ADDRESS);
    let transport = Http::new(L1_RPC_ADDREESS).unwrap();
    let web3 = web3::Web3::new(transport);

    let state_root_contract = StateRootContract::new(
        &web3,
        STATE_ROOT_CONTRACT_ADDRESS.to_string(),
    ).await;

    let wallet = SecretKey::from_str(PREFUNDED_SECRET).unwrap();

    let tx_id = state_root_contract
        .update_state_root(
            &wallet, Bytes::from(vec![1, 2, 3, 4])
        );
    println!("I notifed L1 with new state root. txId = [{:#x}]", tx_id.await);

    Ok(())
}
