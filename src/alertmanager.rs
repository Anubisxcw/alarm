pub async fn get_alerts() {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:9093/api/v1/alerts");
}