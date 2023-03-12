use pack.boomborg.*;
use pack.std;
use std.num.*;
use std.time.Time;
use std.list.List;
use std.Equal;
use std.bool.prelude.*;
use std.string.String;

pub let(*) relu = fun _(n: Int): Nat {
    match n {
        neg(_) => Nat.zero,
        nat(nn) => nn,
    }
};

pub let(*) max1_int = fun _(n: Int): Pos {
    match n {
        neg(_) => Pos.one,
        nat(nn) => match nn {
            zero => Pos.one,
            pos(np) => np,
        },
    }
};

pub type Pfrac {
    pfrac(num: Pos, div: Pos): Pfrac,
}

pub let(*) pfrac_mult = fun _(pf: Pfrac, right: Nat): Nat {
    match pf {
        pfrac(num, div) => int.floor_div_nat(
            nat.mul(Nat.pos(num), right),
            div,
        ),
    }
};

pub let(*) pos1 = Pos.one;
pub let(*) pos2 = pos.add(pos1, pos1);
pub let(*) pos3 = pos.add(pos2, pos1);
pub let(*) pos4 = pos.add(pos3, pos1);
pub let(*) pos8 = pos.pow(pos2, pos3);
pub let(*) pos10 = pos.add(pos2, pos8);
pub let(*) pos64 = pos.mul(pos8, pos8);
pub let(*) pos65 = pos.add(pos1, pos64);
pub let(*) pos66 = pos.add(pos1, pos65);
pub let(*) pos67 = pos.add(pos1, pos66);
pub let(*) pos1000 = pos.pow(pos10, pos3);

pub let(*) refl_true = Equal.refl(Bool, true);

pub let(*) u8_65 = U8.u8(Nat.pos(pos65), refl_true);
pub let(*) u8_66 = U8.u8(Nat.pos(pos66), refl_true);
pub let(*) u8_67 = U8.u8(Nat.pos(pos67), refl_true);

pub let(*) half = Pfrac.pfrac(
    pos1,
    pos2,
);

pub let(*) paddle_height_factor = Pfrac.pfrac(
    pos1,
    pos8,
);
pub let(*) ball_height_factor = Pfrac.pfrac(
    pos1,
    pos64,
);
pub let(*) paddle_x_margin_factor = Pfrac.pfrac(
    pos1,
    pos64,
);
pub let(*) paddle_width_factor = Pfrac.pfrac(
    pos1,
    pos64,
);

pub let(*) ball_vx_mag_factor = Pfrac.pfrac(
    pos1,
    pos3,
);

pub let(*) ball_vy_mag_factor = Pfrac.pfrac(
    pos2,
    pos3,
);

// 'A'
pub let(*) background_image_str = String.utf8(
    List.cons(
        U8,
        u8_65,
        List.nil(U8),
    ),
);
// 'B'
pub let(*) paddle_image_str = String.utf8(
    List.cons(
        U8,
        u8_66,
        List.nil(U8),
    ),
);
// 'C'
pub let(*) ball_image_str = String.utf8(
    List.cons(
        U8,
        u8_67,
        List.nil(U8),
    ),
);

pub type State {
    state(
        ~time: Time,
        ~window: Window,
        ~left_paddle_y: Nat,
        ~right_paddle_y: Nat,
        ~ball_x: Nat,
        ~ball_y: Nat,
        ~ball_vx_sign: Sign,
        ~ball_vy_sign: Sign,
    ): State,
}

pub let(*) init = fun _(window: Window, time: Time): State {
    match window {
        window(window_w, window_h) => State.state(
            :time,
            :window,
            left_paddle_y: relu(int.sub(
                Int.nat(pfrac_mult(half, window_h)),
                Int.nat(pfrac_mult(half, pfrac_mult(paddle_height_factor, window_h))),
            )),
            right_paddle_y: relu(int.sub(
                Int.nat(pfrac_mult(half, window_h)),
                Int.nat(pfrac_mult(half, pfrac_mult(paddle_height_factor, window_h))),
            )),
            ball_x: relu(int.sub(
                Int.nat(pfrac_mult(half, window_w)),
                Int.nat(pfrac_mult(half, pfrac_mult(ball_height_factor, window_h))),
            )),
            ball_y: relu(int.sub(
                Int.nat(pfrac_mult(half, window_h)),
                Int.nat(pfrac_mult(half, pfrac_mult(ball_height_factor, window_h))),
            )),
            ball_vx_sign: Sign.pos,
            ball_vy_sign: Sign.neg,
        )
    }
};

pub let(*) render_background = fun _(state: State): Entity {
    match state {
        state(:window, ...) => match window {
            window(window_w, window_h) => Entity.scaled(
                Nat.zero,
                Nat.zero,
                window_w,
                window_h,
                background_image_str,
            ),
        },
    }
};

pub let(*) render_left_paddle = fun _(state: State): Entity {
    match state {
        state(:window, :left_paddle_y, ...) => match window {
            window(window_w, window_h) => Entity.scaled(
                pfrac_mult(paddle_x_margin_factor, window_w),
                left_paddle_y,
                pfrac_mult(paddle_width_factor, window_w),
                pfrac_mult(paddle_height_factor, window_h),
                paddle_image_str,
            ),
        },
    }
};

pub let(*) render_right_paddle = fun _(state: State): Entity {
    match state {
        state(:window, :right_paddle_y, ...) => match window {
            window(window_w, window_h) => Entity.scaled(
                relu(int.sub(
                    int.sub(
                        Int.nat(window_w),
                        Int.nat(pfrac_mult(paddle_x_margin_factor, window_w)),
                    ),
                    Int.nat(pfrac_mult(paddle_width_factor, window_w)),
                )),
                right_paddle_y,
                pfrac_mult(paddle_width_factor, window_w),
                pfrac_mult(paddle_height_factor, window_h),
                paddle_image_str,
            ),
        },
    }
};

pub let(*) render_ball = fun _(state: State): Entity {
    match state {
        state(:window, :ball_x, :ball_y, ...) => match window {
            window(window_w, window_h) => Entity.scaled(
                ball_x,
                ball_y,
                pfrac_mult(ball_height_factor, window_h),
                pfrac_mult(ball_height_factor, window_h),
                ball_image_str,
            ),
        },
    }
};

pub let(*) render = fun _(state: State): List(Entity) {
    List.cons(
        Entity,
        render_background(state),
        List.cons(
            Entity,
            render_left_paddle(state),
            List.cons(
                Entity,
                render_right_paddle(state),
                List.cons(
                    Entity,
                    render_ball(state),
                    List.nil(Entity),
                ),
            ),
        ),
    )
};

pub let(*) time_millis = fun _(t: Time): Nat {
    match t {
        time(millis) => millis,
    }
};

pub let sign_nat = fun _(s: Sign, n: Nat): Int {
    match s {
        pos => Int.nat(n),
        neg => nat.neg(n),
    }
};

pub let(*) tick = fun _(state: State, new_time: Time): State {
    // TODO
    match state {
        state(
            time: old_time,
            :window,
            :left_paddle_y,
            :right_paddle_y,
            ball_x: old_ball_x,
            ball_y: old_ball_y,
            :ball_vx_sign,
            :ball_vy_sign,
        ) => fun _(~elapsed_millis: Pos, ~ball_vx_mag: Nat, ~ball_vy_mag: Nat): State {
            State.state(
                time: new_time,
                :window,
                :left_paddle_y,
                :right_paddle_y,
                ball_x: relu(int.add(
                    Int.nat(old_ball_x),
                    sign_nat(
                        ball_vx_sign,
                        pfrac_mult(
                            Pfrac.pfrac(elapsed_millis, pos1000),
                            ball_vx_mag,
                        ),
                    ),
                )),
                ball_y: relu(int.add(
                    Int.nat(old_ball_y),
                    sign_nat(
                        ball_vy_sign,
                        pfrac_mult(
                            Pfrac.pfrac(elapsed_millis, pos1000),
                            ball_vy_mag,
                        ),
                    ),
                )),
                :ball_vx_sign,
                :ball_vy_sign,
            )
        }(
            elapsed_millis: max1_int(int.sub(
                Int.nat(time_millis(new_time)),
                Int.nat(time_millis(old_time)),
            )),

            ball_vx_mag: pfrac_mult(ball_vx_mag_factor, match window { window(w, _) => w }),

            ball_vy_mag: pfrac_mult(ball_vy_mag_factor, match window { window(_, h) => h }),
        ),
    }
};

pub let(*) handle = fun _(state: State, event: Event): State {
    // TODO
    state
};

pub let(*) app = App.app(
    State,
    render,
    tick,
    init,
    handle,
);
