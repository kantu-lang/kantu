use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub pack_yscl_abs_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub pack_yscl_abs_path: PathBuf,
    pub kantu_version: KantuVersion,
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
