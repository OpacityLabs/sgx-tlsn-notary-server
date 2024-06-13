use structopt::StructOpt;

/// Fields loaded from the command line when launching this server.
#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Opacity CLI Options")]
pub struct CliFields {
    /// Configuration file location
    #[structopt(long, default_value = "./config/config.yaml")]
    pub tlsn_config: String,
    #[structopt(long, default_value = "./config/operator.config.yaml")]
    pub operator_config: String,
    #[structopt(long)]
    pub operator_ecdsa_key_password: String,
    #[structopt(long)]
    pub operator_bls_key_password: String,
}
