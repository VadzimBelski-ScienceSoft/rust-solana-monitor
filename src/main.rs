
use bip39::{Language, Mnemonic, MnemonicType, Seed};

use solana_sdk::{
    signature::{keypair_from_seed_and_derivation_path, Signer},
    derivation_path::{DerivationPath}, signer::keypair
};


use solana_client::rpc_client::RpcClient;

use std::{time::Duration, sync::Arc};
use ticker::Ticker;


fn main() {
    
    get_address();

    let ticker = Ticker::new(0.., Duration::from_secs(5));

    let rpc_client = RpcClient::new("https://api.devnet.solana.com");

    let latest_block = rpc_client.get_latest_blockhash().unwrap();
    println!("block count: {}", latest_block);

    let epoch_info = rpc_client.get_epoch_info().unwrap();

    let absolute_slot = epoch_info.absolute_slot;

    let mut next_block = latest_block;
    let slot = rpc_client.get_slot().unwrap();
    let mut latest_scanned_block: String = "".to_string();

    for _ in ticker {
        println!("We are on the block {}", next_block);

        let block = rpc_client.get_block(slot).unwrap();

        if latest_scanned_block != block.blockhash {
            for tx in &block.transactions {
                // scan_transaction(&tx, &rpc);
                println!("Transaction {:?}", tx);
            }
        }
    }

}

fn get_address()  {

    let words = MnemonicType::Words12;
    let mnemonic = Mnemonic::new(words, Language::English);

    let index = 1;
    let path = format!("m/44'/501'/{}/0'", index);

    let derivation_path = DerivationPath::from_key_str(&path);
    let seed = Seed::new(&mnemonic, "");


    let keypair = keypair_from_seed_and_derivation_path(seed.as_bytes(),derivation_path.ok()).unwrap();
    let secret_key_bytes = keypair.secret().to_bytes();

    println!("Private key: {:?}", secret_key_bytes);
    println!("Public key: {}", keypair.pubkey());
}
