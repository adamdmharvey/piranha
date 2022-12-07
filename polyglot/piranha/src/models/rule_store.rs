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

use std::{collections::HashMap, path::Path};

use colored::Colorize;
use getset::Getters;
use log::{debug, info, trace};
use tree_sitter::{Language, Query};

use crate::{
  models::piranha_arguments::{PiranhaArguments, PiranhaArgumentsBuilder},
  models::{
    rule::Rule,
    rule_graph::RuleGraph,
    scopes::{ScopeGenerator, ScopeQueryGenerator},
  },
  utilities::{read_toml, MapOfVec},
};

use super::{
  language::PiranhaLanguage,
  outgoing_edges::{Edges, OutgoingEdges},
  rule::Rules,
};

pub(crate) static GLOBAL: &str = "Global";
pub(crate) static PARENT: &str = "Parent";
/// This maintains the state for Piranha.
#[derive(Debug, Getters)]
pub(crate) struct RuleStore {
  // A graph that captures the flow amongst the rules
  rule_graph: RuleGraph,
  // Caches the compiled tree-sitter queries.
  rule_query_cache: HashMap<String, Query>,
  // All the input rules stored by name
  rules_by_name: HashMap<String, Rule>,
  // Current global rules to be applied.
  #[get = "pub"]
  global_rules: Vec<Rule>,
  // Scope generators.
  scopes: Vec<ScopeGenerator>,
  // Command line arguments passed to piranha
  #[get = "pub"]
  piranha_args: PiranhaArguments,
  // Command line arguments passed to piranha
  #[get = "pub"]
  global_tags: HashMap<String, String>,
  /// Tree-sitter language model
  #[get = "pub"]
  language: Language,
}

impl RuleStore {
  pub(crate) fn new(args: &PiranhaArguments) -> RuleStore {
    let (rules, edges, scopes) = read_config_files(args);
    let rule_graph = RuleGraph::new(&edges, &rules);
    let mut rule_store = RuleStore {
      rule_graph,
      rules_by_name: rules.iter().map(|r| (r.name(), r.clone())).collect(),
      scopes,
      piranha_args: args.clone(),
      language: *args.piranha_language().language(),
      ..Default::default()
    };

    for (_, rule) in rule_store.rules_by_name.clone() {
      if rule.is_seed_rule() {
        rule_store.add_to_global_rules(&rule, args.input_substitutions());
      }
    }
    info!(
      "Number of rules and edges loaded : {:?}",
      rule_store.rule_graph.get_number_of_rules_and_edges()
    );
    trace!("Rule Store {}", format!("{:#?}", rule_store));
    rule_store
  }

  #[cfg(test)]
  pub(crate) fn default_with_scopes(scopes: Vec<ScopeGenerator>) -> RuleStore {
    RuleStore {
      scopes,
      ..Default::default()
    }
  }

  pub(crate) fn default_substitutions(&self) -> HashMap<String, String> {
    let mut default_subs = self.piranha_args.input_substitutions().clone();
    default_subs.extend(self.global_tags().clone());
    default_subs
  }

  /// Add a new global rule, along with grep heuristics (If it doesn't already exist)
  pub(crate) fn add_to_global_rules(
    &mut self, rule: &Rule, tag_captures: &HashMap<String, String>,
  ) {
    if let Ok(r) = rule.try_instantiate(tag_captures) {
      if !self.global_rules.contains(&r){
        #[rustfmt::skip]
        debug!("{}", format!("Added Global Rule - \n {}", r).bright_blue());
        self.global_rules.push(r);
      }
    }
  }

  /// Get the compiled query for the `query_str` from the cache
  /// else compile it, add it to the cache and return it.
  pub(crate) fn query(&mut self, query_str: &String) -> &Query {
    self
      .rule_query_cache
      .entry(query_str.to_string())
      .or_insert_with(|| {
        self
          .piranha_args
          .piranha_language()
          .create_query(query_str.to_string())
      })
  }

  /// Get the next rules to be applied grouped by the scope in which they should be performed.
  pub(crate) fn get_next(
    &self, rule_name: &String, tag_matches: &HashMap<String, String>,
  ) -> HashMap<String, Vec<Rule>> {
    // let rule_name = rule.name();
    let mut next_rules: HashMap<String, Vec<Rule>> = HashMap::new();
    // Iterate over each entry (Edge) in the adjacency list corresponding to `rule_name`
    for (scope, to_rule) in self.rule_graph.get_neighbors(rule_name) {
      let to_rule_name = &self.rules_by_name[&to_rule];
      // If the to_rule_name is a dummy rule, skip it and rather return it's next rules.
      if to_rule_name.is_dummy() {
        // Call this method recursively on the dummy node
        for (next_next_rules_scope, next_next_rules) in
          self.get_next(&to_rule_name.name(), tag_matches)
        {
          for next_next_rule in next_next_rules {
            // Group the next rules based on the scope
            next_rules.collect(
              String::from(&next_next_rules_scope),
              next_next_rule.instantiate(tag_matches),
            )
          }
        }
      } else {
        // Group the next rules based on the scope
        next_rules.collect(String::from(&scope), to_rule_name.instantiate(tag_matches));
      }
    }
    // Add empty entry, incase no next rule was found for a particular scope
    for scope in [PARENT, GLOBAL] {
      next_rules.entry(scope.to_string()).or_default();
    }
    next_rules
  }

  // For the given scope level, get the ScopeQueryGenerator from the `scope_config.toml` file
  pub(crate) fn get_scope_query_generators(&self, scope_level: &str) -> Vec<ScopeQueryGenerator> {
    self
      .scopes
      .iter()
      .find(|level| level.name().eq(scope_level))
      .map(|scope| scope.rules().to_vec())
      .unwrap_or_else(Vec::new)
  }

  pub(crate) fn add_global_tags(&mut self, new_entries: &HashMap<String, String>) {
    let global_substitutions: HashMap<String, String> = new_entries
      .iter()
      .filter(|e| e.0.starts_with(self.piranha_args.global_tag_prefix()))
      .map(|(a, b)| (a.to_string(), b.to_string()))
      .collect();
    let _ = &self.global_tags.extend(global_substitutions);
  }
}

impl Default for RuleStore {
  fn default() -> Self {
    RuleStore {
      rule_graph: RuleGraph::default(),
      rule_query_cache: HashMap::default(),
      rules_by_name: HashMap::default(),
      global_rules: Vec::default(),
      piranha_args: PiranhaArgumentsBuilder::default().build().unwrap(),
      scopes: Vec::default(),
      global_tags: HashMap::default(),
      language: *PiranhaLanguage::default().language(),
    }
  }
}

fn read_config_files(
  args: &PiranhaArguments,
) -> (Vec<Rule>, Vec<OutgoingEdges>, Vec<ScopeGenerator>) {
  let path_to_config = Path::new(args.path_to_configurations());
  // Read the language specific cleanup rules and edges
  let language_rules: Rules = args.piranha_language().rules().clone().unwrap_or_default();
  let language_edges: Edges = args.piranha_language().edges().clone().unwrap_or_default();
  let scopes = args.piranha_language().scopes().to_vec();

  // Read the API specific cleanup rules and edges
  let mut input_rules: Rules = read_toml(&path_to_config.join("rules.toml"), true);
  let input_edges: Edges = read_toml(&path_to_config.join("edges.toml"), true);

  for r in input_rules.rules.iter_mut() {
    r.add_to_seed_rules_group();
  }

  let all_rules = [language_rules.rules, input_rules.rules].concat();
  let all_edges = [language_edges.edges, input_edges.edges].concat();

  (all_rules, all_edges, scopes)
}
