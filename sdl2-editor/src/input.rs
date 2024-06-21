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

    pub fn add_trailing(current: Rope, val: &str) -> Result<Self, Error> {
        let new_rope = Self {
            root: RopeNode::Leaf {
                val: val.to_string(),
                weight: val.len(),
            },
        };

        Rope::concate(current, new_rope)
    }

    pub fn delete_last(&self) -> Result<Self, Error> {
        let len = self.traverse().unwrap().len();
        println!("val is: {}", self.traverse().unwrap());
        println!("length is: {}", len);
        // getting index out of bound here
        // self.delete_between_index(1, 3).unwrap();
        Ok(Rope {
            root: RopeNode::Leaf {
                val: ":)".to_string(),
                weight: 0,
            },
        })
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

    pub fn traverse(&self) -> Result<String, Error> {
        let mut val = String::new();
        let answer = self.read_recursive(&self.root, &mut val);
        Ok(answer)
    }

    pub fn read_recursive(&self, node: &RopeNode, data: &mut String) -> String {
        match node {
            RopeNode::Leaf { val, .. } => {
                data.push_str(&val);
                data.to_string()
            }

            RopeNode::Internal { left, right, .. } => {
                self.read_recursive(&left, data);
                self.read_recursive(&right, data)
            }
        }
    }

    pub fn split_from_idx(&self, index: usize) -> (Option<RopeNode>, Option<RopeNode>) {
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

    pub fn insert_at_index(&self, val: &str, index: usize) -> Result<Rope, Error> {
        let (leftmost, rightmost) = self.split_from_idx(index);
        let middle = Rope::new(val).unwrap();

        let left_rope = Rope {
            root: leftmost.unwrap(),
        };
        let right_rope = Rope {
            root: rightmost.unwrap(),
        };

        let rp1 = Rope::concate(left_rope, middle).unwrap();
        let rp2 = Rope::concate(rp1, right_rope);

        Ok(rp2.unwrap())
    }

    pub fn delete_between_index(&self, start: usize, end: usize) -> Result<Rope, Error> {
        println!("here");
        let (r1, _r2) = self.split_from_idx(start);
        let (_r3, r4) = self.split_from_idx(end);

        let new_left = Rope { root: r1.unwrap() };
        let new_right = Rope { root: r4.unwrap() };

        let ans = Rope::concate(new_left, new_right);

        Ok(ans.unwrap())
    }
}
