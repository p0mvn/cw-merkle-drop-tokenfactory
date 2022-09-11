use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::process;

mod controller;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// GenerateRoot generates merkle root from file consisting of accounts and
    /// amounts in csv format at a given path
    /// the first column must be an address and second column is an amount
    /// in cosmos-sdk Coin string format.
    /// prints the root hash to stdout, hex encoded.
    GenerateRoot {
        /// path the path to the file with accounts and amounts in csv format.
        /// See example in testdata.
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },


    /// GenerateProof generates a Merkle proof for 
    /// proof_for parameter if it is present in the data set given by
    /// data_set_path. By default, requires a proof_out_path to output
    /// data to. Instead, user has an option to provide a --print flag
    /// In that case, the result is produced to stdout.
    /// One of proof_out_path argument or print flag must be present. 
    GenerateProof {
        /// data_set_path the path to the file with accounts and amounts in csv format.
        /// See example in testdata.
        #[clap(parse(from_os_str))]
        data_set_path: std::path::PathBuf,

        /// proof_for the data to generate proof for.
        #[clap()]
        proof_for: String,

        #[clap(parse(from_os_str))]
        proof_out_path:  Option<std::path::PathBuf>,

        /// print flag indicating whether to print the proof.
        /// It is written to file by default.
        /// If this flag is true 
        #[clap(short, long)]
        print: bool,
    },

    /// Hash hashes the given data with the same hasher as
    /// is used in the underlying Merkle tree.  Outputs the result to stdout.\
    /// Primarily used for debugging purposes and has no production use case.
    Hash {
        /// The data to hash and print.
        #[clap()]
        data: String,
    }
}

fn generate_root_cmd(path: std::path::PathBuf) -> Result<(), Box<dyn Error>> {
    let entries = parse_csv(path)?;
    let hash = controller::run(&entries);
    println!("{}", hash);
    Ok(())
}

fn generate_proof_cmd(path: std::path::PathBuf, proof_for: &String, proof_out_path: &Option<std::path::PathBuf>, print: bool) -> Result<(), Box<dyn Error>> {
    let entries = parse_csv(path)?;

    let proof = controller::get_proof(&entries,  &proof_for.as_bytes().to_vec())?;

    if print {
        println!("{}", proof);
    }

    if proof_out_path.is_some() {
        fs::write(proof_out_path.as_ref().unwrap(), proof)?;
    }
    
    Ok(())
}

fn hash_cmd(data: &String) {
    let hash = controller::hash(data);
    println!("Data: {}", data);
    println!("Data Bytes: {:?}", data.as_bytes());
    println!("Data Hash: {}", hash);
}

fn parse_csv(path: std::path::PathBuf) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut csv_reader = csv::Reader::from_path(path)?;

    let mut entries: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();

    for str_record in csv_reader.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let entry = str_record?;

        entries.push(Vec::<u8>::from(entry.as_slice()));
    }
    Ok(entries)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::GenerateRoot { path }) => {
            if let Err(err) = generate_root_cmd(path.to_path_buf()) {
                eprintln!("error generating merkle root: {}", err);
                process::exit(1);
            }
        }
        Some(Commands::GenerateProof { data_set_path: path, proof_for: data, proof_out_path, print }) => {
            if proof_out_path.is_none() && !print {
                eprintln!("please provide a proof_out_path argument or set --print flag to true");
                process::exit(1);
            }
            
            if let Err(err) = generate_proof_cmd(path.to_path_buf(), data, proof_out_path, *print) {
                eprintln!("error generating merkle proof: {}", err);
                process::exit(1);
            }
        }
        Some(Commands::Hash { data,  }) => {
            if data.len() == 0 {
                eprintln!("data was empty, please provide something to hash");
                process::exit(1);
            }

            hash_cmd(data)
        }
        None => {}
    }
}
