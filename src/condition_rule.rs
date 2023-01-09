#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::automated_graph::ValidityCheck;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct ConditionRule {
    id: Uuid,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Valid,
    Invalid,
}

impl ConditionRule {
    pub fn new(state: State) -> Self {
        Self {
            id: Uuid::new_v4(),
            state,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn state(&self) -> State {
        self.state
    }
}

impl ValidityCheck for ConditionRule {
    fn is_valid(&self) -> bool {
        self.state == State::Valid
    }
}
