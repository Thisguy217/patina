use crate::token::{TokenType, Token, token_to_string};
use crate::token::TokenType::*;
use std::collections::HashSet;
use std::string::String;
use std::fmt;

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.parameters.join(","))
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body_str: Vec<String> = self.body_predicates.iter().map(|p| p.to_string()).collect();
        write!(f, "{} :- {}.", self.head_predicate, body_str.join(","))
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Clone)]
pub struct Predicate {
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Clone)]
pub struct Rule {
    pub head_predicate: Predicate,
    pub body_predicates: Vec<Predicate>,
}

pub struct DatalogProgram {
    pub schemes: Vec<Predicate>,
    pub facts: Vec<Predicate>,
    pub rules: Vec<Rule>,
    pub queries: Vec<Predicate>,
    pub domain: HashSet<String>,
}

impl DatalogProgram {
    pub fn new() -> Self {
        Self {
            schemes: Vec::new(),
            facts: Vec::new(),
            rules: Vec::new(),
            queries: Vec::new(),
            domain: HashSet::new()
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.tokens[self.current].t_type.clone()
    }

    pub fn advance_token(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
    }

    pub fn throw_error(&self) {
        eprintln!("Error parsing at line {}", token_to_string(&self.tokens[self.current]));
        eprintln!("Got {:?}", self.token_type());
        std::process::exit(1);
    }

    pub fn r#match(&mut self, expected_token: TokenType) {
        if self.tokens[self.current].t_type == expected_token {
            //println!("match: {:#?}", self.tokens[self.current].t_type);
            self.advance_token();
        } else {
            self.throw_error();
        }   
    }

    pub fn parse(&mut self) -> DatalogProgram {
        let mut data: DatalogProgram = DatalogProgram::new();

        self.r#match(Schemes);
        self.r#match(Colon);
        self.scheme(&mut data);
        self.scheme_list(&mut data);
        self.r#match(Facts);
        self.r#match(Colon);
        self.fact_list(&mut data);
        self.r#match(Rules);
        self.r#match(Colon);
        self.rule_list(&mut data);
        self.r#match(Queries);
        self.r#match(Colon);
        self.query(&mut data);
        self.query_list(&mut data);
        self.r#match(End);
        
        return data;
    }

    pub fn scheme(&mut self, data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            let mut new_scheme = Predicate{ name: self.tokens[self.current].value.clone(), 
                                            parameters: Vec::new() };
            self.r#match(Id);
            self.r#match(LeftParen);
            new_scheme.parameters.push(self.tokens[self.current].value.clone());
            self.r#match(Id);
            self.id_list(&mut new_scheme);
            self.r#match(RightParen);
            data.schemes.push( new_scheme );
        }
    }

    pub fn fact(&mut self, data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            let mut new_fact = Predicate{ name: self.tokens[self.current].value.clone(),
                                          parameters: Vec::new() };
            self.r#match(Id);
            self.r#match(LeftParen);
            new_fact.parameters.push(self.tokens[self.current].value.clone());
            data.domain.insert(self.tokens[self.current].value.clone());
            self.r#match(r#String);
            self.string_list(&mut new_fact, data);
            self.r#match(RightParen);
            self.r#match(Period);
            data.facts.push( new_fact );
        }
    }

    pub fn query(&mut self, data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            let mut new_query = Predicate{ name: self.tokens[self.current].value.clone(),
                                           parameters: Vec::new() };
            self.predicate(&mut new_query);
            self.r#match(QMark);
            data.queries.push(new_query);
        }
    }

    pub fn scheme_list(&mut self, mut data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            self.scheme(&mut data);
            self.scheme_list(&mut data);
        }
    }

    pub fn fact_list(&mut self, mut data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            self.fact(&mut data);
            self.fact_list(&mut data);
        }
    }

    pub fn rule_list(&mut self, mut data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            self.rule(&mut data);
            self.rule_list(&mut data);
        }
    }

    pub fn query_list(&mut self, mut data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            self.query(&mut data);
            self.query_list(&mut data);
        }
    }

    pub fn id_list(&mut self, scheme: &mut Predicate) {
        if self.tokens[self.current].t_type == Comma {
            self.r#match(Comma);
            scheme.parameters.push(self.tokens[self.current].value.clone());
            self.r#match(Id);
            self.id_list(scheme);
        }
    }

    pub fn string_list(&mut self, fact: &mut Predicate, data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Comma {
            self.r#match(Comma);
            fact.parameters.push(self.tokens[self.current].value.clone());
            data.domain.insert(self.tokens[self.current].value.clone());
            self.r#match(r#String);
            self.string_list(fact, data);
        }
    }

    pub fn rule(&mut self, data: &mut DatalogProgram) {
        if self.tokens[self.current].t_type == Id {
            let mut new_rule = Rule{ head_predicate: self.head_predicate(),
                                     body_predicates: Vec::new()};
            self.r#match(ColonDash);
            self.predicate_rule(&mut new_rule);
            self.predicate_list(&mut new_rule);
            self.r#match(Period);
            data.rules.push(new_rule);
        }
    }

    pub fn head_predicate(&mut self) -> Predicate {
        let mut predicate = Predicate{ name: self.tokens[self.current].value.clone(),
                                       parameters: Vec::new() };
        self.r#match(Id);
        self.r#match(LeftParen);
        predicate.parameters.push(self.tokens[self.current].value.clone());
        self.r#match(Id);
        self.id_list(&mut predicate);
        self.r#match(RightParen);
        return predicate;
    }

    pub fn predicate_rule(&mut self, rule: &mut Rule) {
        if self.tokens[self.current].t_type == Id {
            let mut predicate = Predicate{ name: self.tokens[self.current].value.clone(),
                                           parameters: Vec::new() };
            self.r#match(Id);
            self.r#match(LeftParen);
            self.parameter(&mut predicate);
            self.parameter_list(&mut predicate);
            self.r#match(RightParen);
            rule.body_predicates.push(predicate);
        } else {
            self.throw_error();
        }
    }

    pub fn predicate(&mut self, mut predicate: &mut Predicate) {
        if self.tokens[self.current].t_type == Id {
            self.r#match(Id);
            self.r#match(LeftParen);
            self.parameter(&mut predicate);
            self.parameter_list(&mut predicate);
            self.r#match(RightParen);
        } else {
            self.throw_error();
        }
    }

    pub fn predicate_list(&mut self, mut rule: &mut Rule) {
        if self.tokens[self.current].t_type == Comma {
            self.r#match(Comma);
            self.predicate_rule(&mut rule);
            self.predicate_list(&mut rule);
        }
    }

    pub fn parameter(&mut self, predicate: &mut Predicate) {
        if self.tokens[self.current].t_type == r#String {
            predicate.parameters.push(self.tokens[self.current].value.clone());
            self.r#match(r#String);
        } else if self.tokens[self.current].t_type == Id {
            predicate.parameters.push(self.tokens[self.current].value.clone());
            self.r#match(Id);
        } else {
            self.throw_error();
        }
    }


    pub fn parameter_list(&mut self, mut predicate: &mut Predicate) {
        if self.tokens[self.current].t_type == Comma {
            self.r#match(Comma);
            self.parameter(&mut predicate);
            self.parameter_list(&mut predicate);
        }
    }
}
