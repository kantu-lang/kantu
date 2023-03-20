use super.*;

pub type LocalX {
    locx(x: Nat): LocalX,
}
pub use LocalX.locx;

pub type LocalY {
    locy(y: Nat): LocalY,
}
pub use LocalY.locy;

pub let(*) locx_raw = fun _(lx: LocalX): Nat {
    match lx {
        locx(x) => x,
    }
};
pub let(*) locy_raw = fun _(ly: LocalY): Nat {
    match ly {
        locy(y) => y,
    }
};

pub type IntLocalX {
    ilocx(x: Int): IntLocalX,
}
pub use IntLocalX.ilocx;

pub type IntLocalY {
    ilocy(y: Int): IntLocalY,
}
pub use IntLocalY.ilocy;

pub let ilocx_raw = fun _(lx: IntLocalX): Int {
    match lx {
        ilocx(x) => x,
    }
};

pub let ilocy_raw = fun _(ly: IntLocalY): Int {
    match ly {
        ilocy(y) => y,
    }
};
