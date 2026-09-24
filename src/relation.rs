use std::collections::HashSet;

#[derive(Clone)]
pub struct Relation {
    pub name: String,
    pub scheme: Vec<String>,
    pub tuples: HashSet<Vec<String>>
}

impl Relation {
    pub fn new(new_name: String, schema: Vec<String>) -> Self {
        Relation {
            name: new_name,
            scheme: schema,
            tuples: HashSet::<Vec<String>>::new()
        }
    }

    pub fn add_tuple(&mut self, tuple: Vec<String>) -> bool {
        return self.tuples.insert(tuple);
    }

    pub fn rename(&mut self, index: u32, value: String) -> Relation {
        let mut new_scheme = self.scheme.clone();
        new_scheme[index as usize] = value;
        let mut result = Relation { name: self.name.clone(), 
                                scheme: new_scheme.clone(),
                                tuples: HashSet::new() };
        for tuple in &self.tuples {
            result.add_tuple(tuple.clone());
        }
        return result;
    }

    pub fn join(&self, right: &Relation) -> Relation {
        let left = self;
        let mut new_scheme = left.scheme.clone();
        let mut common_scheme = false;

        for item in &right.scheme {
            if new_scheme.contains(item) {
                common_scheme = true;
            } else {
                new_scheme.push(item.clone());
            }
        }

        let new_name = format!("{} and {}", left.name, right.name);
    
        let mut result = Relation::new(new_name, new_scheme);

        for left_tuple in &left.tuples {
            for right_tuple in &right.tuples {
            
                if self.joinable(&left.scheme, &right.scheme, left_tuple, right_tuple) {
                    let new_tuple = self.join_tuple(&left.scheme, &right.scheme, left_tuple, right_tuple);
                    result.add_tuple(new_tuple);
                
                } else if !common_scheme {
                    let mut information = left_tuple.clone();
                    for item in right_tuple { 
                        information.push(item.clone());
                    }
                    result.add_tuple(information);
                } else {
                    continue;
                }
            }
        }

        result
    }

    pub fn joinable(&self, left_scheme: &[String], right_scheme: &[String], left_tuple: &Vec<String>,
                    right_tuple: &Vec<String>) -> bool {
        for i in 0..left_scheme.len() {
            for j in 0..right_scheme.len() {
                if left_scheme[i] == right_scheme[j] {
                    let left_val = &left_tuple[i];
                    let right_val = &right_tuple[j];
                    if left_val != right_val {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn join_tuple(&self, left_scheme: &[String], right_scheme: &[String], left_tuple: &Vec<String>,
                      right_tuple: &Vec<String>) -> Vec<String> {
        let mut temp = left_tuple.clone();
        for i in 0..right_scheme.len() {
            let mut duplicate = false;
            for j in 0..left_scheme.len() {
                if right_scheme[i] == left_scheme[j] {
                    duplicate = true;
                    break;
                }
            }
            if !duplicate {
                temp.push(right_tuple[i].clone());
            }
        }

        temp
    }

    pub fn project(&self, indices: &Vec<u32>) -> Relation {
        let mut new_scheme: Vec<String> = Vec::new();
        for &title_idx in indices {
            new_scheme.push(self.scheme[title_idx as usize].clone());
        }
        let mut result = Relation::new(self.name.clone(), new_scheme);
        for tuple in &self.tuples {
            let mut temp_tuple: Vec<String> = Vec::new();
            for &idx in indices {
                temp_tuple.push(tuple[idx as usize].clone());
            }
            result.add_tuple(temp_tuple);
        }

        result
    }

    pub fn select_by_value(&self, index: usize, value: &str) -> Relation {
        let mut result = Relation::new(self.name.clone(), self.scheme.clone());
        for tuple in &self.tuples {
            if tuple[index] == value {
                result.add_tuple(tuple.clone());
            }
        }
        result
    }

    pub fn select_by_matching_indices(&self, index1: usize, index2: usize) -> Relation {
        let mut result = Relation::new(self.name.clone(), self.scheme.clone());
        for tuple in &self.tuples {
            if tuple[index1] == tuple[index2] {
                result.add_tuple(tuple.clone());
            }
        }
        result
    }

    pub fn query_verification(&self, name: String, parameters: Vec<String>) {
        let param_string = parameters.join(",");
        if self.tuples.is_empty() {
            println!("{}({})? No", name, param_string);
        } else {
            println!("{}({})? Yes({})", name, param_string, self.tuples.len());
        }
    }

    pub fn to_string_list(&mut self) -> Vec<String> {
        let mut string_vector = Vec::<String>::new();
        for tuple in self.tuples.clone() {
            let formatted: Vec<String> = self.scheme.iter()
                .zip(tuple.iter())
                .map(|(col, val)| format!("{}={}", col, val))
                .collect();
            string_vector.push(formatted.join(", "));
        }
        return string_vector;
    }
}
