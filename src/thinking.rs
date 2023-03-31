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

struct Piranha {
    seed_workflow: SeedWorkflow,
    cleanup_workflows: Vec<CleanupWorkflow>,
}

impl Piranha {
    fn new(seed_piranha_argument: PiranhaArguments, seed_workflow: Vec<SeedWorkflow>, cleanup_workflow: Vec<CleanupWorkflow>) -> Self {
        Self{
            seed_workflows,
            cleanup_workflows,
            seed_piranha_argument
        }
    }

    fn apply(&mut self) {
        let piranha_output_summary = seed_workflow.apply(seed_piranha_argument);
        
        loop {
            let mut cleanup_performed = false;
            for cleanup_workflow in self.cleanup_workflows {
                let cleanup_output_summary = cleanup_workflow.apply(piranha_output_summary);
                if cleanup_output_summary {
                    piranha_output_summary.merge(cleanup_output_summary);
                    cleanup_performed = true;
                }
            }
            if !cleanup_performed {
                break;
            }
        }
        
    }
}

struct SeedWorkflow {
    seed_step: Step // (subset is initialized)
}

impl SeedWorkflow {
    fn apply(&mut self) -> Vec<PiranhaOutputSummary>{
        execute_piranha(&self.seed_step)
    }
}


struct CleanupWorkflow {
    previous_edit: Map<String, Vec<(String, String)>> // path -> (old_content, new_content)
}

impl CleanupWorkflow {

    fn apply_at_path(){
        // Apply the cleanup workflow on a single file
        // Let's say our temporary limitation is that we can only apply the cleanup workflow on a single file
    }


    fn is_applicable(&self) -> bool {
        /// Check if the previous edit makes this cleanup workflow applicable
        /// This is the fun part :) 
        /// 
        /// **Delete unused private Property** : 
        ///  (i) [Step] Get all private properties and their usages before and after the change 
        ///  (ii) [Rust/Python logic based on Step 1 output] If a private property was used before the change and is not used after the change, then it is a candidate for deletion
        ///  (iii) [Step] Delete the property
        /// 
        /// **Inline variable (basic)** :
        /// Our current approach works well for inline because we know during the time of the editing code, which exact variable is candidate for inline 
        /// Now we have to infer it from the previous edit
        /// 
        /// (i) [Step] For each method in old code (before the change), get all the local variable declarations (2/3 step graphs)
        /// (ii) [Step] For each method in new code (after the change), get all the local variable declarations (2/3 step graphs)
        /// (iii) [Rust/Python logic based on Step 1 and 2 output] If the initializer for a variable was changed from some expression 
        ///      to a boolean constant, then it is a candidate for inline
        /// (iv) [Step] Inline the variable (Two step : Delete specified variable > Replace all usages of the variable with the boolean literal)
        ///      Boolean/simplification will auto trigger since we will provide it as built in cleanup rules 
        /// (v) In languages like Go, we cannot simply replace al the variable references, because they have a patter on using same variable name and intializing it again 
        ///      (e.g. disabled, err = GetBoolanProperty("disabled") , they will use `err` again and again). 
        ///     For these languages we can have a intermediate step for more accurately inferring the usages of the variable
        /// 
        /// 
        /// Similarly for : 
        ///  (i) Inline property/private method/variable
        ///  (ii) Delete unused private method/property/local variable
        ///  (iii) Delete unused parameter ? 
        ///  (iv) Inline arguments 
        ///  (v) Delete unused classes within the package? (useful for plugin cleanup)
        /// 
    }

    fn apply(&mut self) {
        for (path, (old, new)) in self.previous_edit {
            if self.is_applicable() {
                self.apply_at_path();
            }
        }
    }
}

struct Step {
    piranha_arguments: PiranhaArguments, // RuleGraph
    summaries: Vec<PiranhaOutputSummary>,
}

impl Step {
    fn new(piranha_arguments: PiranhaArguments) -> Self {
        Self{
            piranha_arguments,
            summaries: Vec::new(),
        } 
    }

    fn apply(&mut self) -> Vec<PiranhaOutputSummary> {
        self.summaries = execute_piranha(&self.piranha_arguments);
    }
}
