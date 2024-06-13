use eth_keystore::{decrypt_key, encrypt_key, new};
use ethers_signers::{LocalWallet, Signer};
use eyre::{eyre, Result};
use hex;
use secp256k1::{PublicKey, SecretKey};
use std::path::Path;
use structopt::StructOpt;
use tracing::debug;
use web3::signing::Key;

use opacity_avs_node::{
    init_tracing, parse_config_file, run_server, CliFields, NotaryServerError,
    NotaryServerProperties, OperatorConfiguration,
};

#[tokio::main]
async fn main() -> Result<(), NotaryServerError> {
    // Load command line arguments which contains the config file location
    let cli_fields: CliFields = CliFields::from_args();
    let tlsn_config: NotaryServerProperties = parse_config_file(&cli_fields.tlsn_config)?;
    let operator_config: OperatorConfiguration = parse_config_file(&cli_fields.operator_config)?;

    // Set up tracing for logging
    init_tracing(&tlsn_config).map_err(|err| eyre!("Failed to set up tracing: {err}"))?;

    debug!(?tlsn_config, "Server config loaded");
    debug!(?operator_config, "Operator config loaded");

    // Load the operator's private key

    // let ecdsa_hex_key = hex::encode(&operator_ecdsa_private_key);

    let wallet = LocalWallet::decrypt_keystore(
        &operator_config.ecdsa_private_key_store_path,
        &cli_fields.operator_ecdsa_key_password,
    )
    .unwrap();

    let wallet = LocalWallet::decrypt_keystore(
        &operator_config.ecdsa_private_key_store_path,
        &cli_fields.operator_ecdsa_key_password,
    )
    .unwrap();

    println!("Wallet: {:?}", &wallet.address());

    // Run the server
    // run_server(&tlsn_config).await?;

    Ok(())
}
