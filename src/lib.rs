pub mod condition_rule;
pub mod transaction;
pub mod transaction_state_graph;

pub mod automated_graph {
    pub trait ValidityCheck {
        fn is_valid(&self) -> bool;
    }
}
