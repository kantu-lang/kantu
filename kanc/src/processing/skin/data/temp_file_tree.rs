use crate::data::{
    file_tree::FileTree,
    unsimplified_ast::{IdentifierName, ModStatement},
    FileId,
};

use rustc_hash::FxHashMap;

#[derive(Clone, Debug)]
pub struct TempFileTree {
    root: FileId,
    children: FxHashMap<FileId, FxHashMap<IdentifierName, (ModStatement, FileId)>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CannotFindChildError {
    CannotFindParent,
    CannotFindChildWithGivenName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChildAlreadyExistsError {
    pub existing_mod: ModStatement,
    pub existing_child: FileId,
    pub new_mod: ModStatement,
    pub new_child: FileId,
}

impl TempFileTree {
    pub fn from_root(root: FileId) -> Self {
        Self {
            root,
            children: FxHashMap::default(),
        }
    }
}

impl TempFileTree {
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
        let Some((_, child)) = child_map.get(name) else {
            return Err(CannotFindChildError::CannotFindChildWithGivenName);
        };
        Ok(*child)
    }

    pub fn mod_defining_child(
        &self,
        file_id: FileId,
        name: &IdentifierName,
    ) -> Result<&ModStatement, CannotFindChildError> {
        let Some(child_map) = self.children.get(&file_id) else {
            return Err(CannotFindChildError::CannotFindParent);
        };
        let Some((mod_, _)) = child_map.get(name) else {
            return Err(CannotFindChildError::CannotFindChildWithGivenName);
        };
        Ok(mod_)
    }

    pub fn add_child(
        &mut self,
        parent: FileId,
        mod_: ModStatement,
        child: FileId,
    ) -> Result<(), ChildAlreadyExistsError> {
        let name = mod_.name.name.clone();
        let old_entry = self
            .children
            .entry(parent)
            .or_default()
            .insert(name.clone(), (mod_, child));

        if let Some(old_entry) = old_entry {
            // Undo the insertion, since it is illegal.
            let new_entry = self
                .children
                .entry(parent)
                .or_default()
                .insert(name.clone(), old_entry.clone())
                .expect("We just inserted an entry, so it must be Some.");
            return Err(ChildAlreadyExistsError {
                existing_mod: old_entry.0,
                existing_child: old_entry.1,
                new_mod: new_entry.0,
                new_child: new_entry.1,
            });
        }

        Ok(())
    }
}

impl From<TempFileTree> for FileTree {
    fn from(temp: TempFileTree) -> Self {
        let mut out = FileTree::from_root(temp.root);
        for (parent_id, child_map) in temp.children {
            for (name, (_, child_id)) in child_map {
                out.add_child(parent_id, &name, child_id)
                    .expect("TempFileTree should never contain duplicate children.");
            }
        }
        out
    }
}
