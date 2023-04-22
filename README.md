# Battlesnake Rust Project

A set of Battlesnakes written in Rust. Get started at [play.battlesnake.com](https://play.battlesnake.com).

## Technologies Used

This project uses [Rust](https://www.rust-lang.org/) and [Rocket](https://rocket.rs). It also comes with an optional [Dockerfile](https://docs.docker.com/engine/reference/builder/) to help with deployment.

## Run Your Battlesnakes

```sh
cargo run
```

You should see the following output once it is running

```sh
🚀 Rocket has launched from http://0.0.0.0:8000
```

You can then make a request to your battlesnake's url using the URI `<snake>/<action>`, where `<snake>` is the name of your snake and `<action>` is the action you'd like to perform, one of `/`, `start`, `move`, or `end`.

## Create A New Battlesnake

1. Create a new `.rs` file under `/snakes` that matches the name of your battlesnake.
1. In the `/snakes/mod.rs` file, add a line to include your battlesnake.
1. Copy/paste the code from `/snakes/righty.rs` into your `/snakes/<snake>.rs` file. This battlesnake is very simple, works well for a template battlesnake.
1. Update the configuration in the `info` method to match how you want your battlesnake to be configured.
1. Update the `MOVE_URI` variable to include the name of your battlesnake instead of righty.
1. In the `main.rs` file, add four routes for your battlesnake: `/`, `/start`, `/move`, and `/end`. Mount them in the `rocket::build()` method.

## Test Your Battlesnakes

```sj
cargo test
```

You should see the following output once it is done running

```sh
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```
