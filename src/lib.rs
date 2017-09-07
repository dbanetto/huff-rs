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

#[derive(Debug, PartialEq, Eq)]
enum TreeNode<V: Debug + Eq> {
    Leaf(V),
    Node(Box<TreeNode<V>>, Box<TreeNode<V>>),
}

impl <V: Debug + Eq> TreeNode<V> {

    pub fn value(self) -> Option<V> {
        match self {
            TreeNode::Leaf(v) => Some(v),
            TreeNode::Node(_,_) => None,
        }
    }
    
    pub fn is_leaf(&self) -> bool {
        match self {
            &TreeNode::Leaf(_) => true,
            &TreeNode::Node(_,_) => false,
        }
    }

    pub fn left(&self) -> Option<&TreeNode<V>> {
        match self {
            &TreeNode::Leaf(_) => None,
            &TreeNode::Node(ref l,_) => Some(l.deref()),
        }
    }

    pub fn right(&self) -> Option<&TreeNode<V>> {
        match self {
            &TreeNode::Leaf(_) => None,
            &TreeNode::Node(_,ref r) => Some(r.deref()),
        }
    }

    pub fn new_leaf(value: V) -> Self {
        TreeNode::Leaf(value)
    }

    pub fn new_node(left: Self, right: Self) -> Self {
        TreeNode::Node(Box::new(left), Box::new(right))
    }
}

#[derive(Debug)]
struct TreeBuilder<V: Debug + Eq, W: PartialOrd> { 
    nodes: Vec<(V, W)>
}

impl <V: Debug + Eq, W: PartialOrd + Add<Output=W>> TreeBuilder<V, W> {
    
    pub fn new() -> Self {
        TreeBuilder {
            nodes: vec![],
        }
    }

    pub fn add(mut self, sym: V, weight: W) -> Self {
        self.nodes.push((sym, weight));
        self
    }

    pub fn build(mut self) -> Option<TreeNode<V>> {
        use std::cmp::Ordering;

        self.nodes.sort_by(|a, b| if b.1 > a.1 {
            Ordering::Greater    
        } else if b.1 < a.1 {
            Ordering::Less    
        } else {
            Ordering::Equal    
        });

        let mut nodes: Vec<(TreeNode<V>, W)> = self.nodes.into_iter()
            .map(|(v, w)| (TreeNode::new_leaf(v), w)).collect();


        while nodes.len() > 1 {
            let (right_value, right_weight) = nodes.pop().unwrap();
            let (left_value, left_weight) = nodes.pop().unwrap();
            
            let new_weight = left_weight + right_weight;

            let node = TreeNode::new_node(left_value, right_value);

            let pos = {
                match nodes.binary_search_by(|a| if new_weight > a.1 {
                    Ordering::Greater    
                } else if new_weight < a.1 {
                    Ordering::Less    
                } else {
                    Ordering::Equal    
                }) {
                    Ok(i) => i,
                    Err(i) => i,
                }
            };
            nodes.insert(pos, (node, new_weight));
        }

       
        match nodes.pop() {
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
            .add('d', 10)
            .build()
            .unwrap();

        let expected = TreeNode::new_node(
            TreeNode::new_leaf('d'),
            TreeNode::new_node(
                TreeNode::new_leaf('b'),
                TreeNode::new_leaf('a')));

        
        assert_eq!(expected, tree);
    }
}
