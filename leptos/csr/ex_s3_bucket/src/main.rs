use std::{env, process, time::Duration};

// use apps::App;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::{Config, Credentials},
    presigning::PresigningConfig,
    Client,
};
// use leptos::{create_resource, create_signal, mount_to_body, view, SignalGet};
// use leptos::{mount_to_body, view};

mod apps;

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("env is gone");
            process::exit(1);
        }
    };

    let access_key: String = match env::var("AWS_ACCESS_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e:#?}");
            process::exit(1);
        }
    };

    let access_secret_key: String = match env::var("AWS_SECRET_KEY") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("{e:#?}");
            process::exit(1);
        }
    };

    let credentials = Credentials::new(access_key, access_secret_key, None, None, "gurupro");
    let region = Region::new("ap-southeast-3");
    let s3_conf = Config::builder()
        .region(region)
        .credentials_provider(credentials)
        .behavior_version(BehaviorVersion::latest())
        .build();

    let client = Client::from_conf(s3_conf);

    let expires_in = Duration::from_secs(180);

    let presign_conf = match PresigningConfig::expires_in(expires_in) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("error crate presign conf {e:#?}");
            process::exit(1);
        }
    };

    let url = client
        .get_object()
        .bucket("gurupro")
        .key("rust.svg")
        .presigned(presign_conf)
        .await;

    match url {
        Ok(val) => {
            println!("{:#?}", val.uri())
        }
        Err(e) => eprintln!("{e:#?}"),
    }
}
