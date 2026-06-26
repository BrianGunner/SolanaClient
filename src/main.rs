use anyhow::{Context, Error, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::transaction::Transaction;
use solana_sdk::{pubkey::Pubkey,signature::Keypair,signer::Signer};
use solana_sdk::signature::read_keypair_file;
use solana_sdk::system_instruction::{self, SystemInstruction};
use spl_token::state::Account as TokenAccount;
use solana_program::program_pack::Pack;
use std::any;
use std::str::FromStr;
use spl_token::state::Mint;
use std::fs;
use solana_sdk::signature::write_keypair_file;

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);

    let main_account = read_keypair_file("main-account.json").map_err(|e|anyhow::anyhow!("Could not read keypair: {}",e))?;
    let main_pubkey = main_account.pubkey();
    
    let mint_account = read_keypair_file("drill-mint.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let mint_pubkey = mint_account.pubkey();

    let mint_info = client.get_account(&mint_pubkey)?;
    let mint_data = Mint::unpack(&mint_info.data)?;
    println!("Mint Data Is Initialized: {}",mint_data.is_initialized);
    println!("Decimals: {}",mint_data.decimals);
    println!("Total supply: {}",mint_data.supply);


    Ok(())

}




