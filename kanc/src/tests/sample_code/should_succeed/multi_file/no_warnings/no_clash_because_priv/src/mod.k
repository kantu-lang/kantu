mod data;

mod priv_export;
pub use priv_export.*;

mod priv_export2;
pub use priv_export2.*;

mod pub_export;
pub use pub_export.Foo;

pub let foo_to_bar = fun _(_: Foo): Bar {
    Bar.c
};
