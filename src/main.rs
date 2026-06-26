use anyhow::{Context, Error, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
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

    let space = Mint::LEN;
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;
    let recent_blockhash = client.get_latest_blockhash()?;

    let mint_ac_inst = system_instruction::create_account(
        &main_pubkey, 
        &mint_pubkey, 
        lamports, 
        space as u64, 
        &spl_token::ID
    );

    let mint_ac_tx = Transaction::new_signed_with_payer(
        &[mint_ac_inst], 
        Some(&main_pubkey), 
        &[&main_account,&mint_account], 
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&mint_ac_tx);

    match signature{
        std::result::Result::Ok(sig)=>println!("Mint account created: {}",sig),
        Err(msg)=>println!("{}",msg),
    }


    

    Ok(())

}




