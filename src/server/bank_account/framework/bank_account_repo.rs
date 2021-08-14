use cqrs_es2_sql::{
    get_cqrs,
    Cqrs,
};

use crate::cqrs::db_connection;

use super::super::{
    aggregate::BankAccount,
    queries::{
        BankAccountQueryRepository,
        SimpleLoggingQueryProcessor,
    },
};

pub fn bank_account_repo() -> Cqrs<BankAccount> {
    let simple_query = SimpleLoggingQueryProcessor {};

    let mut account_query_processor = BankAccountQueryRepository::new(
        "account_query",
        db_connection(),
    );

    account_query_processor
        .with_error_handler(Box::new(|e| println!("{}", e)));

    get_cqrs(
        db_connection(),
        vec![
            Box::new(simple_query),
            Box::new(account_query_processor),
        ],
    )
}
