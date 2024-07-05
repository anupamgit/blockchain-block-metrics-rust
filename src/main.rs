mod display;
mod moralis;
mod transform;
use display::display_metrics_table;
use moralis::MoralisClient;
use transform::{transform_block_to_metrics, Metrics};

use dotenv::dotenv;
use env_logger;
use futures::future::join_all;
use log::error;
use std::env;
use tokio::task;
use warp::Filter;

const BLOCKCHAINS: [&str; 10] = [
    "eth",
    "polygon",
    "bsc",
    "avalanche",
    "fantom",
    "arbitrum",
    "cronos",
    "chiliz",
    "base",
    "optimism",
];

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a number");

    let moralis_client = MoralisClient::new();

    let metrics_route = warp::path("metrics").and(warp::get()).and_then(move || {
        let moralis_client = moralis_client.clone();
        async move {
            let fetches: Vec<_> = BLOCKCHAINS
                .iter()
                .map(|blockchain| {
                    let moralis_client = moralis_client.clone();
                    task::spawn(
                        async move { fetch_and_transform(moralis_client, blockchain).await },
                    )
                })
                .collect();

            let results = join_all(fetches).await;

            let metrics: Vec<_> = results
                .into_iter()
                .filter_map(|res| res.ok().and_then(|inner_res| inner_res.ok()))
                .collect();

            let table = display_metrics_table(metrics);
            Ok::<_, warp::Rejection>(warp::reply::html(table))
        }
    });

    println!("Server running at http://127.0.0.1:{}/metrics", port);

    warp::serve(metrics_route).run(([127, 0, 0, 1], port)).await;
}

async fn fetch_and_transform(
    moralis_client: MoralisClient,
    blockchain: &str,
) -> Result<Metrics, String> {
    match moralis_client.fetch_latest_block(blockchain).await {
        Ok(block) => {
            let metric = transform_block_to_metrics(blockchain, &block);
            Ok(metric)
        }
        Err(err) => {
            error!("Error fetching block for {}: {:?}", blockchain, err);
            Err(format!(
                "Error fetching block for {}: {:?}",
                blockchain, err
            ))
        }
    }
}
