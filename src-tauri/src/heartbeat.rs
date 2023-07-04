use beiboot_desktop::connection::ConnectError;

pub async fn establish_heartbeat_connection(cluster_id: &str, token: &str) -> Result<String, ConnectError> {
    let resp = reqwest::Client::new()
        .post(format!("https://api.getdeck.dev/clusters/{}/heartbeat", cluster_id).as_str())
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send");
    Ok(format!("Heartbeat status: {}", resp.status()))
}

