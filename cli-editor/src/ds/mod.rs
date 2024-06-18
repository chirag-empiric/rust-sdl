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

    pub fn traverse(&self) -> Result<(), Error> {
        self.read_recursive(&self.root);
        Ok(())
    }

    pub fn read_recursive(&self, node: &RopeNode) {
        match node {
            RopeNode::Leaf { val, .. } => {
                print!("{}", val);
            }

            RopeNode::Internal { left, right, .. } => {
                self.read_recursive(&left);
                self.read_recursive(&right);
            }
        }
    }

    pub fn split_from_idx(&self, index: usize) -> (Option<RopeNode>, Option<RopeNode>) {
        println!("index: {}", index);
        let (left, right) = self.split_recursive(&self.root, index);

        (Some(left.unwrap()), Some(right.unwrap()))
    }

    fn split_recursive(
        &self,
        node: &RopeNode,
        index: usize,
    ) -> (Option<RopeNode>, Option<RopeNode>) {
        match node {
            RopeNode::Leaf { val, .. } => {
                let left = RopeNode::Leaf {
                    val: val[..index].to_string(),
                    weight: val[..index].to_string().len(),
                };
                let right = RopeNode::Leaf {
                    val: val[index..].to_string(),
                    weight: val[index..].to_string().len(),
                };

                println!(
                    "value in left: {:?} \n in right: {:?}",
                    val[..index].to_string(),
                    val[index..].to_string()
                );
                (Some(left), Some(right))
            }
            RopeNode::Internal {
                left,
                right,
                weight,
            } => {
                //
                if index < *weight {
                    let (l, r) = self.split_recursive(&left, index);
                    let new_right = RopeNode::Internal {
                        left: Box::new(r.unwrap()),
                        right: right.clone(),
                        weight: weight - index,
                    };

                    (l, Some(new_right))
                } else {
                    //
                    let (l, r) = self.split_recursive(&right, index - weight);
                    let new_left = RopeNode::Internal {
                        left: left.clone(),
                        right: Box::new(l.unwrap()),
                        weight: index - weight,
                    };

                    (Some(new_left), r)
                }
            }
        }
    }
}
