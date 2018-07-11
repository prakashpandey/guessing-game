## Guessing-game

Guessing game is a simple game made to learn rust.

## How it works?

- System generates a random number between [x-y]. 
  Vars `x, y` can be configured. Default `x=0, y=15`.
- User gets `5` chance to pridict this number
- If predicted correctly, user's score gets incremented

## How to run?

For linux, you can find a compiled executable at `/target/release/guessing_game`.
For windows, you can build it from source.

- `git clone https://github.com/prakashpandey/guessing-game`
- `cd guessing-game`
- `./target/release/guessing_game`

You can also add `guessing_game` to bashrc.

## How to build from source?
- `git clone https://github.com/prakashpandey/guessing-game`
- `cd guessing-game`
- `cargo build --release`

## Screenshots

<img src="/docs/media/screen-1.png" alt="Screen-1"/>
