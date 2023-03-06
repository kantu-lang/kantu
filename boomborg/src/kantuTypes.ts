export type Int = ["neg", Pos] | ["nat", Nat];

export type Nat = ["zero"] | ["pos", Pos];

export type Pos = ["one"] | ["extend", Pos, Bit];

export type Bit = ["zero"] | ["one"];

export type U8 = ["u8", Nat, unknown];

export type Bool = ["true"] | ["false"];

export type Time = ["time", Nat];

export type Window = ["window", Nat, Nat];

export type Entity =
  | ["unscaled", Nat, Nat, KString]
  | ["scaled", Nat, Nat, Nat, Nat, KString];

export type KString = ["string", List<U8>];

export type List<T> = ["nil", unknown] | ["cons", unknown, T, List<T>];

export type App<S> = [
  "app",
  RenderFn<S>,
  UpdateFn<S>,
  InitFn<S>,
  HandleEventFn<S>
];

export type RenderFn<S> = (state: S) => List<Entity>;

export type UpdateFn<S> = (state: S, time: Time) => S;

export type InitFn<S> = (w: Window, time: Time) => S;

export type HandleEventFn<S> = (state: S, event: Event) => S;

export type Event =
  | ["keyup", KString]
  | ["keydown", KString]
  | ["window_resize", Window];
