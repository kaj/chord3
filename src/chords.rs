use std::vec::Vec;
use std::collections::BTreeMap;

pub fn get_known_chords() -> BTreeMap<String, Vec<i8>> {
    let mut result = BTreeMap::new();
    {
        let mut chord = |name: &str, base_fret: i8, e: i8, a: i8, d: i8, g: i8, b: i8, e2: i8| {
            result.insert(name.to_string(), vec!(base_fret, e, a, d, g, b, e2));
        };
        chord("A",     0,  -1,  0,  2,  2,  2,  0);
        chord("Am",    0,  -1,  0,  2,  2,  1,  0);
        chord("Am6",   1,  -1,  0,  2,  2,  1,  2);
        chord("Am9",   5,   1,  3,  1,  1,  1,  3);
        chord("Am/C",  1,  -1,  3,  2,  2,  1,  0);
        chord("Am/E",  1,   0,  0,  2,  2,  1,  0);
        chord("Am/F#", 1,   2,  0,  2,  2,  1,  0);
        chord("Am/G",  1,   3,  0,  2,  2,  1,  0);
        chord("A6",    1,  -1,  0,  2,  2,  2,  2);
        chord("A7",    0,  -1,  0,  2,  0,  2,  0);
        chord("A7",    1,  -1,  0,  2,  0,  2,  0);
        chord("A7/C#", 4,  -1,  1,  2,  3,  2,  2);
        chord("A9",    4,   2,  1,  2,  1,  2, -1);
        chord("Am7",   1,  -1,  0,  2,  0,  1,  0);
        chord("Am7/G", 1,   3,  0,  2,  0,  1,  0);
        chord("Amaj7", 1,  -1,  0,  2,  1,  2,  0);
        chord("Asus",  1,  -1,  0,  2,  2,  3,  0);
        chord("Asus2", 1,  -1,  0,  2,  2,  0,  0);
        chord("Asus4", 1,  -1,  0,  0,  2,  3,  0);
        chord("C",     0,  -1,  3,  2,  0,  1,  0);
        chord("Cmaj7", 0,  -1,  3,  2,  0,  0,  0);
        chord("D",     0,  -1, -1,  0,  2,  3,  2);
        chord("Dm",    0,  -1, -1,  0,  2,  3,  1);
        chord("D7",    0,  -1, -1,  0,  2,  1,  2);
        chord("Dmaj7", 0,  -1, -1,  0,  2,  2,  2);
        chord("Dm7",   0,  -1, -1,  0,  2,  1,  1);
        chord("Dsus4", 0,  -1, -1,  0,  2,  3,  3);
        chord("D/A",   0,  -1,  0,  0,  2,  3,  2);
        chord("D/F#",  0,   2,  0,  0,  2,  3,  2);
        chord("D#dim", 0,  -1, -1,  1,  2,  4,  2);
        chord("E",     0,   0,  2,  2,  1,  0,  0);
        chord("Em",    0,   0,  2,  2,  0,  0,  0);
        chord("E7",    0,   0,  2,  2,  1,  3,  0);
        chord("F",     0,   1,  3,  3,  2,  1,  1);
        chord("Fmaj7", 0,   1,  3,  2,  2,  1,  1);
        chord("G",     0,   3,  2,  0,  0,  0,  3);
        chord("G+",    0,   3,  2,  1,  0,  0,  3);
        chord("G6",    0,   3,  2,  0,  0,  0,  0);
        chord("G7",    0,   3,  2,  0,  0,  0,  1);
        chord("G7/D",  0,  -1, -1,  0,  0,  0,  1);
        chord("Gm",    3,   1,  3,  3,  1,  1,  1);
        chord("Gm6",   0,   3,  1,  0,  0,  3,  0);
        chord("Gm7",   3,   1,  3,  1,  1,  1,  1);
        chord("Gmaj7", 0,   3,  2,  0,  0,  0,  2);
        chord("Gsus4", 0,   3,  3,  0,  0,  3,  3);
        chord("Bm",    2,  -1,  1,  3,  3,  2,  1);
        chord("Bm7",   2,  -1,  1,  3,  1,  2,  1);
        chord("B7",    2,  -1,  1,  3,  1,  3,  1);
    }
    result
}
