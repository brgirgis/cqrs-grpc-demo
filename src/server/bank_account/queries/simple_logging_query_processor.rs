use cqrs_es2::{
    EventEnvelope,
    QueryProcessor,
};

use super::super::aggregate::BankAccount;

pub struct SimpleLoggingQueryProcessor {}

impl QueryProcessor<BankAccount> for SimpleLoggingQueryProcessor {
    fn dispatch(
        &mut self,
        aggregate_id: &str,
        events: &[EventEnvelope<BankAccount>],
    ) {
        for event in events {
            let payload =
                serde_json::to_string_pretty(&event.payload).unwrap();
            println!(
                "{}-{}\n{}",
                aggregate_id, event.sequence, payload
            );
        }
    }
}
