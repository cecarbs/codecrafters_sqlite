use anyhow::{bail, Result};
use std::fs::File;
use std::io::{prelude::*, SeekFrom};

fn main() -> Result<()> {
    // Parse arguments
    let args = std::env::args().collect::<Vec<_>>();
    match args.len() {
        0 | 1 => bail!("Missing <database path> and <command>"),
        2 => bail!("Missing <command>"),
        _ => {}
    }

    // Parse command and act accordingly
    let command = &args[2];
    match command.as_str() {
        ".dbinfo" => {
            let mut file = File::open(&args[1])?;
            let mut header = [0; 100];
            file.read_exact(&mut header)?;

            // The page size is stored at the 16th byte offset, using 2 bytes in big-endian order
            let page_size = u16::from_be_bytes([header[16], header[17]]);

            println!("database page size: {}", page_size);

            file.seek(SeekFrom::Start(0))?;

            let mut schema_information = vec![0; page_size as usize];

            // First 100 bytes are allocated for header
            let header_size: usize = 100;

            if let Err(e) = file.read_exact(&mut schema_information) {
                eprintln!("Error: {}", e);
            }

            // The two-byte integer at offset 3 gives the number of cells on the page (from docs)
            println!(
                "number of tables: {}",
                u16::from_be_bytes([
                    schema_information[header_size + 3],
                    schema_information[header_size + 4]
                ])
            );
        }
        _ => bail!("Missing or invalid command passed: {}", command),
    }

    Ok(())
}
