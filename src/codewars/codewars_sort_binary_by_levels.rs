// https://www.codewars.com/kata/52bef5e3588c56132c0003bc/train/rust
// 4 kyu

// You are given a binary tree:

// struct Node {
//   value: u32,
//   left: Option<Box<Node>>,
//   right: Option<Box<Node>>
// }
// Your task is to return the list with elements from tree sorted by levels, which means the root element goes first, then root children (from left to right) are second and third, and so on.

// Inputs will always contain at least one node.

// Example 1 - following tree:

//                  2
//             8        9
//           1  3     4   5
// Should return following list:

// [2,8,9,1,3,4,5]
// Example 2 - following tree:

//                  1
//             8        4
//               3        5
//                          7
// Should return following list:

// [1,8,4,3,5,7]
// Note: A Node's tree can be displayed via Debug: println!("{:?}", node)

#[derive(Debug)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: Node) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    fn right(mut self, node: Node) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(
            tree_by_levels(&Node::new(42)),
            [42],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
        assert_eq!(
            tree_by_levels(&root),
            [1, 2, 3, 4, 5, 6],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut queue = std::collections::VecDeque::new();
    let mut result = Vec::new();

    // Start with the root node
    queue.push_back(root);
    while let Some(node) = queue.pop_front() {
        // Add the value of the current node to the result vector
        result.push(node.value);
        // Add left child to queue if it exists
        if let Some(left) = &node.left {
            queue.push_back(left);
        }

        // Add right child to queue if it exists
        if let Some(right) = &node.right {
            queue.push_back(right);
        }
    }
    result
}

pub fn run() {
    let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
    tree_by_levels(&root);
}

/* CODEWARS GOOD SOLUTIONS



*/
