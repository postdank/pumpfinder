use ed25519_dalek::{Keypair, PublicKey};
use rand::rngs::OsRng;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    let vanity_suffix = "pump";  // Suffix we're looking for
    let output_dir = "vanity_addresses"; // Folder to store found addresses

    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let count = AtomicU64::new(0); // Atomic counter for addresses tried

    loop {
        (0..num_cpus::get())
            .into_par_iter()
            .for_each(|_| {
                let mut csprng = OsRng{};
                let keypair = Keypair::generate(&mut csprng);
                let public_key = keypair.public;
                let address = bs58::encode(public_key.as_bytes()).into_string();

                if address.ends_with(vanity_suffix) {
                    // Serialize directly to Vec<u8> instead of using a struct
                    let keypair_bytes = keypair.to_bytes().to_vec();

                    // Write keypair to file
                    let file_name = format!("{}/{}.json", output_dir, address);
                    write_keypair_to_file(&file_name, &keypair_bytes).expect("Failed to write keypair to file");
                    
                    // Print vanity address and current count
                    println!("Found vanity address '{}', Total addresses tried: {}", address, count.load(Ordering::Relaxed));
                }

                // Increment counter
                count.fetch_add(1, Ordering::Relaxed);
            });

        // Print the count periodically
        if count.load(Ordering::Relaxed) % 10000 == 0 {
            println!("Addresses tried: {}", count.load(Ordering::Relaxed));
        }
    }
}

fn write_keypair_to_file(file_name: &str, keypair_bytes: &[u8]) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(file_name)?;
    serde_json::to_writer(&mut file, keypair_bytes)?;
    Ok(())
}