pub mod filesystem;
pub mod node;
pub mod metadata_store;
pub mod content_store;

// pub struct FileMetadata {
//     pub name: String,
//     pub size: usize,
//     pub created_at: u64,
//     pub modified_at: u64,
// }

// pub trait MetadataStore {
//     fn get_metadata(&self, path: &str) -> Option<FileMetadata>;
//     fn set_metadata(&mut self, path: &str, metadata: FileMetadata);
// }

// pub trait ContentStore {
//     fn read_content(&self, path: &str) -> Option<Vec<u8>>;
//     fn write_content(&mut self, path: &str, content: &[u8]);
// }

// // Boilerplate code
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
