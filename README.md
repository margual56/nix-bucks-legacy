[![Rust](https://github.com/margual56/nix-bucks/actions/workflows/rust.yml/badge.svg)](https://github.com/margual56/nix-bucks/actions/workflows/rust.yml)

# NixBucks
A simple budgeting app

![Demo](https://github.com/margual56/budgeting/assets/30444886/74378021-9cb7-4e22-9ea5-c512e6077c5c)

# Install
- Running the pre-compiled binary: Go to [the releases page](https://github.com/margual56/budgeting/releases/latest) and download the appropriate version for you.
- With cargo: `cargo install https://github.com/margual56/budgeting.git`.
- Clone and compile yourself: `git clone https://github.com/margual56/budgeting.git`, cd into the folder and `cargo build --release`.

## About windows
"Smart Screen" will yell at you when you try to open this program, because it is not signed by a "trusted" source.
FYI, in this case a "trusted" source means literally anyone who has $2,000/year to spend on a key to sign the program.

Since I don't have a disposable $2K/year, just click "Show more" > "Run anyway".

# Usage
Remember that you can back up the config file, and also you can create copies to test new arrangements :)

# Planning
## Goals
- Provide a simple way to track subscription costs, expenses and income
- Provide the bottomline information that we want to know (montly balance, how much money will I have, etc)
- Store the information locally, while allowing for easy manual editing, parsing and reading of it.
- Zero tracking, 100% local processing
- Use the least amount of resources possible
- Be accurate

## Non-goals
- Provide detailed information
- Do statistics with the data

## Planned features
- [x] Translation to Spanish (if you know more languages, please feel free to contribute)!
- [ ] A complete overhaul of the UI: New colors, new layout, new design. I hired a professional designer to design this (please consider donating).
- [ ] A small banner at the bottom that only shows up the second time that you open the program. Once you dismiss it, it won't show up again. It will ask the user to consider donating to the project.
- [ ] An icon for the application.
- [ ] A splash screen to organize profiles: save, copy, load, etc.
- [ ] A refactor of the code: Cleanup, documentation and optimizations.

# Help!
If you know Rust and think you can help, please do! :)

And if you don't, you can also [buy me a coffee](https://ko-fi.com/margual56). Even a small amount really helps.
