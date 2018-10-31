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
    let repo_name = "web-client";

    let az_acr_output = Command::new("az")
        .arg("acr")
        .arg("repository")
        .arg("show-manifests")
        .arg("--name")
        .arg("astrumu")
        .arg("--repository")
        .arg(repo_name)
        .arg("--orderby")
        .arg("time_asc")
        .output()
        .expect("failed to execute process");

    let image_info_json_array = String::from_utf8_lossy(&az_acr_output.stdout);

    let current_timestamp: DateTime<Utc> = Utc::now();

    let image_info_array: Vec<ImageInfo> =
        serde_json::from_str(&image_info_json_array).unwrap();

    'outer : for image_info in image_info_array.iter() {

        for tag in image_info.tags.iter() {
            if tag == "stable" || tag == "latest" || tag.len() < 10 {
                continue 'outer;
            }
        }
        let image_duration = current_timestamp.signed_duration_since(image_info.timestamp);
        let store_days = Duration::days(30);

        if image_duration < store_days {
            continue;
        }

        // az acr repository delete --name <acrName> --image <repositoryName>@<digest>
        
        let repo_digest = format!("{repository}@{digest}", repository=repo_name, digest=image_info.digest);

        println!("deleting {:?}", image_info);

        let az_acr_delete_output = Command::new("az")
        .arg("acr")
        .arg("repository")
        .arg("delete")
        .arg("--name")
        .arg("astrumu")
        .arg("--image")
        .arg(repo_digest)
        .arg("--yes")
        .output()
        .expect("failed to execute process");
        
}
}
