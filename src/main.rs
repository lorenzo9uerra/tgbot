use std::env;
use teloxide::prelude2::*;
use teloxide::types::InputFile;
pub mod lib;
use lib::error::Error;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
    chat_id: i64,
}

#[tokio::main]
async fn main() {
    run().await.unwrap_or_else(|error| error.exit());
}

async fn run() -> Result<(), Error> {
    let f = std::fs::File::open("tgconfig.yml").expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    teloxide::enable_logging!();
    log::info!("Starting uploading...");
    let args: Vec<String> = env::args().collect();

    let bot = Bot::new(scrape_config.token);
    if args.len() != 3 {
        log::error!("You didn't provide 2 arguments");
        println!("\nUsage: ./tgbot \"/path/to/video.mp4\" \"caption of the video\"");
    } else {
        bot.send_video(scrape_config.chat_id, InputFile::file(&args[1]))
            .caption(&args[2])
            .send()
            .await?;
        log::info!("Upload successful");
    }
    Ok(())
}
