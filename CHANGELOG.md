# Changelog

All notable changes to chord3 will be documented in this file.

The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this
project tries to adhere to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

* Added an option for landscape page format.
* Added an option for not writing out page numbers.


## Version 0.3.4

Released 2023-01-29.

* Added support for song-defined mandolin chords.
* Added `--base-size` command line argument, to allow changing the
  base font size in the generated pdf.
* Added `--no-duplex` option to disable verso and recto page design.
* Changed `--help` output again, by updating `clap` to 4.0.
* Added a changelog (this file).

### Internals

* Use rust edition 2021.
* Minor clippy fixes and rustfmt.


## Version 0.3.2

Released 2022-02-07.

* Added an option for mandolin chords (PR #1).
* Changed: More room for chords on the pages.
* Changed parameter handling to use `clap` 3.0.14 with derive feature
  (mainly a code cleanup, but gives some improvements to the `--help`
  output).
* Added crate categories and keywords metadata.
* Doc: Added crates.io badge to [`README.md`].
* Doc: Added a [`README.md`] section on how to install.
* Added a RPM spec file.

### Internals

* Update rustfmt.
* Use ? operator rather than try macro.
* Use `pdf-canvas`, since the crate name `pdf` is given to a project
  that aims for more complete pdf support.
* Update dependencies.
  - pdf-canvas 0.7.0
* Minor cleanups (partly suggested by clippy)


## Version 0.3.0

The first actual release.  This was done 2016-10-16.

* Added documentation, `chopro.md` to describe the file format, etc.
* Added more or less complete support for the chopro format.
* Added support for command line arguments (multiple input files,
  named output file, and some options to control the output).
* Added support for verso and recto pages (wider inner margins, page
  numbers in the "outer" margin) for double-sided printing.
* Improved output page format in many ways.
* Added basic pdf metadata to output, including document outline with
  song titles.
* Added more known chords, and replacements for some unknown chords.
* Specified dependency versions.

### Internals

* Lots of refactoring, improved error handling.
* Added continous integration on Travis.


## Prerelease

The initial commit of this project was done on friday 2015-09-25.
By wednesday 2016-09-30 I had the main functionality in a "works for
me" state.

Before this, I did a similar project in python, called
[ChordLab](https://github.com/stacken/chordlab).

Before that, there was a program called `chord`, that did the same
thing.
It was mainly implemented in postscript, with a small "driver" program
that mainly concattenated the postscript preamble with the chopro
files.
