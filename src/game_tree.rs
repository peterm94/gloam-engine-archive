struct Game<T>
    where T: PartialEq
{
    all_nodes: Vec<Node<T>>,
}

struct Node<T>
    where T: PartialEq {
    idx: usize,
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
    where T: PartialEq {
    fn new(idx: usize, val: T) -> Self {
        Self { idx, value: val, parent: None, children: vec![] }
    }
}

impl<T> Game<T>
    where T: PartialEq {
    fn node(&mut self, val: T) -> usize {
        for node in &self.all_nodes {
            if node.value == val {
                return node.idx;
            }
        }

        let idx = self.all_nodes.len();
        self.all_nodes.push(Node::new(idx, val));
        idx
    }

    /// Parent has to already exist in the tree.
    fn add_child(&mut self, parent: usize, child: T) {
        let child_node = self.node(child);
        let parent_node = &mut self.all_nodes[parent];
        parent_node.children.push(child_node);
        self.all_nodes[child_node].parent = Some(parent);
    }
}