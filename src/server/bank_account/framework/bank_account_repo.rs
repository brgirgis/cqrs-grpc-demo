use postgres_es2::{
    GenericQueryRepository,
    PostgresCqrs,
};

use crate::cqrs::db_connection;

use super::super::{
    aggregate::BankAccount,
    queries::{
        BankAccountQuery,
        SimpleLoggingQueryProcessor,
    },
};

type AccountQuery =
    GenericQueryRepository<BankAccountQuery, BankAccount>;

pub fn bank_account_repo() -> PostgresCqrs<BankAccount> {
    let simple_query = SimpleLoggingQueryProcessor {};

    let mut account_query_processor =
        AccountQuery::new("account_query", db_connection());

    account_query_processor
        .with_error_handler(Box::new(|e| println!("{}", e)));

    postgres_es2::postgres_cqrs(
        db_connection(),
        vec![
            Box::new(simple_query),
            Box::new(account_query_processor),
        ],
    )
}
