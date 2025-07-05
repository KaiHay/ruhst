use twosumn::seerch::search::*;

#[test]
fn test_dfs_basic() {
    let node1 = Node {
        val: 2,
        left: None,
        right: None,
    };
    let node2 = Node {
        val: 3,
        left: None,
        right: None,
    };
    let node = Node {
        val: 1,
        left: Some(&node1),
        right: Some(&node2),
    };
    let target = 3;
    let result = dfs(&node, target);
    assert!(result.is_some(), "DFS should find a path from 0 to 3");
}
