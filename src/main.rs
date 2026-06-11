use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::str::FromStr;
use anyhow::Result;


fn main() -> Result<()> {
    // 1. Connect to devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());
    let pubkey = Pubkey::from_str("6WYtJeEVcXhMH3pumbZ82Me8NyNAnpXmSydajzeGvwun").unwrap();

    match client.get_balance(&pubkey){
        Ok(balance)=>{
            let Sol = balance as f64/1_000_000_000.0;
            println!("Solana Balance: {}",Sol);
            println!("Lamports Balance: {}",balance);
        },
        Err(msg)=>println!("{}",msg),
    }
    
    // 2. Generate keypairs (you and receiver)

    let receiver = Keypair::new();
    println!("Receiver public key : {}",receiver.pubkey());

    let receiver_pubkey = Pubkey::from_str("Fizs5pjY6Uf5PgsjTUuHPi2t1UT6a9PLmCjqz3jSbsvZ").unwrap();

    match client.get_balance(&receiver_pubkey){
        Ok(balance)=>{
            let Sol = balance as f64/1_000_000_000.0;
            println!("Sol receiver balance: {}",Sol);
        },
        Err(msg)=>println!("{}",msg),
    }    
    // 3. Airdrop SOL to your account
    
    // 4. Check balance
    
    // 5. Create transfer instruction
    
    // 6. Send transaction
    
    // 7. Confirm and print balances
    
    Ok(())
}