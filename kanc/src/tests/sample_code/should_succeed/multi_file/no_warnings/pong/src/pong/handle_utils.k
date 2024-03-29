use super.*;

pub let handle_window_resize = fun _(state: State, new_window: Window): State {
    match state {
        state(
            window: old_window,

            :time,
            :left_paddle_y,
            :right_paddle_y,
            :ball_center_x,
            :ball_center_y,
            :ball_vx,
            :ball_vy,
            :left_paddle_vy_sign,
            :right_paddle_vy_sign,
        ) => State.state(
            window: new_window,

            :time,
            :left_paddle_y,
            :right_paddle_y,
            :ball_center_x,
            :ball_center_y,
            :ball_vx,
            :ball_vy,
            :left_paddle_vy_sign,
            :right_paddle_vy_sign,
        ),
    }
};

pub let is_ball_stationary_at_center = fun _(state: State): Bool {
    match state {
        state(
            :ball_center_x,
            :ball_center_y,
            :ball_vx,
            :ball_vy,
            ...
        ) => and(
            int.eq(int0, ilocx_raw(ball_vx)),
            and(
                int.eq(int0, ilocy_raw(ball_vy)),
                and(
                    nat.eq(nat5000, locx_raw(ball_center_x)),
                    nat.eq(nat5000, locy_raw(ball_center_y)),
                ),
            ),
        ),
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

                        false => match str_list_contains(launch_ball_strs, key) {
                            true => match is_ball_stationary_at_center(state) {
                                true => State.state(
                                    ball_vx: ilocx(negate_on_1_bit(
                                        negatee: ilocx_raw(ball_initial_vx),
                                        negator: time_millis(time),
                                    )),
                                    ball_vy: ilocy(negate_on_2_bit(
                                        negatee: ilocy_raw(ball_initial_vy),
                                        negator: time_millis(time),
                                    )),

                                    :window,
                                    :time,
                                    :ball_center_x,
                                    :ball_center_y,
                                    left_paddle_y: old_left_paddle_y,
                                    right_paddle_y: old_right_paddle_y,
                                    left_paddle_vy_sign: old_left_paddle_vy_sign,
                                    right_paddle_vy_sign: old_right_paddle_vy_sign,
                                ),
                                false => state,
                            },
                            false => state,
                        },
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
