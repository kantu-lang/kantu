use mod;
use mod.foo;
use mod.foo.bar;

use super;
use super.foo;
use super.foo.bar;

use super2;
use super2.foo;
use super2.foo.bar;

use super8;
use super8.foo;
use super8.foo.bar;

use pack;
use pack.foo;
use pack.foo.bar;

use foo;
use foo.bar;
use foo.bar.baz;

pub use mod;
pub use mod.foo;
pub use mod.foo.bar;

pub use super;
pub use super.foo;
pub use super.foo.bar;

pub use super2;
pub use super2.foo;
pub use super2.foo.bar;

pub use super8;
pub use super8.foo;
pub use super8.foo.bar;

pub use pack;
pub use pack.foo;
pub use pack.foo.bar;

pub use foo;
pub use foo.bar;
pub use foo.bar.baz;

pub(pack.foo.bar) use mod;
pub(pack.foo.bar) use mod.foo;
pub(pack.foo.bar) use mod.foo.bar;

pub(pack.foo.bar) use super;
pub(pack.foo.bar) use super.foo;
pub(pack.foo.bar) use super.foo.bar;

pub(pack.foo.bar) use super2;
pub(pack.foo.bar) use super2.foo;
pub(pack.foo.bar) use super2.foo.bar;

pub(pack.foo.bar) use super8;
pub(pack.foo.bar) use super8.foo;
pub(pack.foo.bar) use super8.foo.bar;

pub(pack.foo.bar) use pack;
pub(pack.foo.bar) use pack.foo;
pub(pack.foo.bar) use pack.foo.bar;

pub(pack.foo.bar) use foo;
pub(pack.foo.bar) use foo.bar;
pub(pack.foo.bar) use foo.bar.baz;
