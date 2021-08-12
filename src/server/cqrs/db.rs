use postgres::{Connection, TlsMode};

pub fn db_connection() -> Connection {
  Connection::connect(
    "postgresql://demo_user:demo_pass@localhost:5432/demo",
    TlsMode::None,
  )
  .unwrap()
}
