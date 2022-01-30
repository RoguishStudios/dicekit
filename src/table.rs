//
// Copyright 2021 Hans W. Uhlig. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//


use std::collections::HashMap;
use std::hash::Hash;
use rand::distributions::{Distribution, WeightedError, WeightedIndex};
use rand::RngCore;

/// `WeightedTable` is a class for generating weighted tables for roleplaying easily.
pub struct WeightedTable<T>
    where T: Clone + Eq + Hash
{
    weights: Vec<f64>,
    values: Vec<T>,
    lookup: HashMap<T, usize>,
    state: TableState,
}

enum TableState {
    Clean(WeightedIndex<f64>),
    Dirty,
}

impl<T> WeightedTable<T>
    where T: Clone + Eq + Hash
{
    /// Create a new Weighted Table
    pub fn new() -> WeightedTable<T> {
        WeightedTable {
            weights: Vec::default(),
            values: Vec::default(),
            lookup: HashMap::default(),
            state: TableState::Dirty,
        }
    }
    /// Create a new Weighted Table
    pub fn with_data(data: &[(T, f64)]) -> Result<WeightedTable<T>, WeightedError> {
        let weights: Vec<f64> = data.iter().map(|(_t, v)| *v).collect();
        let values: Vec<T> = data.iter().map(|(t, _v)| t.clone()).collect();
        let lookup: HashMap<T, usize> = {
            let mut table = HashMap::new();
            values.iter().enumerate().for_each(|(idx, t)| {
                table.insert(t.clone(), idx).unwrap();
            });
            table
        };
        let state = TableState::Clean(WeightedIndex::new(&weights)?);
        Ok(WeightedTable { weights, values, lookup, state })
    }
    /// Update or insert value
    pub fn insert(&mut self, value: T, weight: f64) {
        if let Some(idx) = self.lookup.get(&value) {
            self.weights[*idx] = weight;
        } else {
            let idx = self.weights.len();
            self.lookup.insert(value.clone(), idx);
            self.weights.push(weight);
            self.values.push(value);
        }
        self.state = TableState::Dirty;
    }
    /// Update but dont insert if not present.
    pub fn update(&mut self, value: T, weight: f64) {
        if let Some(idx) = self.lookup.get(&value) {
            self.weights[*idx] = weight;
        }
        self.state = TableState::Dirty;
    }
    /// Remove an entry from the table.
    pub fn remove(&mut self, value: T) {
        if let Some(idx) = self.lookup.get(&value) {
            let idx = *idx;
            self.lookup.remove(&value);
            self.weights.remove(idx);
            self.values.remove(idx);
        }
        self.state = TableState::Dirty;
    }
    /// Roll percentile dice against weighted Table
    pub fn roll<R: RngCore>(&mut self, rng: &mut R) -> Result<&T, WeightedError> {
        if let TableState::Clean(ref index) = &self.state {
            let idx = index.sample(rng);
            Ok(&self.values[idx])
        } else {
            let index = WeightedIndex::new(&self.weights)?;
            let idx = index.sample(rng);
            self.state = TableState::Clean(index);
            Ok(&self.values[idx])
        }
    }
}