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

pub let paddle_height = locx(int.floor_div_nat(
    nat10k,
    pos8,
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
pub let ball_initial_vx = locx(int.floor_div_nat(
    nat10k,
    pos3,
));
pub let ball_initial_vy = locy(int.floor_div_nat(
    nat10k,
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
        ~ball_vx: LocalY,
        ~ball_vy: LocalY,
        ~left_paddle_vy_sign: Opt(Sign),
        ~right_paddle_vy_sign: Opt(Sign),
    ): State,
}

pub let init = fun _(window: Window, time: Time): State {
    match window {
        window(window_w, window_h) => State.state(
            :time,
            :window,

            // PROBLEM: paddle_height is LocY, not Nat.
            left_paddle_y: nat_sub(nat5000, int.floor_div_nat(paddle_height, pos2)),
            right_paddle_y: nat_sub(nat5000, int.floor_div_nat(paddle_height, pos2)),
            
            ball_center_x: nat5000,
            ball_center_y: nat5000,
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
            nat_sub(window_width(window), to_real_x(paddle_x_margin, window)),
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
                nat_sub(ball_center_x, ball_radius),
                nat_sub(ball_center_y, ball_radius),
                nat.mul(nat2, ball_radius),
                nat.mul(nat2, ball_radius),
                ball_image_str,
            )
        }(int.floor_div_nat(
            to_real_x(ball_height, window),
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

pub let tick = fun _(state: State, new_time: Time): State {
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
                    old_ball_vx,
                ))),
                ball_center_y: locy(relu(int.add(
                    Int.nat(locy_raw(old_ball_center_y)),
                    old_ball_vy,
                ))),

                // TODO: Collision detection
                ball_vx: old_ball_vx,
                ball_vy: old_ball_vy,
                
            )
        }(
            elapsed_millis: nat_sub(time_millis(new_time), time_millis(old_time)),
        ),
    }
};

pub let handle_keydown = fun _(state: State, key: String): State {
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
                left_paddle_vy_sign: Opt.some(Sign, Sign.neg),

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
                    left_paddle_vy_sign: Opt.some(Sign, Sign.pos),

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
                        right_paddle_vy_sign: Opt.some(Sign, Sign.neg),

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
                            right_paddle_vy_sign: Opt.some(Sign, Sign.pos),

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
