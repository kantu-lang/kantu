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

let do_rects_itersect = fun _(a: Rect, b: Rect): Bool {
    // TODO
    false
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
    // TODO
    false
};

let does_intersect = fun _(a: HitBox, b: HitBox): Bool {
    match a {
        rect(ar) => match b {
            rect(br) => do_rects_itersect(ar, br),
            circ(bc) => do_rect_circ_intersect(ar, bc),
        },
        circ(ac) => match b {
            rect(br) => do_rect_circ_intersect(br, ac),
            circ(bc) => do_circs_intersect(ac, bc),
        },
    }
};

let ball_hitbox = fun _(state: State): HitBox {
    // TODO
    HitBox.rect(Rect.new(locx(Nat.zero), locy(Nat.zero), locx(Nat.zero), locy(Nat.zero)))
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
    // TODO
    HitBox.rect(Rect.new(locx(Nat.zero), locy(Nat.zero), locx(Nat.zero), locy(Nat.zero)))
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
