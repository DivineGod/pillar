# pillar - a new column(1) for a modern terminal

This tool is a small replacement for column(1) which adds support for tty ansi codes
It will not make column gaps too wide if it encounters ansi codes for color etc.

`pillar` is currently not a full one-to-one replacement for column(1) and it doesn't
aim to be.

## Install

```
cargo install pillar
```

## Usage

`pillar` reads from stdin and outputs to stdout.

```
$ printf "a{b{c\n1{{3\n" | pillar
a b c
1   3
```

Currently there's no support for specifying the column delimiter or any other column(1) flags.
