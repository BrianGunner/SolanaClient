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

    let main_account = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let main_pubkey = main_account.pubkey();

    let solomon_token = read_keypair_file("solomon-mint.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let solomon_pubkey = solomon_token.pubkey();

    let token_account = read_keypair_file("token-account.json").map_err(|e|anyhow::anyhow!("could not load token-account: {}",e))?;
    let token_pubkey = token_account.pubkey();

    let space = TokenAccount::LEN;
    let space_lamports = client.get_minimum_balance_for_rent_exemption(space)?;

    let recent_blockhash = client.get_latest_blockhash()?;
    
    let token_account_instruction = system_instruction::create_account(
        &main_pubkey, 
        &token_pubkey, 
        space_lamports, 
        space as u64, 
        &spl_token::ID,
    );

    let init_token_acc_instruction = spl_token::instruction::initialize_account(
        &spl_token::ID, 
        &token_pubkey, 
        &solomon_pubkey, 
        &main_pubkey,
    )?;

    let init_tx = Transaction::new_signed_with_payer(
        &[token_account_instruction,init_token_acc_instruction], 
        Some(&main_pubkey), 
        &[&main_account,&token_account],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&init_tx);

    match signature{
        std::result::Result::Ok(sig)=>println!("Solomon Token Mint created with signature: {}",sig),
        Err(msg)=>println!("{}",msg),
    }

    println!("Token Account Pubkey: {}",token_account.pubkey());
    let token_account_info = client.get_account(&token_pubkey)?;
    println!("Token Owner: {}",token_account_info.owner);
    println!("Token lamports: {}",token_account_info.lamports);
    println!("Token Executable: {}",token_account_info.executable);
    println!("Token Data Length {}",token_account_info.data.len());

    
    Ok(())
}