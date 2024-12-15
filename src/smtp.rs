pub mod smtp {
    extern crate sendmail;
    use sendmail::email;

    // @brief sends a mail to a hard-encoded address
    fn send_test_email()  {
        // Configure email body and header
        // Send the email
        match email::send(
                // From Address
                "8753422f22@gmail.com",
                // To Address
                &["gaspard.torterat@gmail.com"],
                // Subject
                "Subject - Hello World!",
                // Body
                "<html><body><h1>I am the body. Hello Wolrd!<br/>
                <br/>And I accept html.</h1></body></html>"
                ) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => eprintln!("Could not send email: {:?}", e),
        }
    }

#[test]
    fn test_email_integration() {
        send_test_email();
    }
}
