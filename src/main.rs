use std::fs::File;
use std::io;
use std::io::{Error, Write};
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use ethers::types::H160;

#[derive(Debug, Clone, ValueEnum)]
enum Chains {
    Ethereum,
    Bnb,
    Polygon,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[clap(subcommand)]
    subcommand: Subcommands,
}

#[derive(Debug, Clone, Parser)]
struct SourceArgs {
    /// chain of the contract to be downloaded
    chain: Chains,
    /// address of the contract to be downloaded
    address: ethers::types::H160,
    /// file path, default will be current directory and combination of chain and address.
    /// Directory will be created if doesn't exist
    #[clap(long, value_hint = ValueHint::FilePath, value_name = "PATH")]
    file_path: Option<String>,
    /// Open the downloaded file in VsCode
    #[clap(long, value_name = "SIGNATURE")]
    open_vscode: bool,
}

#[derive(Debug, Subcommand)]
#[clap(
about = "Build, test, fuzz, debug and deploy Solidity contracts.",
after_help = "Find more information in the book: http://book.getfoundry.sh/reference/forge/forge.html",
next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
enum Subcommands {
    #[clap(visible_alias = "s", about = "download source from an explorer, such as etherscan")]
    Source(SourceArgs),
    #[clap(visible_alias = "ci", about = "get general information about the contract")]
    ContractInfo,
    #[clap(visible_alias = "di", about = "get general information about the deployer")]
    DeployerInfo,
}


fn main() -> eyre::Result<()> {

    //utils::load_dotenv(); //TODO: create env. file with apikeys?

    let cli = Cli::parse();

    match cli.subcommand {
        Subcommands::Source(args) => {
            unimplemented!("donwloading source")
        }
        Subcommands::ContractInfo => {
            unimplemented!("Getting contract info")
        }
        Subcommands::DeployerInfo => {
            unimplemented!("Getting deployer info")
        }
    }

    Ok(())
}

fn write_to_file(file_path_string: &str, file_content: &str) -> Result<(), Error>{
    let path = std::path::Path::new(file_path_string);

    // creates the full directory path, if it does not exist
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut output = File::create(path)?;
    output.write(file_content.as_bytes())?;

    Ok(())
}

fn get_source(chain: Chains, address: H160){
    unimplemented!("download contract source code");
    match chain {
        Chains::Ethereum => {}
        Chains::Bnb => {}
        Chains::Polygon => {}
    }
}