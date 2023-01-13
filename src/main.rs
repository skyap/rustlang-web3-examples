#![allow(unused_imports, dead_code, unused_mut, unused_variables)]

use std::str::FromStr;

use secp256k1::{rand::rngs, PublicKey, SecretKey};
use std::time::{SystemTime, UNIX_EPOCH};
use web3::contract::{Contract, Options};
use web3::types::{Address, TransactionParameters, U256, H160};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn get_nstime() -> u64 {
    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // The correct way to calculate the current time is
    // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
    // But this is faster, and the difference in terms of entropy is
    // negligible (log2(10^9) == 29.9).
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

#[tokio::main]

async fn main() -> web3::Result<()> {

    /* --------------------------------------- connect to http -------------------------------------- */
    // let transport = web3::transports::Http::new(
    //     "https://eth-mainnet.g.alchemy.com/v2/WQRI3y8g2_IavtJ1cZlpTnkiSODZey6O",
    // )?;
    // let transport = web3::transports::WebSocket::new(
    //     "wss://eth-mainnet.g.alchemy.com/v2/WQRI3y8g2_IavtJ1cZlpTnkiSODZey6O",
    // ).await?;

    /* ---------------------------------------- Check balance --------------------------------------- */
    // let web3 = web3::Web3::new(transport);
    
    // let account = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa").unwrap();
    // let balance = web3.eth().balance(account, None).await?;
    // println!("0x00000000219ab540356cBB839Cbe05303d7705Fa: {:?}", balance);
    /* ---------------------------------------- create wallet --------------------------------------- */
    // let secp = secp256k1::Secp256k1::new();
    // let mut rng = rngs::JitterRng::new_with_timer(get_nstime);
    // let (secretkey, publickey) = secp.generate_keypair(&mut rng);
    // println!("{:?},{:?}", secretkey, publickey);
    
    /* ---------------------------------- Connect to ERC20 contract --------------------------------- */
    // let aave_addr = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
    // let token_contract =
    //     Contract::from_json(web3.eth(), aave_addr, include_bytes!("erc20_abi.json")).unwrap();

    //     let token_name: String = token_contract
    //     .query(&"name".to_string(), (), None, Options::default(), None)
    //     .await
    //     .unwrap();
    // println!("{:?}", token_name);

    /* --------------------------------------- private testnet -------------------------------------- */
    // ganache -d
    // let transport = web3::transports::Http::new(
    //     "http:127.0.0.1:8545",
    // )?;
    // let web3 = web3::Web3::new(transport);
    // let mut accounts = web3.eth().accounts().await?;
    // println!("Accounts:{:?}",accounts);
    // for account in accounts{
    //     let balance = web3.eth().balance(account,None).await?;
    //     println!("Balance of {:?}: {}",account,balance);
    // }

    /* -------------------------------- private testnet submit batch -------------------------------- */
    // ganache -d
    // let transport = web3::transports::Http::new(
    //     "http:127.0.0.1:8545",
    // )?;
    // let web3 = web3::Web3::new(web3::transports::Batch::new(transport));    

    // let accounts = web3.eth().accounts();
    // let block = web3.eth().block_number();

    // let result = web3.transport().submit_batch().await?;
    // println!("Result: {:?}",result);

    // println!("Accounts: {:?}",accounts.await?);
    // println!("Block: {:#?}",block.await?);
    
    /* ------------------------------ private testnet sent transaction ------------------------------ */
    // ganache -d
    let transport = web3::transports::Http::new(
        "http:127.0.0.1:8545",
    )?;
    let web3 = web3::Web3::new(transport);  

    let accounts = web3.eth().accounts().await?;

    // let to = Address::from_str("0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1").unwrap();

    let to = accounts[0];

    let prvk = SecretKey::from_str("6cbed15c793ce57650b9877cf6fa156fbef513c4e6134f022a85b1ffdd59b2a1").unwrap();

    let tx_object = TransactionParameters{
        to: Some(to),
        value: U256::exp10(18),
        ..Default::default()
    };
    println!("Balance: {}",web3.eth().balance(to,None).await?);
    let signed = web3.accounts().sign_transaction(tx_object,&prvk).await?;

    let result = web3.eth().send_raw_transaction(signed.raw_transaction).await?;
    println!("Balance: {}",web3.eth().balance(to,None).await?);
    println!("Tx succedded with hash:{}",result);

    Ok(())
}
