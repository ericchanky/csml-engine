use crate::data::{position::Position, Interval, Literal, primitive::{PrimitiveString, PrimitiveObject}};
use crate::error_format::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ArgsType {
    Named(HashMap<String, Literal>),
    Normal(HashMap<String, Literal>),
}

impl ArgsType {

    pub fn args_to_debug(&self, interval: Interval) -> Literal {
        match self {
            Self::Named(map)
            | Self::Normal(map) => {
                let mut obj = HashMap::new();

                let value = PrimitiveObject::get_literal(&map, interval);
                obj.insert("debug".to_owned(),
                    PrimitiveString::get_literal(
                        &value.primitive.to_string(),
                        value.interval
                    )
                );

                let mut lit = PrimitiveObject::get_literal(&obj, interval);
                lit.set_content_type("debug");

                lit
            }
        }
    }

    pub fn get<'a>(&'a self, key: &str, index: usize) -> Option<&'a Literal> {
        match self {
            Self::Named(var) => {
                match (var.get(key), index) {
                    (Some(val), _) => Some(val),
                    // tmp ?
                    (None, 0) => var.get(&format!("arg{}", index)),
                    (None, _) => None,
                }
            }
            Self::Normal(var) => var.get(&format!("arg{}", index)),
        }
    }

    pub fn populate(
        &self,
        map: &mut HashMap<String, Literal>,
        vec: &[&str],
        interval: Interval,
    ) -> Result<(), ErrorInfo> {
        match self {
            Self::Named(var) => {
                for (key, value) in var.iter() {
                    if !vec.contains(&(key as &str)) && key != "arg0" {
                        map.insert(key.to_owned(), value.to_owned());
                    }
                }
                Ok(())
            }
            Self::Normal(var) => {
                if vec.len() < var.len() {
                    //TODO:: error msg
                    Err(gen_error_info(
                        Position::new(interval),
                        "to many arguments".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn populate_json_to_literal(
        &self,
        map: &mut HashMap<String, Literal>,
        vec: &[serde_json::Value],
        interval: Interval,
    ) -> Result<(), ErrorInfo> {
        match self {
            Self::Named(var) => {
                for (key, value) in var.iter() {
                    let contains = vec.iter().find(|obj| {
                        if let Some(map) = obj.as_object() {
                            map.contains_key(key)
                        } else {
                            false
                        }
                    });

                    if let (None, true) = (contains, key != "arg0") {
                        map.insert(key.to_owned(), value.to_owned());
                    }
                }
                Ok(())
            }
            Self::Normal(var) => {
                if vec.len() < var.len() {
                    Err(gen_error_info(
                        Position::new(interval),
                        "to many arguments".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }
}
