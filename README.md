<div align="center">
  <h1>iced_tutorial_app</h1>
  <img src="assets/vsm-usage.gif">
</div>
<br>
<div align="center">
  <img alt="Standard Readme" src="https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square">
  <br>
  <p>Trying to make a iced-rs app which is well documented in an effort to help the lack of documentation for the gui framework</p>
</div>

## Table of Contents

- [Background](#background)
- [Development](#development)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Background

Trying to find a good gui toolkit in rust is hard, the popular retained mode Gtk4 rust bindings are nice, but are overly complicated
and is not fun to use. Egui is very nice but I wanted to learn iced, since egui has far superior documentation and examples, I wanted to
contribute in some meaningfull way to iced documentation. As well as creating a clear and concise example application that attempts to use
many features. The code organization and clippy lints are very opinionated but are used to keep everything clean but not overly complicated.
This "repo" was inspired by [German Nikolishins totorial series](https://nikolish.in/gs-with-iced-1), it is basically the only tutorial available.
So this is my attempt to expand upon it.

### Disclaimer

I am in no way a iced or rust expert (I using this repo to learn iced).
If you think there is a better way to do something or explain something in this repo,
please open a PR.

## Development

Only requires `just` to bootstrap all tools and configuration.
Some other tools will be required, listed below.

```bash
# dependecies that 'just' will need.
sudo pacman -S python-pip
pip install pre-commit --user
sudo pacman -S openssl-1.1
cargo install just
just init # setup repo, install hooks and all required tools
```

To run:

```bash
just run
```

To test:

```bash
just test
```

Before committing your work:

```bash
just pre-commit
```

To see all available commands:

```bash
just list
```

## Maintainers

[@mattcoding4days](https://github.com/mattcoding4days)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

This project is licensed under:

- [MIT © 2023 Matt Williams](LICENSE)
