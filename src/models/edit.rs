/*
Copyright (c) 2022 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

use std::collections::HashMap;

use getset::Getters;
use serde_derive::Serialize;
use tree_sitter::Range;

use super::matches::Match;
use pyo3::prelude::pyclass;

#[derive(Serialize, Debug, Clone, Getters)]
#[pyclass]
pub(crate) struct Edit {
  // The match representing the target site of the edit
  #[pyo3(get)]
  #[get = "pub"]
  p_match: Match,
  // The string to replace the substring encompassed by the match
  #[pyo3(get)]
  #[get = "pub"]
  replacement_string: String,
  // The rule used for creating this match-replace
  #[pyo3(get)]
  #[get = "pub"]
  matched_rule: String,
}

impl Edit {
  pub(crate) fn new(p_match: Match, replacement_string: String, matched_rule: String) -> Self {
    Self {
      p_match,
      replacement_string,
      matched_rule,
    }
  }

  pub(crate) fn delete_range(replacement_range: Range) -> Self {
    Self {
      p_match: Match::new(replacement_range, HashMap::new()),
      replacement_string: String::new(),
      matched_rule: "Delete Range".to_string(),
    }
  }
}
