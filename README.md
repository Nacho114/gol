
# gol

A [Zellij](https://zellij.dev) plugin for enjoying the Conway's [game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

![usage](https://github.com/Nacho114/gol/raw/main/img/usage.gif)

## Usage

- `s` start
- `r` reset
- `Up` and `Down` or `j` and `k` to change the density of the random initial population
- `Left` and `Right` or `h` and `l` to change the time speed

## Why?

Why not? 

More seriously, it is actually quite fun to play around with the game of life to get a feel for the idea
of [emergence](https://en.wikipedia.org/wiki/Emergence), as you play with different speeds and densities
you will notice that certain configurations seem "noisy" while simply changing the speed will bring a 
certain beautiful order out of the noise. 

## Installation

You'll need [rust](https://rustup.rs/) installed.

- `git clone git@github.com:Nacho114/gol.git`
- `cd gol`
- `cargo build --release`
- `mkdir -p ~/.config/zellij/plugins/`
- `mv target/wasm32-wasi/release/gol.wasm ~/.config/zellij/plugins/`

## Keybinding

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html)
somewhere inside the [keybinds](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl y" {
        LaunchOrFocusPlugin "file:~/.config/zellij/plugins/gol.wasm" {
            floating true
        }
    }
}
```

> You likely already have a `shared_except "locked"` section in your configs. Feel free to add `bind` there.## Contributing

If you find any issues or want to suggest ideas please [open an issue](https://github.com/Nacho114/gol/issues/new).

### Development

Make sure you have [rust](https://rustup.rs/) installed then run:

```sh
zellij action new-tab --layout ./dev.kdl
```

### Testing

To run tests:

```sh
cargo test --target aarch64-apple-darwin -- --nocapture
```

> Replace the target the appropriate cpu architecture.

