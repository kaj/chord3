# The chopro format
    
The chopro format is a pseudo-standard format for song lyrics with
chords.
The basic format is simply plain text with chords in brackets.
Some extra markup is added in braces, in the form
`{keyword: value}` (some keywords does not use a value).
    
A common example may look like this:

```chopro
{title: Yesterday}
{subtitle: Beatles}
{define: C/G base-fret 0 frets 3 3 2 0 1 0}

[G]Yesterday[F#m], all my [B7]troubles seemed so [Em]far away
[C]Now it [D]looks as though there [C]here to [G]stay
...
```

The core of the format is simply text lines, with chord names inserted
in brackets.

## Document header

`{title: [Title of the song]}` specifies the title of the song.
Can be abbreviated to `{t: ...}`.

`{subtitle: [subtitle]}` specifies anything that should be prominently
displayed under the title of the song.
Typically the composer / author / band, possibly with a year or record
name added.
A song can have multiple subtitles (but only one title).
Can be abbreviated to `{st: ...}`.

`{define: [chordname] base-fret [basefret] frets [e] [a] [d] [g] [b] [e]}`
Define how _chordname_ should be played.
_basefret_ is the fret where the barre is applied, or 0 if the chord
is not a barre chord.
The keyword `frets` is followed by the fret to press for each string
(minus _basefret_ if the chord is a barre chord).
`0` means the string is played open.
`x` means that the string is not played.

Each `define` is valid for the current song only.
(Chord3 has built-in definitions of more than 100 common chord
definitions plus aliases.)

`{columns: [n]}` specifies how many columns should be used for the
text of this song.
Defaults to 1.
Can be abbreviated to `{col: [n]}`.

## Song contents

The following tags can be used along with the regular text-and-chord lines:

`{comment: [text]}` is a line that is printed along with the text in a
different font.
The text might be instructions or any kind of interesting annotations.
`{c: ...}`, `{ci: ...}`, and `{cb: ...}` are aliases for `{comment: ...}`.

`{start_of_chorus}` (or `{soc}`) marks the start of the chorus.
The chorus itself consists of regular text-and-chord lines.
`{end_or_chorus}` (or `{eoc}`) marks the end of the chorus.
The start and end markers should be on separate lines.

`{start_of_tab}` and `{end_of_tab}` wrabs tabulature, which is currently
simply handled as preformatted (and monospaced) text.
These can be abbreviated as `{sot}` and `{eot}`.

`{colb}` is an explicit end of the current column
(and page, if on the last column).

`{page_break}` (or `{np}`) is an explicit page break.

`{new_song}` ends the current song and starts a new one
(by default, each separate file is a song).
