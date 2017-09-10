extern crate bitstream;

use bitstream::{BitWriter, BitReader, NoPadding};

use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::ops::Add;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub enum TreeNode<V: Eq> {
    Leaf(V),
    Node(Box<TreeNode<V>>, Box<TreeNode<V>>),
}

impl <V: Eq> TreeNode<V> {

    fn value<'a>(self) -> Option<V> {
        match self {
            TreeNode::Leaf(v) => Some(v),
            TreeNode::Node(_,_) => None,
        }
    }
    
    fn is_leaf(&self) -> bool {
        match self {
            &TreeNode::Leaf(_) => true,
            &TreeNode::Node(_,_) => false,
        }
    }

    pub fn new_leaf(value: V) -> Self {
        TreeNode::Leaf(value)
    }

    pub fn new_node(left: Self, right: Self) -> Self {
        TreeNode::Node(Box::new(left), Box::new(right))
    }

}

impl <V: Eq + Hash> TreeNode<V> {

    pub fn encoding(self) -> HashMap<V, Vec<bool>> {
        let trail: Vec<bool> = vec![];
        let mut map = HashMap::new();

        self.build_map(trail, &mut map);

        map
    }

    fn build_map(self, trail: Vec<bool>, map :&mut HashMap<V, Vec<bool>>) {
        match self {
            TreeNode::Leaf(v) => {
                map.insert(v, trail.clone());
            },
            TreeNode::Node(l, r) => {
                //handle left
                let mut left = trail.clone();
                left.push(false);
                l.build_map(left, map);
                //handle right
                let mut right = trail.clone();
                right.push(true);
                r.build_map(right, map);
            },
        }
    }
}

pub struct TreeBuilder<V: Eq, W: PartialOrd + Add<Output=W>> { 
    nodes: Vec<(V, W)>
}

impl <V: Eq, W: PartialOrd + Add<Output=W>> TreeBuilder<V, W> {
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

struct HuffWriter<V: Eq + Hash, W: Write> {
    encoding: HashMap<V, Vec<bool>>,
    writer: BitWriter<W, NoPadding>,
}

impl <V: Eq + Hash, W: Write> HuffWriter<V, W> {
    
    pub fn new(tree : TreeNode<V>, writer: W) -> Self {
        HuffWriter {
            encoding: tree.encoding(),
            writer: BitWriter::new(writer),
        }
    }

    pub fn write(&mut self, value: &V) -> std::io::Result<()> {
        let bits: &Vec<bool> = match self.encoding.get(value) {
           Some(bits) => bits, 
           None => { return Err(Error::from(ErrorKind::InvalidInput)); },
        };
        for bit in bits {
            self.writer.write_bit(*bit)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn build_simple_tree() {
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

    #[test]
    fn build_flat_tree() {
        let tree = TreeBuilder::<char, u32>::new()
            .add('a', 1)
            .add('b', 1)
            .add('c', 1)
            .add('d', 1)
            .build()
            .unwrap();

        let expected = TreeNode::new_node(
            TreeNode::new_node(
                TreeNode::new_leaf('a'),
                TreeNode::new_leaf('b')),
            TreeNode::new_node(
                TreeNode::new_leaf('c'),
                TreeNode::new_leaf('d')));

        assert_eq!(expected, tree);
    }

    #[test]
    fn encoding_map() {
        let tree = TreeBuilder::<char, u32>::new()
            .add('a', 1)
            .add('b', 1)
            .add('c', 1)
            .add('d', 1)
            .build()
            .unwrap();
        
        let mut expected = HashMap::new();
        expected.insert('a', vec![false, false]);
        expected.insert('b', vec![false, true]);
        expected.insert('c', vec![true, false]);
        expected.insert('d', vec![true, true]);

        assert_eq!(expected, tree.encoding());
    }
    
    #[test]
    fn encode() {
        let tree = TreeBuilder::<char, u32>::new()
            .add('a', 1)
            .add('b', 1)
            .add('c', 1)
            .add('d', 1)
            .build()
            .unwrap();

        let mut output: Vec<u8> = vec![];
        {
            let mut writer = HuffWriter::new(tree, &mut output);

            for value in vec!['a', 'b', 'c', 'd', 'a'] {
                writer.write(&value).unwrap();
            }
        }
        
        let expected = vec![0b_00011011, 0b_00000000];

        assert_eq!(expected, output);
    }

    #[test]
    fn encode_value_error() {
        let tree = TreeBuilder::<char, u32>::new()
            .add('a', 1)
            .build()
            .unwrap();

        let mut output: Vec<u8> = vec![];
        {
            let mut writer = HuffWriter::new(tree, &mut output);

            for value in vec!['b'] {
                match writer.write(&value) {
                    Ok(_) => panic!(),
                    Err(_) => (),
                }
            }
        }
    }
}
