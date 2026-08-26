#[derive(Debug, Clone)]
struct Node {
    value: u8,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn contains(node: &Option<Box<Node>>, target: u8) -> bool {
    match node {
        None => false,
        Some(node) => {
            if node.value == target {
                true
            } else {
                contains(&node.left, target) || contains(&node.right, target)
            }
        }
    }
}

fn path_to_node(node: &Option<Box<Node>>, target: u8) -> Vec<u8> {
    match node {
        None => vec![],
        Some(node) => {
            if node.value == target {
                return vec![node.value];
            }

            let left_path = path_to_node(&node.left, target);
            if !left_path.is_empty() {
                return [vec![node.value], left_path].concat();
            }

            let right_path = path_to_node(&node.right, target);
            if !right_path.is_empty() {
                return [vec![node.value], right_path].concat();
            }

            vec![]
        }
    }
}

fn create_tree() -> Box<Node> {
    let tree = Node {
        value: 5,
        right: Some(Box::new(Node {
            value: 8,
            left: None,
            right: None,
        })),
        left: Some(Box::new(Node {
            value: 3,
            left: Some(Box::new(Node {
                value: 1,
                left: None,
                right: None,
            })),
            right: Some(Box::new(Node {
                value: 4,
                left: None,
                right: None,
            })),
        })),
    };

    Box::new(tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_contain_pass() {
        let tree = create_tree();
        let res = contains(&Some(tree), 4);

        assert!(res)
    }

    #[test]
    fn test_case_contain_fail() {
        let tree = create_tree();
        let res = contains(&Some(tree), 10);

        assert!(!res);
    }

    #[test]
    fn test_case_find_path_pass() {
        let tree = create_tree();
        let res = path_to_node(&Some(tree), 4);
        assert_eq!(res, vec![5, 3, 4])
    }

    #[test]
    fn test_case_find_path_fail() {
        let tree = create_tree();
        let res = path_to_node(&Some(tree), 10);
        assert!(res.is_empty())
    }
}
