use std::collections::HashSet;

use crate::parser::{Predicate, DatalogProgram, Rule};
use crate::database::Database;
use crate::relation::Relation;
use crate::graph::Graph;

pub struct Interpreter {
    schemes: Vec<Predicate>,
    facts: Vec<Predicate>,
    rules: Vec<Rule>,
    queries: Vec<Predicate>,
    database: Database,
}

impl Interpreter {
    pub fn new(data: DatalogProgram) -> Self {
        Interpreter {
            schemes: data.schemes,
            facts: data.facts,
            rules: data.rules,
            queries: data.queries,
            database: Database::new()
        }
    }

    pub fn run_interpreter(&mut self) {
        self.make_schemes();
        self.make_facts();
        println!("Dependency Graph");
        let components: Vec<HashSet<u32>> = self.generate_graph();
        println!("Rule Evaluation");
        self.evaluate_rules(components);
        println!("Query Evaluation");
        self.generate_queries();       
    }

    pub fn make_schemes(&mut self) {
        for i in &self.schemes {
            let name = i.name.clone();
            let scheme = i.parameters.clone();
            let result = Relation::new(name, scheme);
            self.database.insert_relation(result);
        }
    }

    pub fn make_facts(&mut self) {
        for i in &self.facts {
            let name = i.name.clone();
            let fact_items = i.parameters.clone();
            self.database.insert_fact(name, fact_items);
        }
    }

    pub fn generate_graph(&mut self) -> Vec<HashSet<u32>> {
        let mut graph: Graph = Graph::new(self.rules.clone());
        let mut reverse_graph: Graph = Graph::new(self.rules.clone());
        for (from_id, from_rule) in self.rules.iter().enumerate() {
            for body_predicate in &from_rule.body_predicates {
                for (to_id, to_rule) in self.rules.iter().enumerate() {
                    if body_predicate.name == to_rule.head_predicate.name {
                        graph.add_edge(from_id as u32, to_id as u32);
                        reverse_graph.add_edge(to_id as u32, from_id as u32);
                    }
                }
            }
        }

        println!("{}", graph);

        let input = reverse_graph.dfs_forest();
        graph.dfs_forest_og(input)
    }

    pub fn evaluate_rules(&mut self, new_order: Vec<HashSet<u32>>) {
        let mut graph: Graph = Graph::new(self.rules.clone());
        for (from_id, from_rule) in self.rules.iter().enumerate() {
            for body_predicate in &from_rule.body_predicates {
                for (to_id, to_rule) in self.rules.iter().enumerate() {
                    if body_predicate.name == to_rule.head_predicate.name {
                        graph.add_edge(from_id as u32, to_id as u32);
                    }
                }
            }
        }
        for scc in new_order.iter() {
            let first_item = scc.iter().next().copied().unwrap_or(0);
            let dependent = graph.dependent(first_item); 
            if scc.len() == 1 && !dependent {
                for item in scc {
                    println!("SCC: R{}", item);
                
                    let rule = &self.rules[*item as usize];
                    println!("{}", rule);
                    let head = &rule.head_predicate;
                    let body = &rule.body_predicates;
                
                    let new_input = self.evaluate_rule(head.clone(), body.clone());
                    self.database.unionize(new_input);
                
                    println!("1 passes: R{}", item);
                }
            } else if scc.len() > 1 || dependent {
                let mut number = 0;
                let mut fixed = false;

                let mut scc_sorted: Vec<u32> = scc.iter().cloned().collect();
                scc_sorted.sort();
                let scc_list: Vec<String> = scc_sorted.iter().map(|id| format!("R{}", id)).collect();
                println!("SCC: {}", scc_list.join(","));

                while !fixed {
                    let before_size = self.database.size_before(); // Stub

                    for item in &scc_sorted {
                        let rule = &self.rules[*item as usize];
                        println!("{}", rule);
                        let head = &rule.head_predicate; // Stub
                        let body = &rule.body_predicates; // Stub
                        
                        let new_input = self.evaluate_rule(head.clone(), body.clone()); // Stub
                        self.database.unionize(new_input);             // Stub
                    }

                    fixed = self.database.check_size(before_size); // Stub
                    number += 1;
                }
    
                println!("{} passes: {}", number, scc_list.join(","));
            }
            //println!();
        }
        println!();
    }

    pub fn evaluate_rule(&mut self, head: Predicate, body: Vec<Predicate>) -> Relation {
        let project_params = head.parameters;
        let mut indices: Vec<u32> = Vec::new();
        let mut name = &body[0].name;
        let mut params = &body[0].parameters;
        let mut intermediate = self.database.query_rule(name.clone(), params.clone());
        if body.len() > 1 {
            for i in 1..body.len() {
                name = &body[i].name;
                params = &body[i].parameters;
                let mut temp = self.database.query_rule(name.clone(), params.clone());
                for j in 0..body[i].parameters.len() {
                    temp = temp.rename(j as u32, body[i].parameters[j].clone());
                }
                intermediate = intermediate.join(&temp);
            }
        }
        
        for i in 0..project_params.len() {
            for j in 0..intermediate.scheme.len() {
                if intermediate.scheme[j] == project_params[i] {
                    indices.push(j as u32);
                    break;
                }
            }
        }
        intermediate = intermediate.project(&indices);
        for i in 0..project_params.len() {
            intermediate = intermediate.rename(i as u32, project_params[i].clone());
        }
        intermediate.name = head.name;
        
        intermediate
    }

    pub fn generate_queries(&mut self) {
	    for query in &self.queries {
		    let name = query.name.clone();
		    let query_params = &query.parameters;
		    self.database.query(name, query_params.to_vec());
	    }
    }
}
