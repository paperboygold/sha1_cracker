/*
This is a program intended to crack sha1 hashes. It takes a hash as input and
then tries to crack it.

Usage: sha1_cracker <hash> <wordlist>

Example: sha1_cracker 1234567890123456789012345678901234567890 dictionary.txt
*/

use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// Declare the expected hash length of 40 characters for later comparison
const HASH_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    // Collect arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct and print usage if not
    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }
    
    // Check the length of the supplied hash against HASH_LENGTH
    let hash = args[2].trim();
    if hash.len() != HASH_LENGTH {
        return Err("Hash must be 40 characters long".into());
    }

    // Load wordlist into memory
    let wordlist = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist);

    // Iterate over wordlist
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        
        // If the hash is equal to the hash of the word, print the word 
        if hash == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    // If the above does not return Ok, then the password was not found
    println!("Password not found.");

    Ok(())
}

