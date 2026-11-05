#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);
#[derive(Clone, Debug)]
pub enum NodeKind {
    File,
    Directory,
}

pub struct DirEntry {
    name: String,
    id: NodeId,
}

impl DirEntry {
    pub fn new(name: String, id: NodeId) -> Self {
        Self { name, id }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> NodeId {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    id: NodeId,
    kind: NodeKind,
    size: u64,
    created_at: u64,
    modified_at: u64,
}

impl Node {
    pub fn new(id: NodeId, kind: NodeKind, size: u64, created_at: u64, modified_at: u64) -> Self {
        Self {
            id,
            kind,
            size,
            created_at,
            modified_at,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    pub fn modified_at(&self) -> u64 {
        self.modified_at
    }
}
