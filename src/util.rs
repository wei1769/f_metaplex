use solana_sdk::{
    instruction::{AccountMeta, },
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair},
};
use std::str::FromStr;
pub fn get_pub(pubkey: &str) -> Pubkey {
    Pubkey::from_str(pubkey).unwrap()
}
pub fn getkey(public_key: Pubkey, is_signer: bool, is_writable: bool) -> AccountMeta {
    if is_writable {
        AccountMeta::new(public_key, is_signer)
    } else {
        AccountMeta::new_readonly(public_key, is_signer)
    }
}

pub fn load_config_keypair() -> Keypair {
    let config_path = solana_cli_config::CONFIG_FILE.as_ref().unwrap();
    let cli_config =
        solana_cli_config::Config::load(config_path).expect("failed to load config file");
    read_keypair_file(cli_config.keypair_path).expect("failed to load keypair")
}
