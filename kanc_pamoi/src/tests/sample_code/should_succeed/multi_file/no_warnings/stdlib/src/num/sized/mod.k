use super.*;

pub let(*) max0 = fun _(i: Int): Nat {
    match i {
        neg(_) => Nat.zero,
        nat(in) => in,
    }
};

pub let(*) nat_2 = nat.add(nat.one, nat.one);
pub let(*) nat_3 = nat.add(nat_2, nat.one);
pub let(*) nat_8 = nat.pow(nat_2, nat_3);
pub let(*) nat_255 = max0(int.sub(Int.nat(nat.pow(nat_2, nat_8)), int.one));

pub type U8 {
    u8(n: Nat, upper: Trueb(nat.le(n, nat_255))): U8,
}

pub mod u8;
