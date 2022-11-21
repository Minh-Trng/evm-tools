use std::borrow::Cow;
use std::fmt::format;
use std::fs::File;
use std::io::{Write};
use std::process::Command;
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use ethers::types::H160;
use serde_json::Value;

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
    /// directory path, default will be current directory. directory will be created if doesn't exist
    #[clap(long, default_value = ".",value_hint = ValueHint::DirPath, value_name = "DIR")]
    parent_dir: String,
    /// name of the file (if single file) or directory, where the source will be stored. For single
    /// files, the ending ".sol" will be appended.
    #[clap(long, value_name = "FILE_NAME")]
    destination: Option<String>,
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
            handle_source(&args);
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

fn handle_source(args: &SourceArgs){
    match get_source(&args.chain, &args.address) {
        Ok(source) => {
            write_source_to_disk(&args, &source);
            if args.open_vscode {
                Command::new("code").arg(&args.parent_dir);
            }
        }
        Err(error) => { eprintln!("Couldnt download source. Error: {error}")}
    }
}

fn write_source_to_disk(args: &SourceArgs, source: &str){
    let is_single_file = source_is_single_file(source);

    let file_name =
        if let Some(passed_name) = &args.destination {
            if is_single_file {
                Cow::Owned(format!("{passed_name}.sol"))
            } else {
                Cow::Borrowed(passed_name)
            }
        } else {
            let file_ending = if is_single_file {".sol"} else { "" };
            Cow::Owned(format!("{:?}_{:?}{:?}", &args.chain, &args.address, file_ending))
        };

    if is_single_file {
        let path_str = format!("{}/{}", args.parent_dir, file_name);
        let path = std::path::Path::new(&path_str);
        // creates the full directory path, if it does not exist
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let mut output = File::create(path).expect("file should be created");
        output.write(source.as_bytes()).expect("file should be written to");
    } else {
        //remove redundant curly braces
        let source = &source[1..source.len() - 1];
        let source_value: Value = serde_json::from_str(source).expect("source should be valid json");
        let source_iterable = source_value["sources"].as_object().expect("value should be convertable to object");
        for (source_file_path, source_file_object) in source_iterable{
            if let Value::String(source_file_string) = &source_file_object["content"]{
                let path_str = format!("{}/{}{}", args.parent_dir, file_name, source_file_path);
                let path = std::path::Path::new(&path_str);
                let prefix = path.parent().unwrap();
                std::fs::create_dir_all(prefix).unwrap();
                let mut output = File::create(path).expect("file should be created");
                output.write(source_file_string.as_bytes()).expect("file should be written to");
            }
        }
    }

}


fn source_is_single_file(source: &str) -> bool {
    // based on https://github.com/amimaro/smart-contract-downloader/commit/fafc613e82e457098005442afa5d1e0037d962d6
    // return source.starts_with("pragma") ||
    //     source.starts_with("//") ||
    //     source.starts_with("\r\n") ||
    //     source.starts_with("/*")

    // alternatively:
    return !(source.starts_with("{{") && source.ends_with("}}"))
}

fn get_source(chain: &Chains, address: &H160) -> eyre::Result<String>{
    match chain {
        Chains::Ethereum => {
            let url = format!("https://api.etherscan.io/api?module=contract&\
                                action=getsourcecode&address={:?}", address);
            let json_str = reqwest::blocking::get(url)?.text()?;
            let parsed: serde_json::Value = serde_json::from_str(&json_str)?;
            let source_value = &parsed["result"][0]["SourceCode"];
            match source_value {
                Value::String(source_string) => {
                    Ok(source_string.to_string())
                }
                _ => {panic!("unexpected serde_json variant in api response")}
            }
        }
        Chains::Bnb => {unimplemented!("download from bnb explorer")}
        Chains::Polygon => {unimplemented!("download from polygon explorer")}
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works(){

    }

}