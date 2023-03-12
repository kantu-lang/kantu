use pack.boomborg.*;
use pack.std;
use std.num.*;
use std.time.Time;
use std.list.List;
use std.Equal;
use std.bool.prelude.*;
use std.string.String;
use std.opt.Opt;

pub let u8_eq = fun _(a: U8, b: U8): Bool {
    match a {
        u8(an, _) => match b {
            u8(bn, _) => nat.eq(an, bn),
        },
    }
};

pub let u8_list_eq = fun u8_list_eq(-a: List(U8), b: List(U8)): Bool {
    match a {
        nil(_) => match b {
            nil(_) => true,
            cons(_, _, _) => false,
        },
        cons(_, a_car, a_cdr) => match b {
            nil(_) => false,
            cons(_, b_car, b_cdr) => match u8_eq(a_car, b_car) {
                true => u8_list_eq(a_cdr, b_cdr),
                false => false,
            },
        },
    }
};

pub let str_eq = fun _(s1: String, s2: String): Bool {
    match s1 {
        utf8(s1_bytes) => match s2 {
            utf8(s2_bytes) => u8_list_eq(s1_bytes, s2_bytes),
        },
    }
};

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

pub let pos1 = Pos.one;
pub let pos2 = pos.add(pos1, pos1);
pub let pos3 = pos.add(pos1, pos2);
pub let pos4 = pos.add(pos1, pos3);
pub let pos5 = pos.add(pos1, pos4);
pub let pos6 = pos.add(pos1, pos5);
pub let pos7 = pos.add(pos1, pos6);
pub let pos8 = pos.add(pos1, pos7);
pub let pos9 = pos.add(pos1, pos8);
pub let pos10 = pos.add(pos1, pos9);

pub let pos20 = pos.mul(pos2, pos10);
pub let pos30 = pos.mul(pos3, pos10);
pub let pos40 = pos.mul(pos4, pos10);
pub let pos50 = pos.mul(pos5, pos10);
pub let pos60 = pos.mul(pos6, pos10);
pub let pos70 = pos.mul(pos7, pos10);
pub let pos80 = pos.mul(pos8, pos10);
pub let pos90 = pos.mul(pos9, pos10);
pub let pos100 = pos.mul(pos10, pos10);

pub let pos64 = pos.mul(pos8, pos8);
pub let pos65 = pos.add(pos1, pos64);
pub let pos66 = pos.add(pos1, pos65);
pub let pos67 = pos.add(pos1, pos66);
pub let pos68 = pos.add(pos1, pos67);

pub let pos83 = pos.add(pos3, pos80);
pub let pos85 = pos.add(pos5, pos80);
pub let pos87 = pos.add(pos7, pos80);
pub let pos110 = pos.add(pos10, pos100);
pub let pos111 = pos.add(pos1, pos110);
pub let pos112 = pos.add(pos1, pos111);
pub let pos114 = pos.add(pos4, pos.add(pos10, pos100));
pub let pos115 = pos.add(pos5, pos.add(pos10, pos100));
pub let pos119 = pos.add(pos9, pos.add(pos10, pos100));

pub let pos1000 = pos.pow(pos10, pos3);

pub let(*) refl_true = Equal.refl(Bool, true);

pub let u8_65 = U8.u8(Nat.pos(pos65), refl_true);
pub let u8_66 = U8.u8(Nat.pos(pos66), refl_true);
pub let u8_67 = U8.u8(Nat.pos(pos67), refl_true);
pub let u8_68 = U8.u8(Nat.pos(pos68), refl_true);
pub let u8_83 = U8.u8(Nat.pos(pos83), refl_true);
pub let u8_85 = U8.u8(Nat.pos(pos85), refl_true);
pub let u8_87 = U8.u8(Nat.pos(pos87), refl_true);
pub let u8_110 = U8.u8(Nat.pos(pos110), refl_true);
pub let u8_111 = U8.u8(Nat.pos(pos111), refl_true);
pub let u8_112 = U8.u8(Nat.pos(pos112), refl_true);
pub let u8_114 = U8.u8(Nat.pos(pos114), refl_true);
pub let u8_115 = U8.u8(Nat.pos(pos115), refl_true);
pub let u8_119 = U8.u8(Nat.pos(pos119), refl_true);

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

pub let ch_A = u8_65;
pub let ch_B = u8_66;
pub let ch_C = u8_67;
pub let ch_D = u8_68;
pub let ch_S = u8_83;
pub let ch_U = u8_85;
pub let ch_W = u8_87;

pub let ch_n = u8_110;
pub let ch_o = u8_111;
pub let ch_p = u8_112;
pub let ch_r = u8_114;
pub let ch_s = u8_115;
pub let ch_w = u8_119;

// "A"
pub let(*) background_image_str = String.utf8(
    List.cons(
        U8,
        ch_A,
        List.nil(U8),
    ),
);
// "B"
pub let(*) paddle_image_str = String.utf8(
    List.cons(
        U8,
        ch_B,
        List.nil(U8),
    ),
);
// "C"
pub let(*) ball_image_str = String.utf8(
    List.cons(
        U8,
        ch_C,
        List.nil(U8),
    ),
);

// ["ArrowUp"]
pub let(*) right_paddle_up_strs = List.cons(
    String,
    String.utf8(
        List.cons(
            U8,
            ch_A,
            List.cons(
                U8,
                ch_r,
                List.cons(
                    U8,
                    ch_r,
                    List.cons(
                        U8,
                        ch_o,
                        List.cons(
                            U8,
                            ch_w,
                            List.cons(
                                U8,
                                ch_U,
                                List.cons(
                                    U8,
                                    ch_p,
                                    List.nil(U8),
                                )
                            )
                        )
                    )
                )
            )
        )
    ),
    List.nil(String),
);

// ["ArrowDown"]
pub let(*) right_paddle_down_strs = List.cons(
    String,
    String.utf8(
        List.cons(
            U8,
            ch_A,
            List.cons(
                U8,
                ch_r,
                List.cons(
                    U8,
                    ch_r,
                    List.cons(
                        U8,
                        ch_o,
                        List.cons(
                            U8,
                            ch_w,
                            List.cons(
                                U8,
                                ch_D,
                                List.cons(
                                    U8,
                                    ch_o,
                                    List.cons(
                                        U8,
                                        ch_w,
                                        List.cons(
                                            U8,
                                            ch_n,
                                            List.nil(U8),
                                        )
                                    )
                                )
                            )
                        )
                    )
                )
            )
        )
    ),
    List.nil(String),
);


// ["w", "W"]
pub let(*) left_paddle_up_strs = List.cons(
    String,
    String.utf8(
        List.cons(U8, ch_w, List.nil(U8))
    ),
    List.cons(
        String,
        String.utf8(
            List.cons(U8, ch_W, List.nil(U8))
        ),
        List.nil(String),
    )
);

// ["s", "S"]
pub let(*) left_paddle_down_strs = List.cons(
    String,
    String.utf8(
        List.cons(U8, ch_s, List.nil(U8))
    ),
    List.cons(
        String,
        String.utf8(
            List.cons(U8, ch_S, List.nil(U8))
        ),
        List.nil(String),
    )
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
        ~left_paddle_vy_sign: Opt(Sign),
        ~right_paddle_vy_sign: Opt(Sign),
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
            left_paddle_vy_sign: Opt.none(Sign),
            right_paddle_vy_sign: Opt.none(Sign),
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
            :left_paddle_vy_sign,
            :right_paddle_vy_sign,
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
                :left_paddle_vy_sign,
                :right_paddle_vy_sign,
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

pub let(*) handle_keyup = fun _(state: State, key: String): State {
    // TODO
    state
};

pub let(*) handle_keydown = fun _(state: State, key: String): State {
    // TODO
    state
};

pub let(*) handle = fun _(state: State, event: Event): State {
    match event {
        window_resize(_) => /* TODO */ state,
        keyup(key) => handle_keyup(state, key),
        keydown(key) => handle_keydown(state, key),
    }
};

pub let(*) app = App.app(
    State,
    render,
    tick,
    init,
    handle,
);
