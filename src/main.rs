use dotenv::dotenv;
use std::{env, str::FromStr};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
//    commitment_config::CommitmentConfig,
};
use mpl_token_metadata::{
    ID as TOKEN_METADATA_PROGRAM_ID,
    instructions::CreateV1Builder,
    types::{TokenStandard, PrintSupply},
};

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Load the secret key from .env
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Failed to decode key");
    let user = Keypair::from_bytes(&as_array).expect("Failed to create Keypair from secret key");

    let token_mint_account = Pubkey::from_str("GLCkK1D5aKAaeeQSLRXHLzdWrrkmad2rJXBD3A5mWTis")
        .expect("Failed to create Pubkey from string");

    // Finding the PDA for the metadata account
    let (metadata_pda, _bump_seed) = Pubkey::find_program_address(
        &[
            b"metadata",
            &TOKEN_METADATA_PROGRAM_ID.to_bytes(),
            &token_mint_account.to_bytes(),
        ],
        &TOKEN_METADATA_PROGRAM_ID,
    );

    // Metadata information
    let name = String::from("Solana Training Token");
    let symbol = String::from("TRAIN_KHAL");
    let uri = String::from("https://arweave.net/1234");

    // Creating the metadata account instruction
    let create_metadata_account_ix = CreateV1Builder::new()
        .metadata(metadata_pda)
        .mint(token_mint_account, true)
        .authority(user.pubkey())
        .payer(user.pubkey())
        .update_authority(user.pubkey(), true)
        .is_mutable(true)
        .primary_sale_happened(false)
        .name(name)
        .symbol(symbol)
        .uri(uri)
        .seller_fee_basis_points(0)
        .token_standard(TokenStandard::NonFungible)
        .print_supply(PrintSupply::Zero)
        .instruction();

    // Create the transaction and add the instruction
    let mut transaction = Transaction::new_with_payer(
        &[create_metadata_account_ix],
        Some(&user.pubkey()),
    );

    // Assuming `connection` is your RpcClient and you have the necessary blockhash
    let connection = RpcClient::new("https://api.devnet.solana.com");
    let recent_blockhash = connection.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Sign the transaction with both the user keypair and the new account keypair
    transaction.sign(&[&user], recent_blockhash);

    // Send and confirm the transaction
    connection
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print the metadata PDA for verification
    println!("Metadata PDA: {}", metadata_pda);
}
