use base64::encode;
use mpl_token_metadata::{
    accounts,
    instructions::{self, CreateMetadataAccountV3InstructionArgs},
    types::{Creator, DataV2},
    ID,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    system_program,
    transaction::Transaction,
};

mod util;
use spl_associated_token_account::{
    get_associated_token_address,
    instruction::create_associated_token_account_idempotent,
};
fn main() {
    let key_pair = util::load_config_keypair();
    let mut ins: Vec<Instruction> = vec![];
    let wallet_publickey = key_pair.pubkey();
    let fee_payer = Some(&wallet_publickey);
    let mut signer: Vec<&Keypair> = vec![&key_pair];
    // change RPC endpoint here
    let rpc_url: String = "https://api.mainnet-beta.solana.com".to_string();
    let commitment = CommitmentConfig::confirmed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, commitment);
    let new_mint = Keypair::new();
    let mint_pub = new_mint.pubkey();
    let recent = rpc_client
        .get_latest_blockhash()
        .expect("failed to get recent blockhash");
    let lamport_needed = rpc_client
        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
        .unwrap();
    let space: u64 = spl_token::state::Mint::LEN.try_into().unwrap();
    let create_account_tx = create_account(
        &wallet_publickey,
        &mint_pub,
        lamport_needed,
        space,
        &spl_token::ID,
    );
    let create_mint_ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &mint_pub,
        &wallet_publickey,
        Some(&wallet_publickey),
        0,
    )
    .unwrap();
    let ata = get_associated_token_address(&wallet_publickey, &mint_pub);
    let create_ata_ins = create_associated_token_account_idempotent(
        &wallet_publickey,
        &wallet_publickey,
        &mint_pub,
        &spl_token::ID,
    );
    let mint_one_ins =
        spl_token::instruction::mint_to(&spl_token::ID, &mint_pub, &ata, &wallet_publickey, &[], 1)
            .unwrap();
    let seeds = &[
        accounts::Metadata::PREFIX,
        &ID.to_bytes(),
        &mint_pub.to_bytes(),
    ];
    let creator = Creator {
        address: wallet_publickey,
        verified: true,
        share: 100,
    };
    let creators = Some(vec![creator]);
    let metadata_account = Pubkey::find_program_address(seeds, &ID).0;
    let metadata_ins_builder = instructions::CreateMetadataAccountV3 {
        metadata: metadata_account,
        mint: mint_pub,
        mint_authority: wallet_publickey,
        payer: wallet_publickey,
        update_authority: (wallet_publickey, true),
        system_program: system_program::ID,
        rent: None,
    };
    let metadata_ins = metadata_ins_builder.instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "".to_string(),
            symbol: "".to_string(),
            uri: "".to_string(),
            seller_fee_basis_points: 0,
            creators,
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    });

    ins.push(create_account_tx);
    ins.push(create_mint_ix);
    ins.push(create_ata_ins);
    ins.push(mint_one_ins);
    ins.push(metadata_ins);
    signer.push(&new_mint);
    let mut tx = Transaction::new_with_payer(&ins, fee_payer);
    tx.sign(&signer, recent);
    let messagee = encode(tx.message_data());

    let simulation = rpc_client.simulate_transaction(&tx);
    match simulation {
        Ok(_) => {
            let send = rpc_client.send_and_confirm_transaction_with_spinner(&tx);
            println!(
                "tx: {:?} \nmint:{:?}\nresult:{:?}",
                messagee,
                new_mint.pubkey().to_string(),
                send
            );
        }
        Err(e) => {
            println!(
                "tx: {:?} \nmint:{:?}\nresult:{:?}",
                messagee,
                new_mint.pubkey().to_string(),
                e
            );
        }
    }
}
