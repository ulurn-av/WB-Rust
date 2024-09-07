#[warn(non_snake_case)]

use L0::run;

#[tokio::main]
async fn main() {
    run().await;
}
