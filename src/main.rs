use flate2::read::ZlibDecoder;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => {
            // Get hash from command line
            let blob_sha = &args[3];
            // Split the hash in two parts to get correct path (two first characters is the folder name, next is file name)
            let splitted_sha = blob_sha.split_at(2);
            let string_path = format!(".git/objects/{}/{}", splitted_sha.0, splitted_sha.1);
            let path = Path::new(&string_path);

            // Read and decompress blob file content
            let bytes = fs::read(path).unwrap();
            let mut z = ZlibDecoder::new(&bytes[..]);
            let mut blob_content = String::new();
            z.read_to_string(&mut blob_content).unwrap();
            // Remove header from blob content, get only real file content by splitting from the null byte
            let file_content: &str = blob_content.split("\0").collect::<Vec<_>>()[1];
            print!("{}", file_content.trim_end());
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
