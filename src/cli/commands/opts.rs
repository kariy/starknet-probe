use crate::cli::parser::ChainParser;

use clap::Parser;
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, Parser)]
pub struct StarkNetOptions {
    #[clap(long)]
    #[clap(value_name = "URL")]
    #[clap(help = "The RPC endpoint")]
    #[clap(env = "STARKNET_RPC_URL")]
    #[clap(default_value = "http://localhost:5050/rpc")]
    pub rpc_url: String,

    #[clap(long)]
    #[clap(env = "STARKNET_CHAIN")]
    #[clap(value_name ="CHAIN_ID")]
    #[clap(value_parser(ChainParser))]
    pub chain: Option<FieldElement>,
    
    #[clap(flatten)]
    #[clap(next_help_heading = "WALLET OPTIONS")]
    pub wallet: WalletOptions,
}

#[derive(Debug, Clone, Parser)]
pub struct TransactionOptions {
    #[clap(long)]
    pub nonce: Option<FieldElement>,
    
    #[clap(long)]
    pub max_fee: Option<FieldElement>
}

#[derive(Debug, Clone, Parser)]
pub struct WalletOptions {
    #[clap(long)]
    #[clap(value_name = "PRIVATE_KEY")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "The raw private key associated with the account contract.")]
    pub private_key: Option<String>,

    #[clap(long)]
    #[clap(value_name = "ACCOUNT_ADDRESS")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Account contract to initiate the transaction from.")]
    pub account_address: Option<String>
}