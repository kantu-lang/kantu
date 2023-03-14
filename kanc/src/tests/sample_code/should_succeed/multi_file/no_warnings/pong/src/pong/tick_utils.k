use super.*;

pub let ball_bounce_y_top = fun _(raw: Int): Int {
    int.sub(
        int10k,
        Int.nat(int_abs(
            int.sub(
                Int.nat(nat_mod(int_abs(raw), pos20k)),
                int10k
            )
        ))
    )
};

// TODO: Refactor?
// Do we really need two params?
pub let ball_bounce_y = fun _(old_y: LocalY, delta_y: IntLocalY): LocalY {
    locy(relu(int.add(
        ball_bounce_y_top(int.sub(
            int.add(Int.nat(locy_raw(old_y)), ilocy_raw(delta_y)),
            Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
        )),
        Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
    )))
};

pub let ball_bounce_vy_sign_top = fun _(raw: Int): Sign {
    match int.parity(match int_abs(raw) {
        zero => int0,
        pos(p) => Int.nat(int.floor_div_nat(
            nat_sub(Nat.pos(p), nat1),
            pos10k,
        )),
    }) {
        zero => Sign.pos,
        one => Sign.neg,
    }
};

// TODO: Refactor
// Do we really need two params?
pub let ball_bounce_vy_sign = fun _(old_y: LocalY, delta_y: IntLocalY): Sign {
    ball_bounce_vy_sign_top(int.sub(
        int.add(Int.nat(locy_raw(old_y)), ilocy_raw(delta_y)),
        Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
    ))
};

pub let is_ball_touching_left_paddle = fun _(state: State): Bool {
    // TODO
    true
};

pub let is_ball_touching_right_paddle = fun _(state: State): Bool {
    // TODO
    true
};
