pub struct Node<'a> {
    pub val: usize,
    pub left: Option<&'a Node<'a>>,
    pub right: Option<&'a Node<'a>>,
}

pub fn dfs<'a>(n: &'a Node<'a>, target: usize) -> Option<usize> {
    if n.val == target {
        return Some(n.val);
    }

    if let Some(left) = n.left {
        if let Some(found) = dfs(left, target) {
            return Some(found);
        }
    }
    if let Some(right) = n.right {
        if let Some(found) = dfs(right, target) {
            return Some(found);
        }
    }

    None
}
