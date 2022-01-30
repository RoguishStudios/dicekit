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

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

//!
//! Dice Toolkit
//!
//! This crate provides various tools used for developing Roleplaying or Roguelike games in rust.
//!
//! * Dice - Trait Implementations for the `rand` crate to emulate dice.
//! * WeightedTable - Weighted table for rolling percentile.
//!

mod dice;
mod table;

pub use crate::dice::Dice;
pub use crate::table::WeightedTable;
