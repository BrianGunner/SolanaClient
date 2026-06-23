use anyhow::{Context, Result};
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

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);

    let sender = read_keypair_file("id.json").map_err(|e|anyhow::anyhow!("could not read keypair: {}",e))?;
    let sender_pubkey = sender.pubkey();
    let account_a = Keypair::new();
    let account_a_pubkey = account_a.pubkey();
    let space_a = 64 as usize;
    let min_lamports_a = client.get_minimum_balance_for_rent_exemption(space_a)?;
     let latest_blockhash = client.get_latest_blockhash()?;
    let instruction_account_a = system_instruction::create_account(
        &sender_pubkey, 
        &account_a_pubkey, 
        min_lamports_a, 
        space_a as u64, 
        &sender_pubkey,
    );
    let transaction_a = Transaction::new_signed_with_payer(
        &[instruction_account_a], 
        Some(&sender_pubkey), 
        &[&sender,&account_a], 
        latest_blockhash,
    );
    
    let singature_a = client.send_and_confirm_transaction(&transaction_a)?;
    println!("Account a created with signature: {}",singature_a);

    let account_b = Keypair::new();
    let account_b_pubkey = account_b.pubkey();
    let space_b = TokenAccount::LEN;
    let min_lamports_b = client.get_minimum_balance_for_rent_exemption(space_b)?;
   

    let instruction_new_account = system_instruction::create_account(
        &sender_pubkey, 
        &account_b_pubkey, 
        min_lamports_b,
        space_b as u64, 
        &spl_token::ID,
    );

    let transaction_account = Transaction::new_signed_with_payer(
        &[instruction_new_account],
        Some(&sender_pubkey), 
        &[&sender,&account_b], 
        latest_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction_account)?;
    println!("Account b created with Signature: {}",signature);

    let account_a_info = client.get_account(&account_a_pubkey)?;
    let account_b_info = client.get_account(&account_b_pubkey)?;

    println!("Account a owner: {}",account_a_info.owner);
    println!("Account a lamports: {}",account_a_info.lamports);
    println!("Account b owner: {}",account_b_info.owner);
    println!("Account b lamports: {}",account_b_info.lamports);

    //Transfer from Account b//

    let transfer_b_blockhash = client.get_latest_blockhash()?;
    let transfer_from_b_instruction = system_instruction::transfer(
        &account_b_pubkey, 
        &account_a_pubkey, 
        1_000_000
    );
    let transfer_from_b_tx = Transaction::new_signed_with_payer(
        &[transfer_from_b_instruction], 
        Some(&sender_pubkey), 
        &[&sender,&account_b], 
        transfer_b_blockhash,
    );

    let signature_b = client.send_and_confirm_transaction(&transfer_from_b_tx);
    match signature_b{
        std::result::Result::Ok(sig)=>println!("B success: signature: {}",sig),
        Err(msg)=>println!("B failed: {}",msg),
    }
    

    //Transfer from account a//
    let transfer_a_blockhash = client.get_latest_blockhash()?;
    let transfer_from_a_instruction = system_instruction::transfer(
        &account_a_pubkey, 
        &account_b_pubkey, 
        1_000_000
    );

    let transfer_from_a_tx = Transaction::new_signed_with_payer(
        &[transfer_from_a_instruction], 
        Some(&sender_pubkey), 
        &[&sender,&account_a], 
        transfer_a_blockhash,
    );

    let signature_a = client.send_and_confirm_transaction(&transfer_from_a_tx);

     match signature_a{
        std::result::Result::Ok(sig)=>println!("account success: signature: {}",sig),
        Err(msg)=>println!("A failed: {}",msg),
    }


    Ok(())
}