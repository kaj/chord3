use clap::ValueEnum;
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet};
use std::vec::Vec;

#[derive(PartialEq, Debug, Copy, Clone, ValueEnum)]
pub enum Instrument {
    /// Guitar in e-a-d-g-b-e tuning without capo.
    Guitar,
    /// Mandolin in g-d-a-e tuning.
    Mandolin,
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument::Guitar
    }
}

pub struct ChordHolder {
    unknown_chord: &'static Vec<i8>,
    known_chords: &'static BTreeMap<&'static str, Vec<i8>>,
    local: BTreeMap<String, Vec<i8>>,
    used: BTreeSet<String>,
}

impl ChordHolder {
    pub fn new_for(instrument: Instrument) -> Self {
        match instrument {
            Instrument::Guitar => ChordHolder {
                unknown_chord: &UNKNOWN_CHORD,
                known_chords: &KNOWN_CHORDS,
                local: BTreeMap::new(),
                used: BTreeSet::new(),
            },
            Instrument::Mandolin => ChordHolder {
                unknown_chord: &UNKNOWN_MANDOLIN_CHORD,
                known_chords: &KNOWN_MANDOLIN_CHORDS,
                local: BTreeMap::new(),
                used: BTreeSet::new(),
            },
        }
    }
    pub fn use_chord(&mut self, chord: &str) {
        if !(chord.is_empty()
            || chord == "NC"
            || chord == "N.C."
            || chord == "%"
            || chord == "-"
            || chord.starts_with('/')
            || chord.starts_with('x'))
        {
            self.used.insert(chord.to_string());
        }
    }
    pub fn define(&mut self, chord: String, def: Vec<i8>) {
        if def.len() == self.unknown_chord.len() {
            self.local.insert(chord, def);
        } else {
            println!("Ignoring chord def {chord}, wrong instrument");
        }
    }
    pub fn get_used(&self) -> Vec<(&str, &Vec<i8>)> {
        self.used
            .iter()
            .map(|name| {
                (
                    name as &str,
                    self.local
                        .get(name)
                        .or_else(|| self.known_chords.get(name as &str))
                        .or_else(|| {
                            ChordHolder::replacement(name).and_then(|repl| {
                                self.known_chords.get(&repl as &str)
                            })
                        })
                        .unwrap_or_else(|| {
                            println!("Warning: Unknown chord {name}");
                            self.unknown_chord
                        }),
                )
            })
            .collect()
    }

    pub fn get_all_chords(&self) -> Vec<(&str, &Vec<i8>)> {
        self.known_chords.iter().map(|(a, b)| (*a, b)).collect()
    }

    fn replacement(name: &str) -> Option<String> {
        if let Some(opts) = name.strip_prefix('H') {
            Some(format!("B{opts}"))
        } else if name.len() >= 2 {
            match &name[..2] {
                "A#" => Some(format!("Bb{}", &name[2..])),
                "D#" => Some(format!("Eb{}", &name[2..])),
                "Gb" => Some(format!("F#{}", &name[2..])),
                "Cb" => Some(format!("B{}", &name[2..])),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[test]
fn test_simple_chord() {
    let mut test = ChordHolder::new_for(Instrument::Guitar);
    test.use_chord("Am");
    test.use_chord("E");
    assert_eq!(
        vec![
            ("Am", &vec![0, -1, 0, 2, 2, 1, 0]),
            ("E", &vec![0, 0, 2, 2, 1, 0, 0]),
        ],
        test.get_used()
    )
}

#[test]
fn test_override_chord() {
    let mut test = ChordHolder::new_for(Instrument::Guitar);
    test.define("Am".to_string(), vec![5, 1, 3, 3, 1, 1, 1]);
    test.use_chord("Am");
    test.use_chord("E");
    assert_eq!(
        vec![
            ("Am", &vec![5, 1, 3, 3, 1, 1, 1]),
            ("E", &vec![0, 0, 2, 2, 1, 0, 0]),
        ],
        test.get_used()
    )
}

#[test]
fn test_nochord_and_unknown() {
    let mut test = ChordHolder::new_for(Instrument::Guitar);
    test.use_chord("N.C.");
    test.use_chord("Smaj9");
    assert_eq!(
        vec![("Smaj9", &vec![0, -2, -2, -2, -2, -2, -2])],
        test.get_used()
    )
}

lazy_static! {
    static ref UNKNOWN_CHORD: Vec<i8> = vec![0,-2,-2,-2,-2,-2,-2];
    static ref KNOWN_CHORDS: BTreeMap<&'static str, Vec<i8>> = {
    let mut result = BTreeMap::new();
    {
        let mut chord = |name: &'static str, base_fret: i8,
                         e: i8, a: i8, d: i8, g: i8, b: i8, e2: i8| {
            result.insert(name, vec!(base_fret, e, a, d, g, b, e2));
        };
        let x = -1;
        chord("Ab",     4,   1, 3, 3, 2, 1, 1);
        chord("Ab6",    1,   4, 3, 1, 1, 1, 1);
        chord("Ab7",    4,   1, 3, 1, 2, 1, 1);
        chord("Abm",    4,   1, 3, 3, 1, 1, 1);
        chord("Abm7",   4,   1, 3, 1, 1, 1, 1);
        chord("Abmaj7", 4,   1, 3, 2, 2, 1, 1);
        chord("A",      0,   x, 0, 2, 2, 2, 0);
        chord("A/E",    0,   0, 0, 2, 2, 2, 0);
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

    static ref UNKNOWN_MANDOLIN_CHORD: Vec<i8> = vec![0,-2,-2,-2,-2];
    static ref KNOWN_MANDOLIN_CHORDS: BTreeMap<&'static str, Vec<i8>> = {
    let mut result = BTreeMap::new();
    {
        let mut chord = |name: &'static str,
                         g: i8, d: i8, a: i8, e: i8| {
            result.insert(name, vec!(0, g, d, a, e));
                         };
        let x = -1;
        chord("Ab",    1, 1, 3, 4); // also 7 6 3 4 or 1 1 3 x

        chord("A",     9, 7, 4, 5); // also 2 2 4 5 or 2 2 4 x
        chord("A7",    6, 5, 7, 5); // also 6 5 0 0
        chord("Am",    9, 7, 3, 5); // also 2 2 3 5 or 5 7 7 x
        chord("Am7",   0, 2, 3, 0); // also 2 2 3 3

        chord("Bb",    3, 3, 5, 7); // also 10 8 5 6 or 3 3 5 x

        chord("B",     4, 4, 6, 7); // also 11 9 6 7
        chord("B7",    2, 1, 2, x); // also  8 7 9 7
        chord("Bm",    4, 4, 5, 7); // also 11 9 5 7
        chord("Bm7",   4, 4, 5, 5); // also  2 4 5 2

        chord("C",     5, 2, 3, 0);
        chord("C7",    3, 2, 3, x); // also 9 8 10 8
        chord("Cm",    5, 1, 3, x); // also 0 1 3 x or 5 5 6 8
        chord("Cm7",   3, 1, 3, x); // also 5 5 6 6
        chord("Cmaj7", 5, 2, 2, 3); // also 5 5 7 7
        chord("C/G",   0, 2, 3, 0);

        chord("C#",    6, 3, 4, 1); // also 1 3 4 x or 6 3 4 x
        chord("C#7",   4, 3, 4, x);
        chord("C#m",   6, 2, 4, x); // also 6 6 7 9

        chord("D",     2, 0, 0, 2); // also 7 4 5 2 or 2 4 5 x
        chord("D7",    5, 4, 5, x);
        chord("Dm",    7, 3, 5, x); // also 2 3 5 x
        chord("Dm7",   5, 3, 5, x);

        chord("Eb",    8, 5, 6, 3); // also 8 5 6 x or 3 5 6 x
        chord("Eb7",   3, 1, 4, 3); // also 6 5 6 x

        chord("E",     4, 6, 7, x); // also 9 6 7 4
        chord("E7",    7, 6, 7, x); // also 4 2 5 4 or 4 6 5 x
        chord("Em",    4, 2, 2, 3); // also 4 5 7 0
        chord("Em7",   4, 2, 5, 3); // also 7 5 7 x

        chord("F",     5, 3, 0, 1); // also 10 7 8 6 or 5 7 8 x
        chord("F7",    5, 3, 6, 5); // also  8 7 8 x or 2 1 3 1
        chord("Fm",    5, 3, 3, 4);
        chord("Fmaj7", 5, 3, 7, 5); // also 10 7 7 8

        chord("F#",    6, 4, 1, 2); // also 11 8 9 6 or 6 8 9 x
        chord("F#7",   6, 4, 7, 6); // also  3 2 4 2 or 9 8 9 x
        chord("F#m",   6, 4, 4, 5); // also  2 4 4 x
        chord("F#m7",  6, 4, 7, 5);

        chord("G",     0, 0, 2, 3); // also  7 5 2 3 or 7 9 10 7
        chord("G7",    5, 4, 6, 4); // also  7 5 8 7
        chord("Gm",    0, 0, 1, 3); // also  7 5 1 3 or 3 5 5 x
        chord("Gm7",   7, 5, 8, 6);

        chord("G#",    8, 6, 3, 4); // also 1 1 3 4 or 1 1 3 x
    }
    result
    };
}
