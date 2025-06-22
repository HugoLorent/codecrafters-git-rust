use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "A simple Git implementation in Rust", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize a new Git repository
    Init,
    /// Provide contents or details of repository objects
    CatFile {
        /// Pretty-print the contents of the object
        #[arg(short = 'p', help = "Pretty-print the contents of the object")]
        pretty_print: bool,
        /// The SHA-1 hash of the object to display
        #[arg(help = "The SHA-1 hash of the object to display")]
        object_hash: String,
    },
    /// Compute object ID and optionally create an object from a file
    HashObject {
        /// Write the object into the object database
        #[arg(short = 'w', help = "Write the object into the object database")]
        write: bool,
        /// Path to the file to hash
        #[arg(help = "Path to the file to hash")]
        file_path: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            fs::create_dir(".git").context("Failed to create .git directory")?;
            fs::create_dir(".git/objects").context("Failed to create .git/objects directory")?;
            fs::create_dir(".git/refs").context("Failed to create .git/refs directory")?;
            fs::write(".git/HEAD", "ref: refs/heads/main\n")
                .context("Failed to write HEAD file")?;
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            // Create path from object_hash, two first characters are folder name, next is the filename
            let file_path = format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]);
            // Read file content as bytes
            let bytes = fs::read(&file_path)
                .with_context(|| format!("Failed to read object file: {}", file_path))?;

            if pretty_print {
                // Read and decompress blob file content
                let mut decoder = ZlibDecoder::new(&bytes[..]);
                let mut blob_content = String::new();
                decoder
                    .read_to_string(&mut blob_content)
                    .context("Failed to decompress object")?;

                // Remove header from blob content, get only real file content by splitting from the null byte
                let file_content = blob_content
                    .split('\0')
                    .nth(1)
                    .context("Invalid object format: missing null separator")?;
                print!("{}", file_content.trim_end());
            }
        }
        Command::HashObject { write, file_path } => {
            // Read file content as bytes
            let file_content = fs::read(&file_path)
                .with_context(|| format!("Failed to read input file: {}", file_path.display()))?;

            // Create the header content
            let header = format!("blob {}\0", file_content.len());
            // Create the object content
            let mut object_content = Vec::new();
            object_content.extend_from_slice(header.as_bytes());
            object_content.extend_from_slice(&file_content);

            // Create hash from object
            let mut hasher = Sha1::new();
            hasher.update(&object_content);
            let hash = hasher.finalize();
            let hash_hex = format!("{:x}", hash);

            if write {
                // Compress object content with zlib
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(&object_content)
                    .context("Failed to compress object")?;
                let compressed = encoder.finish().context("Failed to finish compression")?;

                // Create directory structure
                let dir_path = format!(".git/objects/{}", &hash_hex[..2]);
                fs::create_dir_all(&dir_path)
                    .with_context(|| format!("Failed to create object directory: {}", dir_path))?;

                // Write compressed content to file
                let file_path = format!("{}/{}", dir_path, &hash_hex[2..]);
                fs::write(&file_path, compressed)
                    .with_context(|| format!("Failed to write object file: {}", file_path))?;
            }

            println!("{}", hash_hex);
        }
    }
    Ok(())
}
