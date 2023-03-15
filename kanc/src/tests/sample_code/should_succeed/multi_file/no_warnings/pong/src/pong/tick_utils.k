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
            Int.nat(int.trunc_div_nat(locy_raw(ball_height), pos2)),
        )),
        Int.nat(int.trunc_div_nat(locy_raw(ball_height), pos2)),
    )))
};

pub let ball_bounce_vy_sign_top = fun _(raw: Int): Sign {
    sign_neg_one_exp(match int_abs(raw) {
        zero => Nat.zero,
        pos(p) => nat.add(
            int.trunc_div_nat(
                nat_sub(Nat.pos(p), nat1),
                pos10k,
            ),
            match raw {
                neg(_) => nat.one,
                nat(_) => Nat.zero,
            },
        ),
    })
};

// TODO: Refactor
// Do we really need two params?
pub let ball_bounce_vy_sign = fun _(old_y: LocalY, delta_y: IntLocalY): Sign {
    ball_bounce_vy_sign_top(int.sub(
        int.add(Int.nat(locy_raw(old_y)), ilocy_raw(delta_y)),
        Int.nat(int.trunc_div_nat(locy_raw(ball_height), pos2)),
    ))
};

type Rect {
    new(x: LocalX, y: LocalY, w: LocalX, h: LocalY): Rect,
}

type Circ {
    new(x: LocalX, y: LocalY, r: LocalX): Circ,
}

type HitBox {
    rect(r: Rect): HitBox,
    circ(c: Circ): HitBox,
}

let do_rects_intersect = fun _(a: Rect, b: Rect): Bool {
    /*
    noOverlap = r1.x1 > r2.x2 or
                     r2.x1 > r1.x2 or
                     r1.y1 > r2.y2 or
                     r2.y1 > r1.y2

    return !noOverlap
    */

    match a {
        new(ax, ay, aw, ah) => match b {
            new(bx, by, bw, bh) => not(or(
                nat.gt(locx_raw(ax), nat.add(locx_raw(bx), locx_raw(bw))),
                or(
                    nat.gt(locx_raw(bx), nat.add(locx_raw(ax), locx_raw(aw))),
                    or(
                        nat.gt(locy_raw(ay), nat.add(locy_raw(by), locy_raw(bh))),
                        nat.gt(locy_raw(by), nat.add(locy_raw(ay), locy_raw(ah))),
                    ),
                ),
            )),
        },
    }
};

let do_rect_circ_intersect = fun _(rect: Rect, circ: Circ): Bool {
    /*
    // https://stackoverflow.com/a/1879223/7215455

    // clamp(value, min, max) - limits value to the range min..max

    // Find the closest point to the circle within the rectangle
    float closestX = clamp(circle.X, rectangle.Left, rectangle.Right);
    float closestY = clamp(circle.Y, rectangle.Top, rectangle.Bottom);

    // Calculate the distance between the circle's center and this closest point
    float distanceX = circle.X - closestX;
    float distanceY = circle.Y - closestY;

    // If the distance is less than the circle's radius, an intersection occurs
    float distanceSquared = (distanceX * distanceX) + (distanceY * distanceY);
    return distanceSquared < (circle.Radius * circle.Radius);
    */

    match rect {
        new(rx, ry, rw, rh) => match circ {
            new(cx, cy, cr) => nat.le(
                nat.add(
                    nat.square(nat_dist(
                        locx_raw(cx),
                        clamp(
                            clampee: locx_raw(cx),
                            min: locx_raw(rx),
                            max: nat.add(locx_raw(rx), locx_raw(rw)),
                        ),
                    )),
                    nat.square(nat_dist(
                        locy_raw(cy),
                        clamp(
                            clampee: locy_raw(cy),
                            min: locy_raw(ry),
                            max: nat.add(locy_raw(ry), locy_raw(rh)),
                        ),
                    )),
                ),
                nat.square(locx_raw(cr)),
            ),
        },
    }
};

let do_circs_intersect = fun _(a: Circ, b: Circ): Bool {
    match a {
        new(ax, ay, ar) => match b {
            new(bx, by, br) => nat.le(
                nat.add(
                    nat.square(nat_dist(locx_raw(ax), locx_raw(bx))),
                    nat.square(nat_dist(locy_raw(ay), locy_raw(by))),
                ),
                nat.square(nat.add(locx_raw(ar), locx_raw(br))),
            ),
        },
    }
};

let does_intersect = fun _(a: HitBox, b: HitBox): Bool {
    match a {
        rect(ar) => match b {
            rect(br) => do_rects_intersect(ar, br),
            circ(bc) => do_rect_circ_intersect(ar, bc),
        },
        circ(ac) => match b {
            rect(br) => do_rect_circ_intersect(br, ac),
            circ(bc) => do_circs_intersect(ac, bc),
        },
    }
};

let ball_hitbox = fun _(state: State): HitBox {
    match state {
        state(:ball_center_x, :ball_center_y, ...) => HitBox.rect(Rect.new(
            locx(nat_sub(
                locx_raw(ball_center_x),
                int.trunc_div_nat(locx_raw(get_ball_width(state)), pos2),
            )),
            locy(nat_sub(
                locy_raw(ball_center_y),
                int.trunc_div_nat(locy_raw(ball_height), pos2),
            )),
            get_ball_width(state),
            ball_height,
        )),
    }
};

let left_paddle_hitbox = fun _(state: State): HitBox {
    match state {
        state(:left_paddle_y, ...) => HitBox.rect(Rect.new(
            left_paddle_x,
            left_paddle_y,
            paddle_width,
            paddle_height,
        )),
    }
};

let right_paddle_hitbox = fun _(state: State): HitBox {
    match state {
        state(:right_paddle_y, ...) => HitBox.rect(Rect.new(
            right_paddle_x,
            right_paddle_y,
            paddle_width,
            paddle_height,
        )),
    }
};

pub let is_ball_touching_left_paddle = fun _(state: State): Bool {
    does_intersect(
        ball_hitbox(state),
        left_paddle_hitbox(state),
    )
};

pub let is_ball_touching_right_paddle = fun _(state: State): Bool {
    does_intersect(
        ball_hitbox(state),
        right_paddle_hitbox(state),
    )
};
