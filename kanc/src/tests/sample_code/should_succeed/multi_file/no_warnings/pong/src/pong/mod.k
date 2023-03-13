use pack.boomborg.*;
use pack.std;
use std.num.*;
use std.time.Time;
use std.list.List;
use std.Equal;
use std.bool.prelude.*;
use std.string;
use std.string.String;
use std.opt.Opt;

pub type LocalX {
    locx(x: Nat): LocalX,
}
use LocalX.locx;

pub type LocalY {
    locy(y: Nat): LocalY,
}
use LocalY.locy;

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
use IntLocalX.ilocx;

pub type IntLocalY {
    ilocy(y: Int): IntLocalY,
}
use IntLocalY.ilocy;

pub let(*) ilocx_raw = fun _(lx: IntLocalX): Int {
    match lx {
        ilocx(x) => x,
    }
};

pub let(*) ilocy_raw = fun _(ly: IntLocalY): Int {
    match ly {
        ilocy(y) => y,
    }
};

pub let(*) refl_true = Equal.refl(Bool, true);

pub let(*) identity = fun _(T: Type, t: T): T {
    t
};

pub let(*) ascribe = identity;

pub let str_list_contains = fun str_list_contains(-strs: List(String), str: String): Bool {
    match strs {
        nil(_) => false,
        cons(_, car, cdr) => match string.eq(car, str) {
            true => true,
            false => str_list_contains(cdr, str),
        },
    }
};

pub let(*) relu = fun _(n: Int): Nat {
    match n {
        neg(_) => Nat.zero,
        nat(nn) => nn,
    }
};

pub let(*) int_to_pos = fun _(n: Int): Pos {
    match n {
        neg(_) => Pos.one,
        nat(nn) => match nn {
            zero => Pos.one,
            pos(np) => np,
        },
    }
};

pub let(*) nat_sub = fun _(min: Nat, sub: Nat): Nat {
    relu(int.sub(
        Int.nat(min),
        Int.nat(sub),
    ))
};

pub let(*) time_millis = fun _(t: Time): Nat {
    match t {
        time(millis) => millis,
    }
};

pub let(*) sign_nat = fun _(s: Sign, n: Nat): Int {
    match s {
        pos => Int.nat(n),
        neg => nat.neg(n),
    }
};

pub let(*) window_width = fun _(window: Window): Nat {
    match window {
        window(w, _) => w,
    }
};

pub mod literals;
use literals.*;

pub let paddle_height = locy(int.floor_div_nat(
    nat10k,
    pos7,
));
pub let ball_height = locy(int.floor_div_nat(
    nat10k,
    pos64,
));
pub let paddle_x_margin = locx(int.floor_div_nat(
    nat10k,
    pos64,
));
pub let paddle_width = locx(int.floor_div_nat(
    nat10k,
    pos64,
));
pub let ball_initial_vx = ilocx(int.floor_div(
    Int.nat(nat10k),
    pos4,
));
pub let ball_initial_vy = ilocy(int.floor_div(
    Int.nat(nat10k),
    pos3,
));

pub let to_real_x = fun _(x: LocalX, window: Window): Nat {
    match window {
        window(window_w, _) => int.floor_div_nat(
            nat.mul(locx_raw(x), window_w),
            pos10k,
        ),
    }
};
pub let to_real_y = fun _(y: LocalY, window: Window): Nat {
    match window {
        window(_, window_h) => int.floor_div_nat(
            nat.mul(locy_raw(y), window_h),
            pos10k,
        ),
    }
};

pub type State {
    state(
        ~time: Time,
        ~window: Window,
        ~left_paddle_y: LocalY,
        ~right_paddle_y: LocalY,
        ~ball_center_x: LocalX,
        ~ball_center_y: LocalY,
        ~ball_vx: IntLocalX,
        ~ball_vy: IntLocalY,
        ~left_paddle_vy_sign: Opt(Sign),
        ~right_paddle_vy_sign: Opt(Sign),
    ): State,
}

pub let init = fun _(window: Window, time: Time): State {
    match window {
        window(window_w, window_h) => State.state(
            :time,
            :window,
            left_paddle_y: locy(nat_sub(nat5000, int.floor_div_nat(locy_raw(paddle_height), pos2))),
            right_paddle_y: locy(nat_sub(nat5000, int.floor_div_nat(locy_raw(paddle_height), pos2))),
            ball_center_x: locx(nat5000),
            ball_center_y: locy(nat5000),
            ball_vx: ball_initial_vx,
            ball_vy: ball_initial_vy,
            left_paddle_vy_sign: Opt.none(Sign),
            right_paddle_vy_sign: Opt.none(Sign),
        )
    }
};

pub let render_background = fun _(state: State): Entity {
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

pub let render_left_paddle = fun _(state: State): Entity {
    match state {
        state(:window, :left_paddle_y, ...) => Entity.scaled(
            to_real_x(paddle_x_margin, window),
            to_real_y(left_paddle_y, window),
            to_real_x(paddle_width, window),
            to_real_y(paddle_height, window),
            paddle_image_str,
        ),
    }
};

pub let render_right_paddle = fun _(state: State): Entity {
    match state {
        state(:window, :right_paddle_y, ...) => Entity.scaled(
            nat_sub(
                nat_sub(window_width(window), to_real_x(paddle_x_margin, window)),
                to_real_x(paddle_width, window),
            ),
            to_real_y(right_paddle_y, window),
            to_real_x(paddle_width, window),
            to_real_y(paddle_height, window),
            paddle_image_str,
        ),
    }
};

pub let render_ball = fun _(state: State): Entity {
    match state {
        state(:window, :ball_center_x, :ball_center_y, ...) => fun _(ball_radius: Nat): Entity {
            Entity.scaled(
                nat_sub(to_real_x(ball_center_x, window), ball_radius),
                nat_sub(to_real_y(ball_center_y, window), ball_radius),
                nat.mul(nat2, ball_radius),
                nat.mul(nat2, ball_radius),
                ball_image_str,
            )
        }(int.floor_div_nat(
            to_real_y(ball_height, window),
            pos2,
        ))
    }
};

pub let render = fun _(state: State): List(Entity) {
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

pub let nat_mod = fun _(n: Nat, divisor: Pos): Nat {
    relu(int.sub(
        Int.nat(n),
        int.mul(
            int.floor_div(Int.nat(n), divisor),
            Int.nat(Nat.pos(divisor)),
        ),
    ))
};

pub let int_abs = fun _(i: Int): Nat {
    match i {
        neg(neg_i) => Nat.pos(neg_i),
        nat(in) => in,
    }
};

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
pub let(super) ball_bounce_y = fun _(old_y: LocalY, delta_y: IntLocalY): LocalY {
    locy(relu(int.add(
        ball_bounce_y_top(int.sub(
            int.add(Int.nat(locy_raw(old_y)), ilocy_raw(delta_y)),
            Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
        )),
        Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
    )))
};

pub let sign_int = fun _(sign: Sign, i: Int): Int {
    match sign {
        pos => i,
        neg => int.neg(i),
    }
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
pub let(super) ball_bounce_vy_sign = fun _(old_y: LocalY, delta_y: IntLocalY): Sign {
    ball_bounce_vy_sign_top(int.sub(
        int.add(Int.nat(locy_raw(old_y)), ilocy_raw(delta_y)),
        Int.nat(int.floor_div_nat(locy_raw(ball_height), pos2)),
    ))
};

pub let(*) tick = fun _(state: State, new_time: Time): State {
    // TODO
    match state {
        state(
            :window,
            :left_paddle_vy_sign,
            :right_paddle_vy_sign,

            time: old_time,
            left_paddle_y: old_left_paddle_y,
            right_paddle_y: old_right_paddle_y,
            ball_center_x: old_ball_center_x,
            ball_center_y: old_ball_center_y,
            ball_vx: old_ball_vx,
            ball_vy: old_ball_vy,
            
        ) => fun _(~elapsed_millis: Nat): State {
            State.state(
                :window,
                :left_paddle_vy_sign,
                :right_paddle_vy_sign,

                time: new_time,

                // TODO: Respond to keyboard input
                left_paddle_y: old_left_paddle_y,
                right_paddle_y: old_right_paddle_y,

                ball_center_x: locx(relu(int.add(
                    Int.nat(locx_raw(old_ball_center_x)),
                    int.floor_div(
                        int.mul(ilocx_raw(old_ball_vx), Int.nat(elapsed_millis)),
                        pos1000,
                    ),
                ))),
                ball_center_y: ball_bounce_y(
                    old_ball_center_y,
                    ilocy(int.floor_div(
                        int.mul(ilocy_raw(old_ball_vy), Int.nat(elapsed_millis)),
                        pos1000,
                    )),
                ),

                // TODO: Collision detection
                ball_vx: old_ball_vx,
                ball_vy: ilocy(sign_int(
                    ball_bounce_vy_sign(
                        old_ball_center_y,
                        ilocy(int.floor_div(
                            int.mul(ilocy_raw(old_ball_vy), Int.nat(elapsed_millis)),
                            pos1000,
                        )),
                    ),
                    ilocy_raw(old_ball_vy),
                )),
                
            )
        }(
            elapsed_millis: nat_sub(time_millis(new_time), time_millis(old_time)),
        ),
    }
};

pub let paddle_max_y = locy(nat_sub(nat10k, locy_raw(paddle_height)));

pub let clamp_paddle_y = fun _(yi: Int): LocalY {
    match int.lt(yi, int0) {
        true => locy(Nat.zero),
        false => match int.gt(yi, Int.nat(locy_raw(paddle_max_y))) {
            true => paddle_max_y,
            false => locy(relu(yi)),
        },
    }
};

pub let sign_eq = fun _(a: Sign, b: Sign): Bool {
    match a {
        pos => match b {
            pos => true,
            neg => false,
        },
        neg => match b {
            pos => false,
            neg => true,
        },
    }
};

pub let opt_sign_eq_some = fun _(opt_sign: Opt(Sign), sign: Sign): Bool {
    match opt_sign {
        none(_) => false,
        some(_, sign2) => sign_eq(sign, sign2),
    }
};

pub let handle_keydown = fun _(state: State, key: String): State {
    match state {
        state(
            :time,
            :window,
            left_paddle_y: old_left_paddle_y,
            right_paddle_y: old_right_paddle_y,
            :ball_center_x,
            :ball_center_y,
            :ball_vx,
            :ball_vy,
            left_paddle_vy_sign: old_left_paddle_vy_sign,
            right_paddle_vy_sign: old_right_paddle_vy_sign,
        ) => match str_list_contains(left_paddle_up_strs, key) {
            true => State.state(
                left_paddle_vy_sign: Opt.some(Sign, Sign.neg),
                left_paddle_y: match opt_sign_eq_some(old_left_paddle_vy_sign, Sign.neg) {
                    true => old_left_paddle_y,
                    false => clamp_paddle_y(
                        int.sub(
                            Int.nat(locy_raw(old_left_paddle_y)),
                            Int.nat(locy_raw(paddle_height)),
                        ),
                    ),
                },

                :time,
                :window,
                right_paddle_y: old_right_paddle_y,
                :ball_center_x,
                :ball_center_y,
                :ball_vx,
                :ball_vy,
                right_paddle_vy_sign: old_right_paddle_vy_sign,
            ),

            false => match str_list_contains(left_paddle_down_strs, key) {
                true => State.state(
                    left_paddle_vy_sign: Opt.some(Sign, Sign.pos),
                    left_paddle_y: match opt_sign_eq_some(old_left_paddle_vy_sign, Sign.pos) {
                        true => old_left_paddle_y,
                        false => clamp_paddle_y(
                            int.add(
                                Int.nat(locy_raw(old_left_paddle_y)),
                                Int.nat(locy_raw(paddle_height)),
                            ),
                        ),
                    },

                    :time,
                    :window,
                    right_paddle_y: old_right_paddle_y,
                    :ball_center_x,
                    :ball_center_y,
                    :ball_vx,
                    :ball_vy,
                    right_paddle_vy_sign: old_right_paddle_vy_sign,
                ),

                false => match str_list_contains(right_paddle_up_strs, key) {
                    true => State.state(
                        right_paddle_vy_sign: Opt.some(Sign, Sign.neg),
                        right_paddle_y: match opt_sign_eq_some(old_right_paddle_vy_sign, Sign.neg) {
                            true => old_right_paddle_y,
                            false => clamp_paddle_y(
                                int.sub(
                                    Int.nat(locy_raw(old_right_paddle_y)),
                                    Int.nat(locy_raw(paddle_height)),
                                ),
                            ),
                        },

                        :time,
                        :window,
                        left_paddle_y: old_left_paddle_y,
                        :ball_center_x,
                        :ball_center_y,
                        :ball_vx,
                        :ball_vy,
                        left_paddle_vy_sign: old_left_paddle_vy_sign,
                    ),

                    false => match str_list_contains(right_paddle_down_strs, key) {
                        true => State.state(
                            right_paddle_vy_sign: Opt.some(Sign, Sign.pos),
                            right_paddle_y: match opt_sign_eq_some(old_right_paddle_vy_sign, Sign.pos) {
                                true => old_right_paddle_y,
                                false => clamp_paddle_y(
                                    int.add(
                                        Int.nat(locy_raw(old_right_paddle_y)),
                                        Int.nat(locy_raw(paddle_height)),
                                    ),
                                ),
                            },

                            :time,
                            :window,
                            left_paddle_y: old_left_paddle_y,
                            :ball_center_x,
                            :ball_center_y,
                            :ball_vx,
                            :ball_vy,
                            left_paddle_vy_sign: old_left_paddle_vy_sign,
                        ),

                        false => state,
                    },
                },
            },
        },
    }
};

pub let handle_keyup = fun _(state: State, key: String): State {
    match state {
        state(
            :time,
            :window,
            :left_paddle_y,
            :right_paddle_y,
            :ball_center_x,
            :ball_center_y,
            :ball_vx,
            :ball_vy,
            :left_paddle_vy_sign,
            :right_paddle_vy_sign,
        ) => match str_list_contains(left_paddle_up_strs, key) {
            true => State.state(
                left_paddle_vy_sign: match left_paddle_vy_sign {
                    none(_) => left_paddle_vy_sign,
                    some(_, some_left_paddle_vy_sign) => match ascribe(Sign, some_left_paddle_vy_sign) {
                        neg => Opt.none(Sign),
                        pos => left_paddle_vy_sign,
                    }
                },

                :time,
                :window,
                :left_paddle_y,
                :right_paddle_y,
                :ball_center_x,
                :ball_center_y,
                :ball_vx,
                :ball_vy,
                :right_paddle_vy_sign,
            ),

            false => match str_list_contains(left_paddle_down_strs, key) {
                true => State.state(
                    left_paddle_vy_sign: match left_paddle_vy_sign {
                        none(_) => left_paddle_vy_sign,
                        some(_, some_left_paddle_vy_sign) => match ascribe(Sign, some_left_paddle_vy_sign) {
                            pos => Opt.none(Sign),
                            neg => left_paddle_vy_sign,
                        }
                    },

                    :time,
                    :window,
                    :left_paddle_y,
                    :right_paddle_y,
                    :ball_center_x,
                    :ball_center_y,
                    :ball_vx,
                    :ball_vy,
                    :right_paddle_vy_sign,
                ),

                false => match str_list_contains(right_paddle_up_strs, key) {
                    true => State.state(
                        right_paddle_vy_sign: match right_paddle_vy_sign {
                            none(_) => right_paddle_vy_sign,
                            some(_, some_right_paddle_vy_sign) => match ascribe(Sign, some_right_paddle_vy_sign) {
                                neg => Opt.none(Sign),
                                pos => right_paddle_vy_sign,
                            }
                        },

                        :time,
                        :window,
                        :left_paddle_y,
                        :right_paddle_y,
                        :ball_center_x,
                        :ball_center_y,
                        :ball_vx,
                        :ball_vy,
                        :left_paddle_vy_sign,
                    ),

                    false => match str_list_contains(right_paddle_down_strs, key) {
                        true => State.state(
                            right_paddle_vy_sign: match right_paddle_vy_sign {
                                none(_) => right_paddle_vy_sign,
                                some(_, some_right_paddle_vy_sign) => match ascribe(Sign, some_right_paddle_vy_sign) {
                                    pos => Opt.none(Sign),
                                    neg => right_paddle_vy_sign,
                                }
                            },

                            :time,
                            :window,
                            :left_paddle_y,
                            :right_paddle_y,
                            :ball_center_x,
                            :ball_center_y,
                            :ball_vx,
                            :ball_vy,
                            :left_paddle_vy_sign,
                        ),

                        false => state,
                    },
                },
            },
        },
    }
};

pub let handle = fun _(state: State, event: Event): State {
    match event {
        window_resize(_) => /* TODO */ state,
        keyup(key) => handle_keyup(state, key),
        keydown(key) => handle_keydown(state, key),
    }
};

pub let app = App.app(
    State,
    render,
    tick,
    init,
    handle,
);
