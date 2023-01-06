use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub pack_abs_path: PackPath,
}

#[derive(Clone, Debug)]
pub enum PackPath {
    PackYscl(PathBuf),
    SingleFile(PathBuf),
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub pack_abs_path: PackPath,

    pub kantu_version: KantuVersion,
    pub target_dir: PathBuf,
    pub show_db_indices: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KantuVersion {
    V1_0_0,
}

impl KantuVersion {
    pub fn new(version: &str) -> Option<Self> {
        match version {
            "1.0.0" => Some(Self::V1_0_0),
            _ => None,
        }
    }
}
