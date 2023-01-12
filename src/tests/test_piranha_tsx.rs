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

use super::{create_match_test, initialize};
use crate::execute_piranha;
use crate::models::{default_configs::TSX, piranha_arguments::PiranhaArguments};

fn match_only_find_fors() -> PiranhaArguments {
  PiranhaArguments::new(
    TSX,
    "test-resources/tsx/structural_find/find_jsx_elements/input/",
    "test-resources/tsx/structural_find/find_jsx_elements/configurations",
  )
}

fn match_find_props_identifiers_within_b_jsx_elements() -> PiranhaArguments {
  PiranhaArguments::new(
    TSX,
    "test-resources/tsx/structural_find/find_props_identifiers_within_b_jsx_elements/input/",
    "test-resources/tsx/structural_find/find_props_identifiers_within_b_jsx_elements/configurations",
  )
}

fn find_props_identifiers_within_variable_declarators_not_within_divs() -> PiranhaArguments {
  PiranhaArguments::new(
    TSX,
    "test-resources/tsx/structural_find/find_props_identifiers_within_variable_declarators_not_within_divs/input/",
    "test-resources/tsx/structural_find/find_props_identifiers_within_variable_declarators_not_within_divs/configurations",
  )
}

create_match_test! {
  test_ts_match_only_find_fors:  match_only_find_fors(), 4,
  test_match_find_props_identifiers_within_b_jsx_elements: match_find_props_identifiers_within_b_jsx_elements(), 2,
  test_find_props_identifiers_within_variable_declarators_not_within_divs: find_props_identifiers_within_variable_declarators_not_within_divs(), 2,
}
