use crate::data::{unsimplified_ast::IdentifierName, FileId};

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct FileTree {
    root: FileId,
    children: FxHashMap<FileId, FxHashMap<IdentifierName, FileId>>,
    parents: FxHashMap<FileId, FileId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CannotFindChildError {
    CannotFindParent,
    CannotFindChildWithGivenName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChildAlreadyExistsError;

impl FileTree {
    pub fn from_root(root: FileId) -> Self {
        Self {
            root,
            children: FxHashMap::default(),
            parents: FxHashMap::default(),
        }
    }
}

impl FileTree {
    pub fn root(&self) -> FileId {
        self.root
    }

    pub fn child(
        &self,
        file_id: FileId,
        name: &IdentifierName,
    ) -> Result<FileId, CannotFindChildError> {
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
        name: &IdentifierName,
        child: FileId,
    ) -> Result<(), ChildAlreadyExistsError> {
        let old_child = self
            .children
            .entry(parent)
            .or_default()
            .insert(name.clone(), child);

        if let Some(old_entry) = old_child {
            // Undo the insertion, since it is illegal.
            self.children
                .entry(parent)
                .or_default()
                .insert(name.clone(), old_entry);
            return Err(ChildAlreadyExistsError);
        }

        self.parents.insert(child, parent);
        Ok(())
    }

    pub fn is_left_strict_descendant_of_right(&self, left: FileId, right: FileId) -> bool {
        self.is_left_non_strict_descendant_of_right(left, right) && left != right
    }

    pub fn is_left_non_strict_descendant_of_right(&self, left: FileId, right: FileId) -> bool {
        let mut current_ancestor = left;
        loop {
            if current_ancestor == right {
                return true;
            }

            if let Some(parent) = self.parents.get(&current_ancestor) {
                current_ancestor = *parent;
            } else {
                return false;
            }
        }
    }
}
