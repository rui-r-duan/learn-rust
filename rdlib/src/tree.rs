use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub left: Link,
    pub right: Link,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn build_tree_from_leetcode_str(s: &str) -> Link {
    // Remove brackets and split by commas
    let s = s.trim();
    if s.len() <= 2 || s == "[]" || s == "null" {
        return None;
    }
    let s = &s[1..s.len() - 1]; // Remove [ and ]
    let tokens: Vec<&str> = s.split(',').map(|s| s.trim()).collect();

    // Convert tokens to Option<i32>
    let mut nodes: Vec<Option<i32>> = Vec::new();
    for val in tokens {
        if val == "null" {
            nodes.push(None);
        } else {
            match val.parse::<i32>() {
                Ok(num) => nodes.push(Some(num)),
                Err(_) => return None, // Invalid number
            }
        }
    }

    if nodes.is_empty() || nodes[0].is_none() {
        return None;
    }

    // Create root node
    let root = Some(Rc::new(RefCell::new(Node::new(nodes[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone().unwrap());
    let mut i = 1; // Index for remaining nodes
    while !queue.is_empty() && i < nodes.len() {
        let current = queue.pop_front().unwrap();
        let mut current_borrow = current.borrow_mut();
        // Assign left child
        if i < nodes.len() {
            if let Some(val) = nodes[i] {
                let new_node = Some(Rc::new(RefCell::new(Node::new(val))));
                current_borrow.left = new_node.clone();
                queue.push_back(new_node.unwrap());
            }
            i += 1;
        }
        // Assign right child
        if i < nodes.len() {
            if let Some(val) = nodes[i] {
                let new_node = Some(Rc::new(RefCell::new(Node::new(val))));
                current_borrow.right = new_node.clone();
                queue.push_back(new_node.unwrap());
            }
            i += 1;
        }
    }

    root
}

pub fn binary_tree_to_string_lisp_style(root: &Link) -> String {
    match root {
        None => format!("nil"),
        Some(node) => {
            let borrowed_node = node.borrow();
            format!(
                "({} {} {})",
                borrowed_node.val,
                binary_tree_to_string_lisp_style(&borrowed_node.left),
                binary_tree_to_string_lisp_style(&borrowed_node.right)
            )
        }
    }
}

pub fn binary_tree_to_string_leetcode_style(root: &Link) -> String {
    fn collect_nodes_in_level_order(root: &Link) -> Vec<Link> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            if current.is_none() {
                result.push(None);
            } else {
                result.push(current.clone());
                queue.push_back(current.as_ref().unwrap().borrow().left.clone());
                queue.push_back(current.as_ref().unwrap().borrow().right.clone());
            }
        }
        result
    }

    if root.is_none() {
        "[]".to_string()
    } else {
        let tokens: Vec<String> = collect_nodes_in_level_order(root)
            .iter()
            .map(|x| match x {
                None => "null".to_string(),
                Some(node) => node.borrow().val.to_string(),
            })
            .collect();
        assert!(tokens.len() > 0);

        // Trim all the trailing "null"s.
        let mut i = tokens.len() - 1;
        while tokens[i] == "null" {
            i -= 1;
        }
        format!("[{}]", tokens[0..(i + 1)].join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_leetcode_in_str_lisp_out() {
        let result = build_tree_from_leetcode_str("null");
        assert_eq!(binary_tree_to_string_lisp_style(&result), "nil");

        let result = build_tree_from_leetcode_str("[]");
        assert_eq!(binary_tree_to_string_lisp_style(&result), "nil");

        let result = build_tree_from_leetcode_str("[null]");
        assert_eq!(binary_tree_to_string_lisp_style(&result), "nil");

        let result = build_tree_from_leetcode_str("[10]");
        assert_eq!(binary_tree_to_string_lisp_style(&result), "(10 nil nil)");

        let result = build_tree_from_leetcode_str("[3,9,20,null,null,15,7]");
        assert_eq!(
            binary_tree_to_string_lisp_style(&result),
            "(3 (9 nil nil) (20 (15 nil nil) (7 nil nil)))"
        );

        let result = build_tree_from_leetcode_str("[2,null,3,null,4,null,5,null,6]");
        assert_eq!(
            binary_tree_to_string_lisp_style(&result),
            "(2 nil (3 nil (4 nil (5 nil (6 nil nil)))))"
        );
    }

    #[test]
    fn test_str_leetcode_in_str_leetcode_out() {
        let result = build_tree_from_leetcode_str("null");
        assert_eq!(binary_tree_to_string_leetcode_style(&result), "[]");

        let result = build_tree_from_leetcode_str("[]");
        assert_eq!(binary_tree_to_string_leetcode_style(&result), "[]");

        let result = build_tree_from_leetcode_str("[null]");
        assert_eq!(binary_tree_to_string_leetcode_style(&result), "[]");

        let result = build_tree_from_leetcode_str("[10]");
        assert_eq!(binary_tree_to_string_leetcode_style(&result), "[10]");

        let result = build_tree_from_leetcode_str("[3,9,20,null,null,15,7]");
        assert_eq!(
            binary_tree_to_string_leetcode_style(&result),
            "[3,9,20,null,null,15,7]"
        );

        let result = build_tree_from_leetcode_str("[2,null,3,null,4,null,5,null,6]");
        assert_eq!(
            binary_tree_to_string_leetcode_style(&result),
            "[2,null,3,null,4,null,5,null,6]"
        );
    }
}
