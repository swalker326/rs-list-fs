use crate::node::NodeId;

pub trait ContentStore {
    fn read_content(&self, id: NodeId) -> Option<Vec<u8>>;
    fn write_content(&mut self, id: NodeId, content: &[u8]);
}