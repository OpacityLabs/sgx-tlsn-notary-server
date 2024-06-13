use serde::Deserialize;
use structopt::StructOpt;

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, StructOpt, Deserialize)]
pub struct OperatorConfiguration {
    // Service manager address
    pub opacity_avs_address: String,
    pub avs_directory_address: String,
    pub chain_id: usize,
    pub eth_rpc_url: String,
    pub ecdsa_private_key_store_path: String,
    pub bls_private_key_store_path: String,
}
