use fs_core::{content_store::ContentStore, node::NodeId};
pub struct InMemoryContentStore {
    data: std::collections::HashMap<NodeId, Vec<u8>>,
}

impl ContentStore for InMemoryContentStore {
    fn read_content(&self, id: NodeId) -> Option<Vec<u8>> {
        self.data.get(&id).cloned()
    }
    fn write_content(&mut self, id: NodeId, content: &[u8]) {
        self.data.insert(id, content.to_vec());
    }
}

impl InMemoryContentStore {
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
}
