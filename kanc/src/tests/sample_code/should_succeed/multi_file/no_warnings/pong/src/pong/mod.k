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

mod local_coords;
use local_coords.*;

pub let refl_true = Equal.refl(Bool, true);

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

mod math_utils;
use math_utils.*;

pub let window_width = fun _(window: Window): Nat {
    match window {
        window(w, _) => w,
    }
};

pub let window_height = fun _(window: Window): Nat {
    match window {
        window(_, h) => h,
    }
};

pub let time_millis = fun _(t: Time): Nat {
    match t {
        time(millis) => millis,
    }
};

pub mod literals;
use literals.*;

pub let paddle_height = locy(int.trunc_div_nat(
    nat10k,
    pos7,
));
pub let ball_height = locy(int.trunc_div_nat(
    nat10k,
    pos64,
));
pub let paddle_x_margin = locx(int.trunc_div_nat(
    nat10k,
    pos64,
));
pub let paddle_width = locx(int.trunc_div_nat(
    nat10k,
    pos64,
));
pub let left_paddle_x = paddle_x_margin;
pub let right_paddle_x = locx(nat_sub(
    nat_sub(
        nat10k,
        locx_raw(paddle_x_margin),
    ),
    locx_raw(paddle_width),
));
pub let ball_initial_vx = ilocx(int.trunc_div(
    Int.nat(nat10k),
    pos4,
));
pub let ball_initial_vy = ilocy(int.trunc_div(
    Int.nat(nat10k),
    pos3,
));
pub let paddle_max_y = locy(nat_sub(
    nat10k,
    locy_raw(paddle_height),
));

pub let clamp_paddle_y = fun _(yi: Int): LocalY {
    match int.lt(yi, int0) {
        true => locy(Nat.zero),
        false => match int.gt(yi, Int.nat(locy_raw(paddle_max_y))) {
            true => paddle_max_y,
            false => locy(relu(yi)),
        },
    }
};

pub let to_real_x = fun _(x: LocalX, window: Window): Nat {
    match window {
        window(window_w, _) => int.trunc_div_nat(
            nat.mul(locx_raw(x), window_w),
            pos10k,
        ),
    }
};
pub let to_real_y = fun _(y: LocalY, window: Window): Nat {
    match window {
        window(_, window_h) => int.trunc_div_nat(
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
            left_paddle_y: locy(nat_sub(nat5000, int.trunc_div_nat(locy_raw(paddle_height), pos2))),
            right_paddle_y: locy(nat_sub(nat5000, int.trunc_div_nat(locy_raw(paddle_height), pos2))),
            ball_center_x: locx(nat5000),
            ball_center_y: locy(nat5000),
            ball_vx: ball_initial_vx,
            ball_vy: ball_initial_vy,
            left_paddle_vy_sign: Opt.none(Sign),
            right_paddle_vy_sign: Opt.none(Sign),
        )
    }
};

mod render_utils;
use render_utils.*;

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

mod tick_utils;
use tick_utils.*;

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
                    int.trunc_div(
                        int.mul(ilocx_raw(old_ball_vx), Int.nat(elapsed_millis)),
                        pos1000,
                    ),
                ))),
                ball_center_y: ball_bounce_y(
                    old_ball_center_y,
                    ilocy(int.trunc_div(
                        int.mul(ilocy_raw(old_ball_vy), Int.nat(elapsed_millis)),
                        pos1000,
                    )),
                ),

                ball_vx: match is_ball_touching_left_paddle(state) {
                    true => ilocx(Int.nat(int_abs(ilocx_raw(old_ball_vx)))),
                    false => match is_ball_touching_right_paddle(state) {
                        true => ilocx(int.neg(Int.nat(int_abs(ilocx_raw(old_ball_vx))))),
                        false => old_ball_vx,
                    },
                },
                ball_vy: ilocy(sign_int(
                    ball_bounce_vy_sign(
                        old_ball_center_y,
                        ilocy(int.trunc_div(
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

mod handle_utils;
use handle_utils.*;

pub let handle = fun _(state: State, event: Event): State {
    match event {
        window_resize(new_window) => handle_window_resize(state, new_window),
        keyup(key) => handle_keyup(state, key),
        keydown(key) => handle_keydown(state, key),
    }
};

// We set the transparency scope to `pack`
// as a hack to speed up compilation.
// Currently, the compiler doesn't cache
// type derivation results, so if it expands
// an alias to its referent, it will have to
// re-typecheck the referent all over again.
// By setting the transparency scope to `pack`,
// we prevent alias variables declared in this
// module from being expanded, thereby avoiding
// the need to re-typecheck the referent.
pub let(pack) app = App.app(
    State,
    render,
    tick,
    init,
    handle,
);
