use std::ops::{Deref, Add};
use std::default::Default;
use std::fmt::Debug;

/// Encodes a given string with a given string
fn encode(input: &str, symbols :String) -> String  {
    return input.to_owned();
}

/// Decodes the given input with the given tree
fn decode(input: &str, symbols :String) -> String  {
    return input.to_owned();
}

#[derive(Debug)]
struct TreeNode<V: Debug> {
    value: Option<V>,
    left: Option<Box<TreeNode<V>>>,
    right: Option<Box<TreeNode<V>>>,
}

impl <V: Debug> TreeNode<V> {

    pub fn value(self) -> Option<V> {
        self.value 
    }
    
    pub fn is_leaf(&self) -> bool {
        self.value.is_some()
    }

    pub fn left(&self) -> Option<&TreeNode<V>> {
       match self.left {
           Some(ref l) => Some(l.deref()),
           None => None,
       }
    }

    pub fn right(&self) -> Option<&TreeNode<V>> {
       match self.right {
           Some(ref r) => Some(r.deref()),
           None => None,
       }
    }

    pub fn new_leaf(value: V) -> Self {
        TreeNode {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    pub fn new_node(left: Self, right: Self) -> Self {
        TreeNode {
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

#[derive(Debug)]
struct TreeBuilder<V: Debug, W: PartialOrd> { 
    nodes: Vec<(TreeNode<V>, W)>
}

impl <V: Debug> TreeBuilder<V, u32> {
    
    pub fn new() -> Self {
        TreeBuilder {
            nodes: vec![],
        }
    }

    pub fn add(mut self, sym: V, weight: u32) -> Self {
        self.nodes.push((TreeNode::new_leaf(sym), weight));
        self
    }

    pub fn build(mut self) -> Option<TreeNode<V>> {
        
        // decending
        self.nodes.sort_by(|a,b| b.1.cmp(&a.1));


        while self.nodes.len() > 1 {
            println!("{:?}", self);
            let (right_value, right_weight) = self.nodes.pop().unwrap();
            let (left_value, left_weight) = self.nodes.pop().unwrap();
            
            let new_weight = left_weight + right_weight;

            let node = TreeNode::new_node(left_value, right_value);

            let pos = {
                match self.nodes.binary_search_by(|a| new_weight.cmp(&a.1)) {
                    Ok(i) => i,
                    Err(i) => i,
                }
            };
            self.nodes.insert(pos, (node, new_weight));
        }

       
        match self.nodes.pop() {
            Some((val, _)) => Some(val),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_message() {
        let message = "Hello World".to_owned();
        let key = "aabbcc".to_owned();
        assert_eq!(message, encode(&message, key));
    }

    #[test]
    fn decode_message() {
        let message = "Hello World".to_owned();
        let key = "aabbcc".to_owned();
        assert_eq!(message, decode(&message, key));
    }
    
    #[test]
    fn build_tree() {
        
        let tree = TreeBuilder::<char, u32>::new()
            .add('a', 1)
            .add('b', 2)
            .add('c', 4)
            .add('d', 10)
            .build();
        
        println!("{:?}", tree);

        panic!();
    }
}
