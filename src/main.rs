use anyhow::{Context, Error, Ok, Result};
use solana_client::pubsub_client::PubsubSignatureClientSubscription;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_pack::IsInitialized;
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
use mpl_token_metadata::accounts::Metadata;

fn main()->Result<()>{
    let rpc_url = "https://devnet.helius-rpc.com/?api-key=a6a3f92d-2503-4f4f-bb01-11a49e284aa5".to_string();
    let client = RpcClient::new(rpc_url);

    let main_account = read_keypair_file("main-account.json").map_err(|e|anyhow::anyhow!("Could not read keypair: {}",e))?;
    let main_pubkey = main_account.pubkey();
    
    let mint_account = read_keypair_file("drill-mint.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let mint_pubkey = mint_account.pubkey();

    let token_account = read_keypair_file("token-account-2.json").map_err(|e|anyhow::anyhow!("Could not load keypair: {}",e))?;
    let token_pubkey = token_account.pubkey();

    let metadata_pda = Metadata::find_pda(&mint_pubkey);
    println!("{}",metadata_pda.1);

    let meta_inst = mpl_token_metadata::instructions::CreateMetadataAccountV3{
        metadata:metadata_pda.0,
        mint:mint_pubkey,
        mint_authority:main_pubkey,
        payer:main_pubkey,
        update_authority:(main_pubkey,true),
        system_program:solana_sdk::system_program::ID,
        rent:None,
    };

    let meta_args = mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs{
        data:mpl_token_metadata::types::DataV2{
            name:"Solomon Token".to_string(),
            symbol:"Solo".to_string(),
            uri:"".to_string(),
            seller_fee_basis_points:0,
            creators:None,
            collection:None,
            uses:None,
        },
        is_mutable:true,
        collection_details:None,
    };

    let meta_instruction = meta_inst.instruction(meta_args);

    let recent_blockhash = client.get_latest_blockhash()?;

    let meta_tx = Transaction::new_signed_with_payer(
        &[meta_instruction], 
        Some(&main_pubkey), 
        &[&main_account], 
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&meta_tx);

    match signature{
        std::result::Result::Ok(sig)=>println!("Signature: {}",sig),
        Err(msg)=>println!("{}",msg),
    }
    
    Ok(())

}




