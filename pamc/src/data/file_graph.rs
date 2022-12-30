use crate::data::FileId;

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct FileGraph {
    root: FileId,
    children: FxHashMap<FileId, FxHashMap<String, FileId>>,
    parents: FxHashMap<FileId, FileId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CannotFindChildError {
    CannotFindParent,
    CannotFindChildWithGivenName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChildAlreadyExistsError;

impl FileGraph {
    pub fn from_root(root: FileId) -> Self {
        Self {
            root,
            children: FxHashMap::default(),
            parents: FxHashMap::default(),
        }
    }
}

impl FileGraph {
    pub fn root(&self) -> FileId {
        self.root
    }

    pub fn child(&self, file_id: FileId, name: &str) -> Result<FileId, CannotFindChildError> {
        let Some(child_map) = self.children.get(&file_id) else {
            return Err(CannotFindChildError::CannotFindParent);
        };
        let Some(child) = child_map.get(name) else {
            return Err(CannotFindChildError::CannotFindChildWithGivenName);
        };
        Ok(*child)
    }

    pub fn parent(&self, file_id: FileId) -> Option<FileId> {
        self.parents.get(&file_id).copied()
    }

    pub fn add_child(
        &mut self,
        parent: FileId,
        name: &str,
        child: FileId,
    ) -> Result<(), ChildAlreadyExistsError> {
        let old_child = self
            .children
            .entry(parent)
            .or_default()
            .insert(name.to_string(), child);

        if let Some(old_entry) = old_child {
            // Undo the insertion, since it is illegal.
            self.children
                .entry(parent)
                .or_default()
                .insert(name.to_string(), old_entry);
            return Err(ChildAlreadyExistsError);
        }

        self.parents.insert(child, parent);
        Ok(())
    }
}