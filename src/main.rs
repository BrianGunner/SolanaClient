

use solana_client::rpc_client::RpcClient;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction;
use std::str::FromStr;


fn main(){
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5";
    let client = RpcClient::new(rpc_url.to_string());
    let sender = read_keypair_file("id.json").expect("Failed to read Json");
    let sender_pubkey = sender.pubkey();

    let receiver = Pubkey::from_str("Hy5nk6c3ga3DvmiWTpeW8o5z7ohii6YCvHoHai9FX8TD").expect("Could not read pubkey");

    let transfer_instruction = system_instruction::transfer(&sender_pubkey, &receiver, 1_000_000_00);
   

    let blockhash = client.get_latest_blockhash().unwrap();

    //.let transaction = Transaction::new_signed_with_payer(&[transfer_instruction], 
        //Some(&sender_pubkey), 
        //&[&sender], 
        //blockhash,
    //);
    //let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    //println!("{}",signature);
    let receiver_balance = client.get_balance(&receiver).unwrap();
    println!("{}",receiver_balance as f64/1_000_000_000.0);
}
