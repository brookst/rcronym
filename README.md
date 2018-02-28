# rcronym

A reddit bot shamelessly reimplementing [Decronym](http://decronym.xyz/) but in
[Rust](https://www.rust-lang.org/) and for the
[/r/rust subreddit](https://www.reddit.com/r/rust/).

## Installation

    cargo install diesel
    git clone https://github.com/brookst/rcronym.git
    cd rcronym
    echo "DATABASE_URL=postgres://rcronym@localhost/rcronym" > .env
    diesel setup

## Usage

The command line interface can be used to add and remove acronyms with the `add` and `rm`
commands. The `add` command requires a `--key` argument for the acronym, an optional `--regex`
argument to customise the matching, and consumes the explanation on `stdin`.

Acronyms in the database can be listed with the `list` command. The id number of each acronym
can be used to remove it with `rm --id`.

The most recent comments in the `/r/rust` subreddit are fetched, and any matches with acronyms
in the database are printed.

A blanket upper-case regex can be used to look for acronyms in use with the `candidates`
command.


License: AGPL-3.0
