use super::account::WalletCommands;
use super::parser::BlockIdParser;
use super::rpc::RpcArgs;
use super::send::InvokeArgs;
use crate::opts::starknet::StarkNetOptions;

use clap::{Parser, Subcommand};
use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockId};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "probe", version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(visible_alias = "th")]
    #[clap(name = "--to-hex")]
    #[clap(about = "Convert decimal felt to hexadecimal.")]
    DecToHex {
        #[clap(value_name = "DECIMAL")]
        dec: FieldElement,
    },

    #[clap(visible_alias = "td")]
    #[clap(name = "--to-dec")]
    #[clap(about = "Convert hexadecimal felt to decimal.")]
    HexToDec {
        #[clap(value_name = "HEX")]
        hex: FieldElement,
    },

    #[clap(visible_alias = "mxf")]
    #[clap(name = "--max-felt")]
    #[clap(about = "Get the maximum felt value.")]
    MaxUnsignedFelt,

    #[clap(visible_alias = "mxsf")]
    #[clap(name = "--max-sfelt")]
    #[clap(about = "Get the maximum signed felt value.")]
    MaxSignedFelt,

    #[clap(visible_alias = "mnsf")]
    #[clap(name = "--min-sfelt")]
    #[clap(about = "Get the minimum signed felt value.")]
    MinSignedFelt,

    #[clap(visible_alias = "fa")]
    #[clap(name = "--from-ascii")]
    #[clap(about = "Convert from ASCII to Cairo short string.")]
    FromAscii {
        #[clap(value_name = "ASCII")]
        ascii: FieldElement,
    },

    #[clap(visible_alias = "ta")]
    #[clap(name = "--to-ascii")]
    #[clap(about = "Convert Cairo short string to its ASCII format.")]
    ToAscii {
        #[clap(value_name = "SHORT_STRING")]
        short_str: String,
    },

    #[clap(visible_alias = "su")]
    #[clap(name = "--split-u256")]
    #[clap(about = "Split a uint256 into its low and high components.")]
    SplitU256 { value: String },

    #[clap(visible_alias = "acc")]
    #[clap(about = "Account management utilities")]
    Account {
        #[clap(subcommand)]
        commands: WalletCommands,
    },

    #[clap(about = "Get the timestamp of a block.")]
    Age {
        #[clap(next_line_help = true)]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "bal")]
    #[clap(about = "Get the ETH balance of an address.")]
    Balance {
        #[clap(value_name = "ADDRESS")]
        #[clap(help = "The address whose balance you want to query.")]
        address: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "b")]
    #[clap(about = "Get information about a block.")]
    Block {
        #[clap(next_line_help = true)]
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        id: BlockId,

        #[clap(long)]
        #[clap(action(clap::ArgAction::SetTrue))]
        #[clap(help = "Get the full information (incl. transactions) of the block.")]
        full: bool,

        #[clap(long)]
        field: Option<String>,

        #[clap(short = 'j', long = "json")]
        #[clap(help_heading = "Display options")]
        to_json: bool,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "bn")]
    #[clap(about = "Get the latest block number.")]
    BlockNumber {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Call a StarkNet function without creating a transaction.")]
    Call {
        #[clap(display_order = 1)]
        contract_address: FieldElement,

        #[clap(display_order = 2)]
        #[clap(help = "The name of the function to be called")]
        #[clap(value_name = "FUNCTION_NAME")]
        function: String,

        #[clap(short, long)]
        #[clap(display_order = 3)]
        #[clap(value_delimiter = ',')]
        #[clap(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
        input: Vec<FieldElement>,

        #[clap(short, long)]
        #[clap(display_order = 4)]
        #[clap(help = "Path to the contract's abi file to validate the call input.")]
        abi: Option<PathBuf>,

        #[clap(next_line_help = true)]
        #[clap(display_order = 5)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "ci")]
    #[clap(about = "Get the StarkNet chain ID.")]
    ChainId {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "cl")]
    #[clap(
        about = "Get the contract class definition in the given block associated with the given hash"
    )]
    Class {
        #[clap(value_name = "CLASS_HASH")]
        #[clap(help = "The hash of the requested contract class")]
        hash: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "cd")]
    #[clap(about = "Get the contract class definition in the given block at the given address")]
    Code {
        #[clap(help = "The address of the contract whose class definition will be returned")]
        contract_address: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "ca")]
    #[clap(about = "Compute the contract address from the given information")]
    ComputeAddress {
        #[clap(help = "The address of the deploying account contract (currently always zero)")]
        caller_address: FieldElement,

        #[clap(help = "The salt used in the deploy transaction")]
        salt: FieldElement,

        #[clap(help = "The hash of the class to instantiate a new contract from")]
        class_hash: FieldElement,

        #[clap(help = "The inputs passed to the constructor")]
        calldata: Vec<FieldElement>,
    },

    #[clap(visible_alias = "cc")]
    #[clap(
        about = "Get the contract class hash in the given block for the contract deployed at the given address"
    )]
    ContractClass {
        #[clap(help = "The address of the contract whose class hash will be returned")]
        contract_address: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "ch")]
    #[clap(about = "Compute the hash of a StarkNet contract.")]
    ContractHash {
        #[clap(help = "The compiled contract file")]
        contract: PathBuf,
    },

    #[clap(visible_alias = "ec")]
    #[clap(about = "Perform ECDSA operations over the STARK-friendly elliptic curve.")]
    Ecdsa {
        #[clap(subcommand)]
        commands: EcdsaCommand,
    },

    #[clap(visible_alias = "ev")]
    #[clap(about = "Returns all events matching the given filter")]
    #[clap(
        long_about = "Returns all event objects matching the conditions in the provided filter"
    )]
    Events {
        #[clap(short = 'C', long)]
        #[clap(value_name = "CONTRACT_ADDRESS")]
        #[clap(help = "Address of the contract emitting the events")]
        from: Option<FieldElement>,

        #[clap(short, long)]
        #[clap(value_delimiter = ',')]
        #[clap(help = "The values used to filter the events")]
        #[clap(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
        keys: Option<Vec<FieldElement>>,

        #[clap(short, long)]
        #[clap(value_parser(BlockIdParser))]
        from_block: Option<BlockId>,

        #[clap(short, long)]
        #[clap(value_parser(BlockIdParser))]
        to_block: Option<BlockId>,

        #[clap(required = true)]
        #[clap(short = 's', long)]
        chunk_size: u64,

        #[clap(short = 'c', long)]
        #[clap(
            help = "A pointer to the last element of the delivered page, use this token in a subsequent query to obtain the next page"
        )]
        continuation_token: Option<String>,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "idx")]
    #[clap(about = "Compute the address of a storage variable.")]
    Index {
        #[clap(value_name = "VAR_NAME")]
        variable_name: String,

        keys: Vec<FieldElement>,
    },

    #[clap(visible_alias = "inv")]
    #[clap(about = "Submit a new transaction to be added to the chain.")]
    Invoke(InvokeArgs),

    #[clap(visible_alias = "kck")]
    #[clap(about = "Hash abritrary data using StarkNet keccak.")]
    Keccak {
        #[clap(value_name = "DATA")]
        data: String,
    },

    #[clap(visible_alias = "n1")]
    #[clap(about = "Get the latest nonce associated with the address.")]
    Nonce {
        contract_address: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "ped")]
    #[clap(about = "Calculate the Pedersen hash on two field elements.")]
    Pedersen {
        #[clap(value_name = "X")]
        x: String,
        #[clap(value_name = "Y")]
        y: String,
    },

    #[clap(about = "Perform a raw JSON-RPC request.")]
    Rpc(RpcArgs),

    #[clap(about = "Get the information about the result of executing the requested block")]
    StateUpdate {
        #[clap(next_line_help = true)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "str")]
    #[clap(about = "Get the value of a contract's storage at the given index")]
    Storage {
        contract_address: FieldElement,

        index: FieldElement,

        #[clap(next_line_help = true)]
        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(name = "tx")]
    #[clap(about = "Get information about a transaction.")]
    Transaction {
        #[clap(value_name = "TX_HASH")]
        hash: FieldElement,

        #[clap(long)]
        field: Option<String>,

        #[clap(short = 'j', long = "json")]
        #[clap(help_heading = "Display options")]
        to_json: bool,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "txc")]
    #[clap(name = "tx-count")]
    #[clap(about = "Get the number of transactions in a block.")]
    TransactionCount {
        #[clap(next_line_help = true)]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "txp")]
    #[clap(name = "tx-pending")]
    #[clap(about = "Get the transactions in the transaction pool, recognized by the sequencer.")]
    TransactionPending {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "txs")]
    #[clap(name = "tx-status")]
    #[clap(about = "Get the status of a transaction.")]
    TransactionStatus {
        #[clap(value_name = "TX_HASH")]
        hash: FieldElement,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "rct")]
    #[clap(name = "receipt")]
    #[clap(about = "Get the receipt of a transaction.")]
    TransactionReceipt {
        #[clap(value_name = "TX_HASH")]
        hash: FieldElement,

        #[clap(long)]
        field: Option<String>,

        #[clap(short = 'j', long = "json")]
        #[clap(help_heading = "Display options")]
        to_json: bool,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(visible_alias = "gca")]
    #[clap(about = "Generate call array calldata")]
    CallArray {
        #[clap(required = true)]
        #[clap(value_delimiter = ' ')]
        #[clap(help = r#"List of calls seperated with a hyphen, -
        example : <contract address> <function name> [<calldata> ...] - ..."#)]
        calls: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[clap(about = "Sign a message.")]
    Sign {
        #[clap(short, long)]
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(help = "Message hash to be signed.")]
        message: FieldElement,

        #[clap(short, long)]
        #[clap(value_name = "PRIVATE_KEY")]
        #[clap(help = "The private key for signing.")]
        private_key: FieldElement,
    },

    #[clap(about = "Verify the signature of a message.")]
    Verify {
        #[clap(short, long)]
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(help = "Message hash used in the signature.")]
        message: FieldElement,

        #[clap(short, long)]
        #[clap(required = true)]
        #[clap(number_of_values = 2)]
        #[clap(value_names = &["SIGNATURE_R", "SIGNATURE_S"])]
        signature: Vec<FieldElement>,

        #[clap(short, long)]
        #[clap(value_name = "VERIFYING_KEY")]
        #[clap(help = "The key for verification.")]
        verifying_key: FieldElement,
    },
}

#[cfg(test)]
mod tests {
    use super::App;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        App::command().debug_assert()
    }
}
