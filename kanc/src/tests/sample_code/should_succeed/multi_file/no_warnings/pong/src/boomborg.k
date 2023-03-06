use pack.std;
use std.list.List;
use std.num.Nat;
use std.string.String;
use std.time.Time;

pub type Entity {
    unscaled(x: Nat, y: Nat, image: String): Entity,
    scaled(x: Nat, y: Nat, w: Nat, h: Nat, image: String): Entity,
}

pub type Window {
    window(w: Nat, h: Nat): Window,
}

pub type Event {
    keydown(key: String): Event,
    keyup(key: String): Event,
    window_resize(window: Window): Event,
}

pub type App {
    app(
        State: Type,
        render: forall(state: State) { List(Entity) },
        tick: forall(state: State, time: Time) { State },
        init: forall(window: Window, time: Time) { State } ,
        handle: forall(state: State, event: Event) { State },
    ): App,
}
