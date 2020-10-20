use std::env;
use std::str::FromStr;
use warp::Filter;

const DEFAULT_PORT: u16 = 5000;

#[tokio::main]
async fn main() {
    let port = env::var_os("ECHO_PORT")
        .map(|val| {
            val.into_string()
                .map(|s| u16::from_str(&s).unwrap_or(DEFAULT_PORT))
                .unwrap_or(DEFAULT_PORT)
        })
        .unwrap_or(DEFAULT_PORT);

    warp::serve(warp::any().map(|| "Hello World!"))
        .run(([0, 0, 0, 0], port))
        .await;
}
