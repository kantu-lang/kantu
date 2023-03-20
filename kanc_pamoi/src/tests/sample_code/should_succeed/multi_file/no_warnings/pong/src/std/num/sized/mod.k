use super.*;

pub let(*) pos_2 = pos.add(Pos.one, Pos.one);
pub let(*) pos_3 = pos.add(pos_2, Pos.one);
pub let(*) pos_5 = pos.add(pos_2, pos_3);
pub let(*) pos_10 = pos.add(pos_5, pos_5);
pub let(*) pos_50 = pos.mul(pos_5, pos_10);
pub let(*) pos_100 = pos.mul(pos_10, pos_10);
pub let(*) pos_200 = pos.mul(pos_100, pos_2);
pub let(*) pos_255 = pos.add(pos_200, pos.add(pos_50, pos_5));
pub let(*) nat_255 = Nat.pos(pos_255);

pub type U8 {
    u8(n: Nat, upper: Trueb(nat.le(n, nat_255))): U8,
}

pub mod u8;
