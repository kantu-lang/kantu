use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct CliOptions {
    pub pack_omlet_path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct CompilerOptions {
    pub cli: CliOptions,
}
