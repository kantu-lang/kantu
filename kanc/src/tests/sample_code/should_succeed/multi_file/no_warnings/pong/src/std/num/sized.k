use super.*;

pub let(*) nat_2 = nat.add(nat.one, nat.one);
pub let(*) nat_3 = nat.add(nat_2, nat.one);
pub let(*) nat_8 = nat.pow(nat_2, nat_3);
pub let(*) nat_2pow8 = nat.pow(nat_2, nat_8);

pub type U8 {
    u8(n: Nat, upper: Trueb(nat.le(n, nat_2pow8))): U8,
}
