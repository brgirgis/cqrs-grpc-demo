use postgres::{
    Client,
    NoTls,
};

pub fn db_connection() -> Client {
    Client::connect(
        "postgresql://demo_user:demo_pass@localhost:5434/demo",
        NoTls,
    )
    .unwrap()
}
