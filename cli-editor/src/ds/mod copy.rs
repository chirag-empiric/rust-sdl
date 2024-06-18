use serde::{Deserialize, Serialize};

use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    value: Option<String>,
    weight: usize,
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
}

impl Node {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            value: None,
            weight: 0,
            left: Box::new(None),
            right: Box::new(None),
        })
    }
    pub fn add_left(&mut self, value: String) -> Result<Self, Error> {
        let left = Self {
            value: Some(value.clone()),
            weight: value.len(),
            left: Box::new(None),
            right: Box::new(None),
        };

        self.weight = value.len();
        self.left = Box::new(Some(left));

        // println!("Node is: {:?}", self);

        Ok(Self {
            value: Some(value.clone()),
            weight: value.len(),
            left: Box::new(None),
            right: Box::new(None),
        })
    }

    pub fn add_right(&mut self, value: String) -> Result<Self, Error> {
        let right = Self {
            value: Some(value.clone()),
            weight: value.len(),
            left: Box::new(None),
            right: Box::new(None),
        };

        self.weight += value.len();
        self.right = Box::new(Some(right));

        // println!("Node is: {:?}", self);

        Ok(Self {
            value: Some(value.clone()),
            weight: value.len(),
            left: Box::new(None),
            right: Box::new(None),
        })
    }
}
