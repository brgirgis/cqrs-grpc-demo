use cqrs_es2::{
    memory_store::EventStore as MemoryStore,
    CqrsFramework,
};

use crate::bank_account::{
    BankAccount,
    BankAccountCommand,
    DepositMoney,
    SimpleLoggingQueryProcessor,
};

#[test]
fn test_event_store_single_command() {
    let event_store = MemoryStore::<BankAccount>::default();
    let query = SimpleLoggingQueryProcessor {};
    let mut cqrs =
        CqrsFramework::new(event_store, vec![Box::new(query)]);
    cqrs.execute(
        "test_id",
        BankAccountCommand::DepositMoney(DepositMoney {
            amount: 1000_f64,
        }),
    )
    .unwrap()
}
