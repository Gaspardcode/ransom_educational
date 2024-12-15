pub mod dummy {

    use std::fs::OpenOptions;
    use std::io;
    use std::fs::{self, DirEntry};
    use std::path::Path;
    use std::io::Write;

    // @brief performs a Cesar cypher of the file
    fn cyp(file:&DirEntry) -> io::Result<()> {
        let mut data = fs::read(file.path())?;
        for c in &mut data {
            *c -= 1;
        }
        fs::write(file.path(), data)?;
        println!("wrote to {:?}", file.file_name());
        Ok(())
    }
    // @brief Iterates over the files in the directory and applies the given func
    fn visit_dirs(dir: &Path, cyp: fn(&DirEntry) -> io::Result<()>) -> io::Result<()>{
        if dir.is_dir()  {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cyp);
                }
                else
                {
                    let _ = cyp(&entry);
                }
            }
        }
        Ok(())
    }
    // @brief dummy function
    fn create_pg(filename:&str) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(filename)?;
        f.write_all("Hello file !".as_bytes())?;
        Ok(())
    }

}
