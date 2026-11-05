use crate::content_store::ContentStore;
use crate::metadata_store::MetadataStore;
use crate::node::{DirEntry, Node, NodeId, NodeKind};

const ROOT_NODE_ID: NodeId = NodeId(0);

#[derive(Debug)]
pub enum FsError {
    InvalidPath,
    AlreadyExists,
    NotADirectory,
    NotAFile,
    MissingNode,
}

pub type FsResult<T> = Result<T, FsError>;
pub struct FsConfig {
    pub chunk_size: usize,
}

pub struct Filesystem {
    metadata: Box<dyn MetadataStore>,
    content: Box<dyn ContentStore>,
    config: FsConfig,
}

impl Filesystem {
    pub fn new<M, C>(metadata: M, content: C, config: FsConfig) -> Self
    where
        M: MetadataStore + 'static,
        C: ContentStore + 'static,
    {
        Self {
            metadata: Box::new(metadata),
            content: Box::new(content),
            config,
        }
    }
    pub fn create_file(&mut self, path: &str, content: &[u8]) -> FsResult<()> {
        let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();

        if components.is_empty() {
            return Err(FsError::InvalidPath);
        }

        let mut current_id = ROOT_NODE_ID;

        for name in &components[..components.len() - 1] {
            let child_id = match self.metadata.find_child(current_id, name) {
                Some(child_id) => child_id,
                None => {
                    let new_id = self.metadata.next_node_id();
                    self.metadata
                        .set_node(new_id, Node::new(new_id, NodeKind::Directory, 0, 0, 0));
                    self.metadata
                        .add_child(current_id, new_id, name.to_string());
                    current_id = new_id;
                    continue;
                }
            };

            let child = self
                .metadata
                .get_node(child_id)
                .ok_or(FsError::MissingNode)?;
            match child.kind() {
                NodeKind::Directory => current_id = child_id,
                NodeKind::File => return Err(FsError::NotADirectory),
            }
        }

        let file_name = components[components.len() - 1];
        if self.metadata.find_child(current_id, file_name).is_some() {
            return Err(FsError::AlreadyExists);
        }

        let file_id = self.metadata.next_node_id();
        self.metadata.set_node(
            file_id,
            Node::new(file_id, NodeKind::File, content.len() as u64, 0, 0),
        );
        self.metadata
            .add_child(current_id, file_id, file_name.to_string());
        self.content.write_content(file_id, content);

        Ok(())
    }

    fn resolve_node_id(&self, path: &str) -> FsResult<NodeId> {
        let components: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();

        if components.is_empty() {
            return Err(FsError::InvalidPath);
        }

        let mut current_id = ROOT_NODE_ID;

        for name in components {
            current_id = self
                .metadata
                .find_child(current_id, name)
                .ok_or(FsError::MissingNode)?;
        }

        Ok(current_id)
    }

    pub fn read_file(&self, path: &str) -> FsResult<Vec<u8>> {
        let node_id = self.resolve_node_id(path)?;
        let node = self
            .metadata
            .get_node(node_id)
            .ok_or(FsError::MissingNode)?;

        match node.kind() {
            NodeKind::File => self
                .content
                .read_content(node_id)
                .ok_or(FsError::MissingNode),
            NodeKind::Directory => Err(FsError::NotAFile),
        }
    }

    pub fn list_dir(&self, path: &str) -> FsResult<Vec<DirEntry>> {
        let node_id = if path == "/" || path.is_empty() {
            ROOT_NODE_ID
        } else {
            self.resolve_node_id(path)?
        };

        let node = self
            .metadata
            .get_node(node_id)
            .ok_or(FsError::MissingNode)?;

        match node.kind() {
            NodeKind::Directory => Ok(self.metadata.list_children(node_id)),
            NodeKind::File => Err(FsError::NotAFile),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Filesystem, FsConfig, FsError};
    use crate::node::Node;
    use crate::node::NodeId;
    use crate::{content_store::ContentStore, metadata_store::MetadataStore, node::NodeKind};
    use std::collections::HashMap;

    struct InMemoryMetadataStoreForTest {
        nodes: HashMap<NodeId, Node>,
        children: HashMap<NodeId, HashMap<String, NodeId>>,
        next_id: u64,
    }

    impl InMemoryMetadataStoreForTest {
        fn new() -> Self {
            let mut nodes = HashMap::new();
            nodes.insert(
                NodeId(0),
                Node::new(NodeId(0), NodeKind::Directory, 0, 0, 0),
            );

            Self {
                nodes,
                children: HashMap::new(),
                next_id: 0,
            }
        }
    }

    impl MetadataStore for InMemoryMetadataStoreForTest {
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

        fn list_children(&self, parent_id: NodeId) -> Vec<crate::node::DirEntry> {
            self.children
                .get(&parent_id)
                .map(|children| {
                    let mut entries: Vec<_> = children
                        .iter()
                        .map(|(name, id)| crate::node::DirEntry::new(name.clone(), *id))
                        .collect();
                    entries.sort_by_key(|entry| entry.name().to_string());
                    entries
                })
                .unwrap_or_default()
        }
    }

    struct InMemoryContentStoreForTest {
        data: HashMap<NodeId, Vec<u8>>,
    }

    impl InMemoryContentStoreForTest {
        fn new() -> Self {
            Self {
                data: HashMap::new(),
            }
        }
    }

    impl ContentStore for InMemoryContentStoreForTest {
        fn read_content(&self, id: NodeId) -> Option<Vec<u8>> {
            self.data.get(&id).cloned()
        }

        fn write_content(&mut self, id: NodeId, content: &[u8]) {
            self.data.insert(id, content.to_vec());
        }
    }

    #[test]
    fn create_file_creates_nested_path() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(fs
            .create_file("/path/to/nest/ed/folder/test.txt", b"hello world")
            .is_ok());
        assert!(fs
            .create_file("/path/to/nest/ed/folder/other.txt", b"second")
            .is_ok());
        assert!(matches!(
            fs.create_file("/path/to/nest/ed/folder/test.txt", b"dup"),
            Err(FsError::AlreadyExists)
        ));
    }

    #[test]
    fn create_file_fails_when_parent_is_file() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(fs.create_file("/a/file.txt", b"leaf").is_ok());
        assert!(matches!(
            fs.create_file("/a/file.txt/child.txt", b"oops"),
            Err(FsError::NotADirectory)
        ));
    }

    #[test]
    fn read_back_written_content_for_nested_file() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(fs
            .create_file("/path/to/nest/ed/folder/test.txt", b"hello world")
            .is_ok());

        assert_eq!(
            fs.read_file("/path/to/nest/ed/folder/test.txt").unwrap(),
            b"hello world".to_vec()
        );
    }

    #[test]
    fn create_file_rejects_root_path() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(matches!(
            fs.create_file("/", b"x"),
            Err(FsError::InvalidPath)
        ));
        assert!(matches!(
            fs.create_file("", b"x"),
            Err(FsError::InvalidPath)
        ));
    }

    #[test]
    fn read_file_missing_path_returns_missing_node() {
        let fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(matches!(
            fs.read_file("/no/such/file.txt"),
            Err(FsError::MissingNode)
        ));
    }

    #[test]
    fn read_file_on_directory_path_returns_not_a_file() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(fs.create_file("/path/to/file.txt", b"ok").is_ok());

        assert!(matches!(fs.read_file("/path/to"), Err(FsError::NotAFile)));
    }

    #[test]
    fn list_dir_returns_children() {
        let mut fs = Filesystem::new(
            InMemoryMetadataStoreForTest::new(),
            InMemoryContentStoreForTest::new(),
            FsConfig { chunk_size: 4_096 },
        );

        assert!(fs.create_file("/a/b/c.txt", b"one").is_ok());

        let entries = fs.list_dir("/a").unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name(), "b");
    }
}
