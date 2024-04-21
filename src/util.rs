use solana_sdk::signature::{read_keypair_file, Keypair};

pub fn load_config_keypair() -> Keypair {
    let config_path = solana_cli_config::CONFIG_FILE.as_ref().unwrap();
    let cli_config =
        solana_cli_config::Config::load(config_path).expect("failed to load config file");
    read_keypair_file(cli_config.keypair_path).expect("failed to load keypair")
}
