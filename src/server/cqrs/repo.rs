use postgres_es::{GenericQueryRepository, PostgresCqrs};

use super::db::db_connection;

type AccountQuery = GenericQueryRepository<BankAccountQuery, BankAccount>;

pub fn cqrs_repo() -> PostgresCqrs<BankAccount> {
  let simple_query = SimpleLoggingQueryProcessor {};
  let mut account_query_processor = AccountQuery::new("account_query", db_connection());
  account_query_processor.with_error_handler(Box::new(|e| println!("{}", e)));

  postgres_es::postgres_cqrs(
    db_connection(),
    vec![Box::new(simple_query), Box::new(account_query_processor)],
  )
}
