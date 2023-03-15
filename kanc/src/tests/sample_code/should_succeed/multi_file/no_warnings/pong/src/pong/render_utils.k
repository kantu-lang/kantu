use super.*;

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
            to_real_x(left_paddle_x, window),
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
            to_real_x(right_paddle_x, window),
            to_real_y(right_paddle_y, window),
            to_real_x(paddle_width, window),
            to_real_y(paddle_height, window),
            paddle_image_str,
        ),
    }
};

pub let get_ball_width = fun _(state: State): LocalX {
    match state {
        state(:window, ...) => locx(int.floor_div_nat(
            nat.mul(locy_raw(ball_height), window_height(window)),
            nat_to_pos(window_width(window)),
        )),
    }
};

pub let render_ball = fun _(state: State): Entity {
    match state {
        state(:window, :ball_center_x, :ball_center_y, ...) => Entity.scaled(
            nat_sub(
                to_real_x(ball_center_x, window),
                int.floor_div_nat(
                    to_real_x(get_ball_width(state), window),
                    pos2,
                ),
            ),
            nat_sub(
                to_real_y(ball_center_y, window),
                int.floor_div_nat(
                    to_real_y(ball_height, window),
                    pos2,
                ),
            ),
            to_real_x(get_ball_width(state), window),
            to_real_y(ball_height, window),
            ball_image_str,
        ),
    }
};
