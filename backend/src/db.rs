use deadpool_postgres::{ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::{Config, NoTls};

pub fn create_pool(db_url: &str) -> Pool {
    let mut cfg: Config = db_url.parse().expect("Invalid DATABASE_URL");

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = deadpool_postgres::Manager::from_config(cfg, NoTls, mgr_config);
    Pool::builder(mgr)
        .max_size(16)
        .runtime(Runtime::Tokio1)
        .build()
        .expect("Failed to create Postgres pool")
}