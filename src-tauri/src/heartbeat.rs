use beiboot_desktop::connection::ConnectError;

pub async fn establish_heartbeat_connection(cluster_id: &str, token: &str, getdeck_url: &str) -> Result<String, ConnectError> {
    let resp = reqwest::Client::new()
        .post(format!("{}/clusters/{}/heartbeat", getdeck_url, cluster_id).as_str())
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send");
    Ok(format!("Heartbeat status: {}", resp.status()))
}

#[cfg(test)]
mod tests {
    use super::establish_heartbeat_connection;

    #[tokio::test]
    async fn test_establish_heartbeat_connection_success() {
        let mut server = mockito::Server::new();
        let url = server.url();

        let mock = server.mock("POST", "/clusters/123/heartbeat")
        .with_status(200)
        .create();

        let result = establish_heartbeat_connection("123", "token", &url).await;
        mock.assert();
        assert_eq!("Heartbeat status: 200 OK", result.unwrap())
    }

    #[tokio::test]
    async fn test_establish_heartbeat_connection_failure() {
        let mut server = mockito::Server::new();
        let url = server.url();

        let mock = server.mock("POST", "/clusters/123/heartbeat")
        .with_status(500)
        .create();

        let result = establish_heartbeat_connection("123", "token", &url).await;
        mock.assert();
        assert_eq!("Heartbeat status: 500 Internal Server Error", result.unwrap())
    }
}
