#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate chrono;
extern crate time;

use std::process::Command;

use chrono::{DateTime, Utc};
use time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct ImageInfo {
    digest: String,
    tags: Vec<String>,
    timestamp: DateTime<Utc>,
}

fn main() {
    let repo_name = "api-server";

    let az_acr_output = Command::new("az")
        .arg("acr")
        .arg("repository")
        .arg("show-manifests")
        .arg("--name")
        .arg("astrumu")
        .arg("--repository")
        .arg(repo_name)
        .output()
        .expect("failed to execute process");

    let image_info_json_array = String::from_utf8_lossy(&az_acr_output.stdout);

    let current_timestamp: DateTime<Utc> = Utc::now();

    let image_info_array: Vec<ImageInfo> =
        serde_json::from_str(&image_info_json_array).unwrap();
    for image_info in image_info_array.iter() {
        image_info.tags

        let image_duration = current_timestamp.signed_duration_since(image_info.timestamp);
        let store_days = Duration::days(30);

        if image_duration > store_days {
            println!("{}", image_duration);
            println!("{:?}", image_info.timestamp);
        }
    }
}
