#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(clippy::new_without_default)]

use crate::automated_graph::ValidityCheck;
use crate::condition_rule::{ConditionRule, State};
use crate::transaction::Transaction;
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ATSGraph<'a> {
    valid_txs: Vec<Transaction<'a>>,
    invalid_txs: Vec<Transaction<'a>>,
    valid_crs: ConditionRuleDS<'a>,
    invalid_crs: ConditionRuleDS<'a>,
}

#[derive(Debug, Clone)]
pub struct ConditionRuleDS<'a> {
    state: State,
    cr_map: HashMap<Uuid, Vec<Transaction<'a>>>,
    cr_vec: Vec<ConditionRule>,
}

#[derive(Debug, Error)]
#[error("The states seem to be conflicting")]
pub struct ErrorStateMismatch;

impl<'a> ConditionRuleDS<'a> {
    fn new(state: State) -> Self {
        Self {
            state,
            cr_map: HashMap::new(),
            cr_vec: Vec::new(),
        }
    }

    fn add_condition_rule(
        &mut self,
        condition_rule: &ConditionRule,
    ) -> Result<(), ErrorStateMismatch> {
        if (*condition_rule).state() != self.state {
            return Err(ErrorStateMismatch);
        }

        let txs = self.cr_map.get(&(*condition_rule).id());

        if txs.is_none() {
            self.cr_map.insert((*condition_rule).id(), Vec::new());
        }

        Ok(())
    }
}

impl<'a> ATSGraph<'a> {
    pub fn new() -> Self {
        Self {
            valid_txs: Vec::new(),
            invalid_txs: Vec::new(),
            valid_crs: ConditionRuleDS::new(State::Valid),
            invalid_crs: ConditionRuleDS::new(State::Invalid),
        }
    }

    pub fn valid_txs(&self) -> Vec<Transaction> {
        self.clone().valid_txs
    }

    pub fn invalid_txs(&self) -> Vec<Transaction> {
        self.clone().invalid_txs
    }

    pub fn valid_crs(&self) -> ConditionRuleDS {
        self.clone().valid_crs
    }

    pub fn invalid_crs(&self) -> ConditionRuleDS {
        self.clone().invalid_crs
    }

    pub fn add_condition_rule(&mut self, condition_rule: &ConditionRule) {
        match condition_rule.is_valid() {
            true => self.valid_crs.add_condition_rule(condition_rule).unwrap(),
            false => self.invalid_crs.add_condition_rule(condition_rule).unwrap(),
        }
    }

    /// Adds a transaction to the graph
    pub fn add_transaction(&mut self, transaction: &Transaction<'a>) {
        for condition_rule in (*transaction).valid_crs() {
            self.add_condition_rule(condition_rule)
        }
        for condition_rule in (*transaction).invalid_crs() {
            self.add_condition_rule(condition_rule)
        }

        if (*transaction).is_valid() {
            self.valid_txs.push((*transaction).clone())
        }
    }
}
