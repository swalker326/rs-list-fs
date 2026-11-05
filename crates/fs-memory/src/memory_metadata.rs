use fs_core::{
    metadata_store::MetadataStore,
    node::{DirEntry, Node, NodeId, NodeKind},
};
use std::collections::HashMap;

pub const ROOT_NODE_ID: NodeId = NodeId(0);

pub struct InMemoryMetadataStore {
    nodes: HashMap<NodeId, Node>,
    children: HashMap<NodeId, HashMap<String, NodeId>>,
    next_id: u64,
}

impl InMemoryMetadataStore {
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        let root = Node::new(ROOT_NODE_ID, NodeKind::Directory, 0, 0, 0);
        nodes.insert(ROOT_NODE_ID, root);

        Self {
            nodes,
            children: HashMap::new(),
            next_id: ROOT_NODE_ID.0,
        }
    }
}

impl Default for InMemoryMetadataStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataStore for InMemoryMetadataStore {
    fn get_node(&self, id: NodeId) -> Option<Node> {
        self.nodes.get(&id).cloned()
    }

    fn set_node(&mut self, id: NodeId, node: Node) {
        self.nodes.insert(id, node);
    }

    fn add_child(&mut self, parent_id: NodeId, child_id: NodeId, name: String) {
        self.children
            .entry(parent_id)
            .or_insert_with(HashMap::new)
            .insert(name, child_id);
    }
    fn find_child(&self, parent_id: NodeId, name: &str) -> Option<NodeId> {
        self.children
            .get(&parent_id)
            .and_then(|children| children.get(name))
            .cloned()
    }

    fn next_node_id(&mut self) -> NodeId {
        self.next_id += 1;
        NodeId(self.next_id)
    }

    fn list_children(&self, parent_id: NodeId) -> Vec<DirEntry> {
        self.children
            .get(&parent_id)
            .map(|children| {
                let mut entries: Vec<DirEntry> = children
                    .iter()
                    .map(|(name, id)| DirEntry::new(name.clone(), *id))
                    .collect();
                entries.sort_by(|left, right| left.name().cmp(right.name()));
                entries
            })
            .unwrap_or_default()
    }
}
