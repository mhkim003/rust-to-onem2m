use crate::model::ContentInstance;
use reqwest::Client;
use serde_json::json;

pub async fn notify_subscribers(
    ae_id: &str,
    cnt_id: &str,
    cin: &ContentInstance,
    endpoints: &[String],
) {
    let Client = Client::new();

    let body = json!({
        "m2m:sgn": {
            "nev": {
                "rep": {
                    "m2m:cin": cin
                }
            }
        }
    });

    for url in endpoints {
        let _ = Client.post(url)
            .json(&body)
            .send()
            .await;
    }
}