use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub unvalidated_pack_omlet_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub pack_omlet_path: PathBuf,
}
