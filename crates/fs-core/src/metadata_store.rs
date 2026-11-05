use crate::node::{DirEntry, Node, NodeId};

pub trait MetadataStore {
    fn get_node(&self, id: NodeId) -> Option<Node>;
    fn set_node(&mut self, id: NodeId, node: Node);
    fn add_child(&mut self, parent_id: NodeId, child_id: NodeId, name: String);
    fn find_child(&self, parent_id: NodeId, name: &str) -> Option<NodeId>;
    fn next_node_id(&mut self) -> NodeId;
    fn list_children(&self, parent_id: NodeId) -> Vec<DirEntry>;
}
