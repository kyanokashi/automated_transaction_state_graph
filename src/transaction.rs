#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::automated_graph::ValidityCheck;
use crate::condition_rule::{ConditionRule, State};
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Transaction<'a> {
    id: Uuid,
    valid_crs: Vec<&'a ConditionRule>,
    invalid_crs: Vec<&'a ConditionRule>,
}

impl<'a> Transaction<'a> {
    pub fn new(valid_cr: Vec<&'a ConditionRule>, invalid_cr: Vec<&'a ConditionRule>) -> Self {
        Self {
            id: Uuid::new_v4(),
            valid_crs: valid_cr,
            invalid_crs: invalid_cr,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn valid_crs(&self) -> Vec<&ConditionRule> {
        self.clone().valid_crs
    }

    pub fn invalid_crs(&self) -> Vec<&ConditionRule> {
        self.clone().invalid_crs
    }

    pub fn subscribe_rule(&mut self, condition_rule: &'a ConditionRule) {
        match condition_rule.state() {
            _x if _x == State::Valid => self.valid_crs.push(condition_rule),
            _y if _y == State::Invalid => self.invalid_crs.push(condition_rule),
            _ => {}
        }
    }
}

impl<'a> ValidityCheck for Transaction<'a> {
    fn is_valid(&self) -> bool {
        self.invalid_crs.is_empty()
    }
}
