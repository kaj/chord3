# chord3
[![Build Status](https://travis-ci.org/kaj/chord3.svg?branch=master)]
(https://travis-ci.org/kaj/chord3)

Chord3 takes a (set of) chopro file(s) and converts them to a single
pdf file.  If no file names are given as arguments, a single chopro
files is read from standard input.  Chopro files is simply text files
with chord names in brackets and some other options in braces, on
separate lines.

There is also an earlier, discontinued, project (implemented in
python), called [ChordLab](https://github.com/stacken/chordlab).

## Usage

The basic usage is:

```sh
chord3 --output songs.pdf song.chopro other_song.chopro ...
```

A full list of command line flags and options is given by:

```sh
chord3 --help
```

## Installation

If you have the rust toolchain installed, you can install the latest
release of chord3 with cargo:

```sh
cargo install chord3
```

Alternatively, if you run Fedora Linux there is
[a copr repo for chord3](https://copr.fedorainfracloud.org/coprs/kaj/chord3/)
which you can enable to install chord3 from a rpm:

```sh
dnf copr enable kaj/chord3
dnf install chord3
```

## License

Rasmus Kaj <rasmus@krats.se> wrote this program. As long as you retain this
notice you can do whatever you want with this stuff.  If we meet some day, and
you think this stuff is worth it, you can buy me a beer in return.
