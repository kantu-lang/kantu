/// This macro takes a string literal as input.
///
/// If the string literal is a valid path to a file, it returns
/// the string literal as-is.
///
/// If the string literal is not a valid path to a file, it will
/// cause a compilation error.
#[macro_export]
macro_rules! checked_path {
    ($path_:literal) => {{
        let _ = include_str!($path_);
        $path_
    }};
}
