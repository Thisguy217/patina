use std::collections::HashMap;
use crate::relation::Relation;

pub struct Database {
    database: HashMap<String, Relation>
}

impl Database {
    pub fn new() -> Self {
        Database {
            database: HashMap::new()
        }
    }

    pub fn insert_relation(&mut self, result: Relation) {
        self.database.insert(result.name.clone(), result);
    }

    pub fn insert_fact(&mut self, name: String, tuples: Vec<String>) {
        self.database.entry(name).and_modify(|name| { name.add_tuple(tuples); });
    }

    pub fn query_rule(&mut self, name: String, parameters: Vec<String>) -> Relation {
        let mut result: Relation = self.database.get(&name).unwrap().clone();
        let mut variables = HashMap::<String, u32>::new();
        let mut variable_name = Vec::<String>::new();
        let mut variable_index = Vec::<u32>::new();
        for i in 0..parameters.len() {
            if parameters[i].starts_with('\'') {
                result = result.select_by_value(i, &parameters[i]);
            } else {
                let param_name = &parameters[i];
                if variables.contains_key(param_name) {
                    let first_seen_index = *variables.get(param_name).unwrap();
                    result = result.select_by_matching_indices(first_seen_index as usize, i as usize);
                } else {
                    variables.insert(param_name.clone(), i as u32);
                    variable_name.push(param_name.clone());
                    variable_index.push(i as u32);
                }
            }
        }       
        result = result.project(&variable_index);
        for i in 0..variable_name.len() {
            result = result.rename(i as u32, variable_name[i].clone());
        }
        return result;
    }

    pub fn unionize(&mut self, new_relation: Relation) {
        let stored_scheme = self.database.get(&new_relation.name).unwrap().scheme.clone();
        let mut tuples: Vec<Vec<String>> = new_relation.tuples.into_iter().collect();
        tuples.sort_by(|a, b| a.cmp(&b));
        for tuple in tuples {
            let added: bool;
            added = self.database.get_mut(&new_relation.name).unwrap().add_tuple(tuple.clone());
            if added {
                let formatted: Vec<String> = stored_scheme.iter()
                    .zip(tuple.iter())
                    .map(|(col, val)| format!("{}={}", col, val))
                    .collect();
                println!("  {}", formatted.join(", "));
            }
        }
    }

    pub fn size_before(&mut self) -> u32 {
        let mut final_num: u32 = 0;
        let keys: Vec<String> = self.database.keys().cloned().collect();
        for key in keys {
            final_num = final_num + self.database.get(&key).unwrap().tuples.len() as u32;
        }
        final_num
    }

    pub fn check_size(&mut self, before_size: u32) -> bool {
        let mut after_size: u32 = 0;
        let keys: Vec<String> = self.database.keys().cloned().collect();
        for key in keys {
            after_size = after_size + self.database.get(&key).unwrap().tuples.len() as u32;
        }
        if after_size - before_size == 0 {
            return true;
        }
        false
    }

    pub fn query(&mut self, name: String, parameters: Vec<String>) {
        let mut result: Relation = self.database.get(&name).unwrap().clone();
        
        let mut variables = HashMap::<String, usize>::new();
        let mut variable_name = Vec::<String>::new();
        let mut variable_index = Vec::<u32>::new();

        for i in 0..parameters.len() {
            let param = &parameters[i];
            if param.starts_with('\'') {
                result = result.select_by_value(i, param);
            } else {
                if let Some(&first_seen_index) = variables.get(param) {
                    result = result.select_by_matching_indices(first_seen_index, i);
                } else {
                    variables.insert(param.clone(), i);
                    variable_name.push(param.clone());
                    variable_index.push(i as u32);
                }
            }
        }

        result = result.project(&variable_index);

        for i in 0..variable_name.len() {
            result = result.rename(i as u32, variable_name[i].clone());
        }

        result.query_verification(name.clone(), parameters.clone());

        let mut strings: Vec<String> = result.to_string_list();
        strings.sort();

        if !strings.is_empty() {
            for item in &strings {
                if !item.is_empty() {
                    println!("  {}", item);
                }
            }
        }	    
    }
}
