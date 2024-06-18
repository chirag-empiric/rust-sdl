use serde::{Deserialize, Serialize};
use std::io::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RopeNode {
    Leaf {
        val: String,
        weight: usize,
    },
    Internal {
        left: Box<RopeNode>,
        right: Box<RopeNode>,
        weight: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rope {
    root: RopeNode,
}

impl Rope {
    pub fn new(val: &str) -> Result<Self, Error> {
        let root = RopeNode::Leaf {
            val: val.to_string(),
            weight: val.len(),
        };

        Ok(Self { root })
    }

    pub fn concate(left: Rope, right: Rope) -> Result<Self, Error> {
        let left_wt = left.weight();
        let right_wt = right.weight();

        println!("weight: {:?}", left_wt + right_wt);

        let new_root = Self {
            root: RopeNode::Internal {
                left: Box::new(left.root),
                right: Box::new(right.root),
                weight: left_wt,
            },
        };

        Ok(new_root)
    }

    pub fn at_index(&self, index: usize) -> Option<char> {
        let answer = Rope::findat(&self, &self.root, index).unwrap();
        return Some(answer);
    }

    pub fn findat(&self, node: &RopeNode, index: usize) -> Option<char> {
        match node {
            RopeNode::Leaf { val, .. } => {
                let ans = val.chars().nth(index);
                ans
            }
            RopeNode::Internal {
                left,
                right,
                weight,
            } => {
                if index < *weight {
                    let answer = self.findat(&left, index);
                    answer
                } else {
                    let answer = self.findat(&right, index - *weight);
                    answer
                }
            }
        }
    }

    fn weight(&self) -> usize {
        match &self.root {
            RopeNode::Leaf { val, .. } => val.len(),
            RopeNode::Internal { weight, .. } => *weight,
        }
    }
}
