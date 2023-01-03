use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub pack_omlet_abs_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub pack_omlet_abs_path: PathBuf,
}
