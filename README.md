# Chat

A small prototype repo I made to mess with websockets and Rust. It probably has several security flaws and is highly incomplete.

## Why?

Why not?

## Usage

Firstly, rename `.env.example` and fill it out by providing a valid postgres dsn.
Once done, running the server should be as simple as running it using `cargo run` and then opening `static/index.html` twice in two seperate browser tabs to experiment.

## TODO

- [ ] Fix token secret handling
- [ ] Optimize message fetching by fetching author data in the same query

## Contributing

If you're working with database-related code, set the git hooks directory to `.githooks` using `git config core.hooksPath .githooks`. This ensures that the snapshot for sqlx is up to date.
