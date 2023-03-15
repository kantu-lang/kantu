# Boomborg Pong

This is an extremely simple showcase game built with
the Boomborg Game Engine.

## How to play

- As soon as the page loads, the ball will start moving.
- Move the paddles with `W`/`S` (for the left player)
  and `Up Arrow`/`Down Arrow` (for the right player).
- If the ball goes off the left or right edge,
  the ball will be reset to the center, and the game
  will pause.
  - Press `Space` to relaunch the ball.

## How to build the project

Required tools:

1. [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (most Rust users will already have this installed)
2. [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

Instructions:

1. `cd` into this directory.
2. Run `cargo run --release`.
3. Open `target/index.js` (relative to this directory), and scroll
   down to the **bottom of the file**.
   1. The last line should begin with `const app = `.
   2. Add `export` before the `const`. That is, the resulting line
      should now start with `export const app = `.
4. Copy this code into `<kantu_repo_root_dir>/boomborg/src/app.js`.
5. `cd` into `<kantu_repo_root_dir>/boomborg`.
6. Run `npm install`.
7. Run `npm start`.
   1. This should print a localhost address in the console.
      Open that address in your web browser.
8. Play.

## IF YOU'RE NOT A `kanc` DEV, NO NEED TO READ FURTHER

The rest of this document is only written for kanc devs.

## Goals of this project

1. Demonstrate that we can build a game in the Kantu programming language.
2. Serve as a sample Kantu codebase that we can use for
   speeding up kanc.

## Non-goals

1. Be a fun game.
   I mean, let's be real--it's pong.
2. Be an example of idiomatic/clean Kantu code.

## Main learnings

1. It seems like compile times were _massively_ sped up
   using opaque `let`s.
   This is probably because it...
   1. Avoids rederiving types, and
   2. Reduces cost of shifting/substitution
      (since the alias is a leaf node, whereas the referent
      is probably a much deeper subtree).
2. It's a _huge_ pain to have to duplicate a ton of fields. For example:

   ```kantu
   let increment_b = fun _(s: State): State {
       match s {
           new(:a, :b, :c, :d, :e, :f) => State.new(
               b: nat.succ(b),
               :a,
               :c,
               :d,
               :e,
               :f,
           ),
       }
   };
   ```

   We need a spread operator that would let us do
   something like:

   ```kantu
   let increment_b = fun _(s: State): State {
       match s {
           new(:b, ...) => State.new(
               // Spread operator using `s`:
               ...s,
               b: nat.succ(b),
           ),
       }
   };
   ```

   I still need to work out the exact rules for this,
   though.
   It's a bit trickier with multiple variants and
   dependent types.

3. It's also a pain (but not as huge) to have to write a `match`
   just to access a single field.
   **So, we should introduce dot syntax.**
4. It's also a pain (my, I'm getting repetitive) to have to
   manually construct constants.
   **So, we should introduce number, string, and list literals.**
5. The `<LOCATION_NOT_FOUND>` for error message traces are
   quite irritating.
   On a "small" project like this, it's
   usually possible to eventually figure
   out where the problematic code is.
   But for anything larger, lack of file/line number information
   essentially renders kanc (nearly) unusable.
   **So, we need to track "original expressions" at each step of the eval process**.
6. Type conversion (e.g., `Int` to `Nat`,
   or even trivial conversions like `Nat` to `Int`)
   code
   makes the codebase a lot harder to read.
   Maybe we should support coercions.
   Although this would greatly complicate the language.
7. Operator syntax would be nice.
8. if-elif-else syntax would be nice,
   but it's a very low priority at the moment.
