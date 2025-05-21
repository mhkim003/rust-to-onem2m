use crate::model::ContentInstance;
use reqwest::Client;
use serde_json::json;

pub async fn notify_subscribers(
    ae_id: &str,
    cnt_id: &str,
    cin: &ContentInstance,
    endpoints: &[String],
) {
    let client = Client::new();

    let body = json!({
        "m2m:sgn": {
            "sur": format!("/csebase/{}/{}", ae_id, cnt_id),
            "nev": {
                "rep": {
                    "m2m:cin": cin
                },
                "net": [3]
            }
        }
    });

    for url in endpoints {
        let url = url.clone();
        let body = body.clone();
        let client = client.clone();

        tokio::spawn(async move {
            match client.post(&url).json(&body).send().await {
                Ok(res) => println!("✅ Notified {} ({})", url, res.status()),
                Err(err) => println!("❌ Failed to notify {}: {:?}", url, err),
            }
        });
    }
}