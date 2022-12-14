# albo-server

dev

```sh
./dev.sh
```

test

```sh
cargo test -- --nocapture
```

## notes to self / todo / goal

- goal is learning general "rust" -> "wasm" -> "use it from js" workflow
- do this by writing some webgl2 online game
- use wasm compiled from rust **at some point** in the process.

summary on plan:

- server

  1. [x] listen for player input
  2. [x] run game logic <- call wasm here
  3. [x] emit game state

- client

  1. [x] listen for keypresses and send to server
  2. [x] listen for game state update and draw it
  3. [ ] optional: proper interpolation of state clientside, "netcode rollback" etc

## comments

- client is essentially just "draw recieved things with webgl"
  - no need for rust-wasm there although it would be possible (but not beneficial?) via web-sys crate.
- lets pick wasm for game state update which is on server, which means in fact server could be pure rust.. but thats not the goal. In any case, go with the plan and wrap it it with a deno websocket server.

  - use some rust Entity Component System or write my own? probably write my own as learning experience.
    - I like the look of [Legion](https://github.com/amethyst/legion).
    - The full fledged framework would be [Bevy](https://github.com/bevyengine/bevy).

- When/if something heavier need to be done on client, can try rust-wasm there aswell, for example game state interpolation or something.

## references

[Legion docs](https://docs.rs/legion/latest/legion)
