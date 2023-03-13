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
