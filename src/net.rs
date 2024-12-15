pub mod net {
    use reqwest::Client;
    use serde_json::json;

    pub async fn post_data(key:&str, ip:&str) -> Result<(), reqwest::Error> {
        let client = Client::new();

        let payload = json!({
                "key": key,
                "ip": ip
                });

        let url = "https://gaspardcode.github.io/tools/end-point/index.html";
        let _response = client.post(url)
            .header("x-api-key", "fce33fb8-6816-4c16-a94a-0215e3b47dca")
            .json(&payload)
            .send()
            .await?;
        Ok(())
    }
}
