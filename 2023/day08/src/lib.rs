pub fn bisect_search<'a>(map: &'a Vec<Node>, next_node: &str) -> &'a Node {
    let mut i0 = 0;
    let mut i1 = map.len();
    let mut middle_node = &map[(i0 + i1) / 2];
    while middle_node.node != *next_node {
        if middle_node.node < next_node.to_string() {
            i0 = (i0 + i1) / 2;
        } else {
            i1 = (i0 + i1) / 2;
        }
        middle_node = &map[(i0 + i1) / 2];
    }
    &middle_node
}

#[derive(Debug)]
pub struct Node {
    pub node: String,
    pub left: String,
    pub right: String,
}
