use std::vec::Vec;
use std::collections::BTreeMap;

lazy_static! {
    pub static ref KNOWN_CHORDS: BTreeMap<String, Vec<i8>> = {
    let mut result = BTreeMap::new();
    {
        let mut chord = |name: &str, base_fret: i8, e: i8, a: i8, d: i8, g: i8, b: i8, e2: i8| {
            result.insert(name.to_string(), vec!(base_fret, e, a, d, g, b, e2));
        };
        let x = -1;
        chord("Ab",     4,   1, 3, 3, 2, 1, 1);
        chord("Ab6",    1,   4, 3, 1, 1, 1, 1);
        chord("Ab7",    4,   1, 3, 1, 2, 1, 1);
        chord("Abm",    4,   1, 3, 3, 1, 1, 1);
        chord("Abm7",   4,   1, 3, 1, 1, 1, 1);
        chord("Abmaj7", 4,   1, 3, 2, 2, 1, 1);
        chord("A",      0,   x, 0, 2, 2, 2, 0);
        chord("A6",     1,   x, 0, 2, 2, 2, 2);
        chord("A7",     0,   x, 0, 2, 0, 2, 0);
        chord("A7/C#",  4,   x, 1, 2, 3, 2, 2);
        chord("A9",     4,   2, 1, 2, 1, 2, x);
        chord("Am",     0,   x, 0, 2, 2, 1, 0);
        chord("Am/C",   1,   x, 3, 2, 2, 1, 0);
        chord("Am/E",   1,   0, 0, 2, 2, 1, 0);
        chord("Am/F#",  1,   2, 0, 2, 2, 1, 0);
        chord("Am/G",   1,   3, 0, 2, 2, 1, 0);
        chord("Am6",    1,   x, 0, 2, 2, 1, 2);
        chord("Am7",    1,   x, 0, 2, 0, 1, 0);
        chord("Am7/G",  1,   3, 0, 2, 0, 1, 0);
        chord("Am9",    5,   1, 3, 1, 1, 1, 3);
        chord("Amaj7",  1,   x, 0, 2, 1, 2, 0);
        chord("Asus",   1,   x, 0, 2, 2, 3, 0);
        chord("Asus2",  1,   x, 0, 2, 2, 0, 0);
        chord("Asus4",  1,   x, 0, 0, 2, 3, 0);

        chord("Bb",     1,   x, 1, 3, 3, 3, 1);
        chord("Bbm",    1,   x, 1, 3, 3, 2, 1);
        chord("Bb7",    1,   x, 1, 3, 1, 3, 1);
        chord("Bbm7",   1,   x, 1, 3, 1, 2, 1);
        chord("Bb9'",   1,   x, 1, 0, 1, 1, 1);
        chord("Bb/E",   1,   0, 1, 3, 3, 3, 1);
        chord("Bb/D",   1,   x, x, 0, 2, 2, 1);
        chord("B",      2,   x, 1, 3, 3, 3, 1);
        chord("B6",     2,   x, 1, 3, 3, 3, 3);
        chord("B7",     2,   x, 1, 3, 1, 3, 1);
        chord("Bdim",   1,   x, 2, 3, 4, 3, x);
        chord("Bm",     2,   x, 1, 3, 3, 2, 1);
        chord("Bm/F#",  2,   1, 1, 3, 3, 2, 1);
        chord("Bm/G#",  4,   1, 2, 1, 4, 4, 4);
        chord("Bm7",    2,   x, 1, 3, 1, 2, 1);
        chord("Bmaj7",  2,   x, 1, 3, 2, 3, 1);
        chord("Bsus4",  2,   x, 1, 1, 3, 4, 1);

        chord("C",      0,   x, 3, 2, 0, 1, 0);
        chord("C/B",    1,   x, 2, 2, 0, 1, 0);
        chord("C/D",    1,   x, x, 0, 0, 1, 0);
        chord("C/E",    1,   0, 3, 2, 0, 1, 0);
        chord("C/G",    1,   3, 3, 2, 0, 1, 0);
        chord("C6",     1,   x, 3, 2, 2, 1, 0);
        chord("C7",     1,   x, 3, 2, 3, 1, 0);
        chord("C7sus4", 1,   x, 3, 3, 3, 1, 1);
        chord("Cdim",   3,   x, 1, 2, 3, 2, x);
        chord("Cm",     3,   x, 1, 3, 3, 2, 1);
        chord("Cm7",    3,   x, 1, 3, 1, 2, 1);
        chord("Cmaj7",  0,   x, 3, 2, 0, 0, 0);
        chord("Cmaj7",  1,   x, 3, 2, 0, 0, 0);
        chord("Csus4",  1,   x, 3, 3, 0, 1, 1);
        chord("C#",     4,   x, 1, 3, 3, 3, 1);
        chord("C#7",    4,   x, 1, 3, 1, 3, 1);
        chord("C#m",    4,   x, 1, 3, 3, 2, 1);
        chord("C#m7",   4,   x, 1, 3, 1, 2, 1);
        chord("C#sus4", 1,   x, 4, 4, 1, 2, 2);

        chord("Db",     1,   x, 4, 3, 1, 2, 1);
        chord("D",      0,   x, x, 0, 2, 3, 2);
        chord("D/A",    0,   x, 0, 0, 2, 3, 2);
        chord("D/F#",   0,   2, 0, 0, 2, 3, 2);
        chord("D7",     0,   x, x, 0, 2, 1, 2);
        chord("Dm",     0,   x, x, 0, 2, 3, 1);
        chord("Dm7",    0,   x, x, 0, 2, 1, 1);
        chord("Dmaj7",  0,   x, x, 0, 2, 2, 2);
        chord("Dsus4",  0,   x, x, 0, 2, 3, 3);
        chord("D#dim",  0,   x, x, 1, 2, 4, 2);

        chord("Eb",     3,   x, 4, 3, 1, 2, 1);
        chord("Eb7",    0,   x, x, 1, 0, 2, 3);
        chord("Ebm",    0,   x, x, 1, 3, 4, 2);
        chord("E",      0,   0, 2, 2, 1, 0, 0);
        chord("E/G#",   0,   x, x, x, 1, 0, 0);
        chord("E7",     0,   0, 2, 2, 1, 3, 0);
        chord("E9",     0,   0, 2, 0, 1, 0, 2);
        chord("Eadd9",  0,   0, 2, 2, 1, 3, 3);
        chord("Em",     0,   0, 2, 2, 0, 0, 0);
        chord("Em6",    0,   0, 2, 2, 0, 2, 0);
        chord("Em7",    0,   0, 2, 2, 0, 3, 0);
        chord("Emaj7",  0,   0, 2, 1, 1, 0, 0);
        chord("Esus4",  0,   0, 0, 2, 2, 0, 0);

        chord("F",      0,   1, 3, 3, 2, 1, 1);
        chord("F/A",    0,   x, 0, 3, 2, 1, 1);
        chord("F7",     0,   1, 3, 1, 2, 1, 1);
        chord("Fm",     0,   1, 3, 3, 1, 1, 1);
        chord("Fmaj7",  0,   1, 3, 2, 2, 1, 1);
        chord("Fsus4",  0,   1, 1, 3, 3, 1, 1);
        chord("F#",     2,   1, 3, 3, 2, 1, 1);
        chord("F#7",    2,   1, 3, 1, 2, 1, 1);
        chord("F#dim",  0,   x, x, 4, 2, 1, 2);
        chord("F#dim*", 0,   2, 3, 4, 2, x, x);
        chord("F#m",    2,   1, 3, 3, 1, 1, 1);
        chord("F#m7",   2,   1, 3, 1, 1, 1, 1);

        chord("G",      0,   3, 2, 0, 0, 0, 3);
        chord("G+",     0,   3, 2, 1, 0, 0, 3);
        chord("G/A",    0,   x, 0, 0, 0, 0, 3);
        chord("G/B",    0,   x, 2, 0, 0, 3, 3);
        chord("G/C",    0,   x, 3, 0, 0, 0, 3);
        chord("G/F",    0,   1, 2, 0, 0, 3, 3);
        chord("G/F#",   0,   2, 2, 0, 0, 0, 3);
        chord("G6",     0,   3, 2, 0, 0, 0, 0);
        chord("G7",     0,   3, 2, 0, 0, 0, 1);
        chord("G7/D",   0,   x, x, 0, 0, 0, 1);
        chord("Gm",     3,   1, 3, 3, 1, 1, 1);
        chord("Gm6",    0,   3, 1, 0, 0, 3, 0);
        chord("Gm7",    3,   1, 3, 1, 1, 1, 1);
        chord("Gmaj7",  0,   3, 2, 0, 0, 0, 2);
        chord("Gsus4",  0,   3, 3, 0, 0, 3, 3);
        chord("G#",     4,   1, 3, 3, 2, 1, 1);
        chord("G#7",    4,   1, 3, 1, 2, 1, 1);
        chord("G#dim",  4,   1, 2, 3, 1, x, x);
        chord("G#m",    4,   1, 3, 3, 1, 1, 1);
        chord("G#m7",   4,   1, 3, 1, 1, 1, 1);
    }
    result
    };
}
