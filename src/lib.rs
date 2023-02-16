mod models;

pub use models::*;

mod client {

    use leaky_bucket_lite::LeakyBucket;

    pub struct Client {
        c: reqwest::Client,
        rate_limiter: LeakyBucket,
    }
}
