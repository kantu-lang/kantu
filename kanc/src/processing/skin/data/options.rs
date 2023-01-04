use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub pack_yscl_abs_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub pack_yscl_abs_path: PathBuf,
}
