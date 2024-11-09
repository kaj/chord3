use std::io::Result;

/// A key to transpose to and from
pub struct Key {
    /// The actual key, A = 0, Bb = 1, B = 2, etc.
    base: u8,
    /// Is this a major or minor key?
    maj: bool,
    /// The name of each tone in the scale from A to G#/Ab
    notes: &'static [&'static str; 12],
}

impl Key {
    pub fn new(key: &str) -> Result<Key> {
        let (maj, base) = match key.strip_suffix('m') {
            Some(base) => (false, base),
            None => (true, key),
        };
        let base = match base {
            "A" => 0,
            "A#" | "Bb" => 1,
            "B" => 2,
            "C" => 3,
            "C#" | "Db" => 4,
            "D" => 5,
            "D#" | "Eb" => 6,
            "E" => 7,
            "F" => 8,
            "F#" | "Gb" => 9,
            "G" => 10,
            "G#" | "Ab" => 11,
            _ => todo!("Unkown key {key:?}"),
        };
        let noteset = if key.contains('b') || !maj && !key.contains('#') {
            0
        } else {
            1
        };
        Ok(Key {
            base,
            maj,
            notes: &NOTE[noteset],
        })
    }
    pub fn from_nashville(&self, chord: &str) -> String {
        let (n, m) = parse_nashville(chord, self.maj);
        let note = self.notes[usize::from((self.base + n) % 12)];
        format!("{note}{m}")
    }
}

fn parse_nashville(chord: &str, maj: bool) -> (u8, &str) {
    let mut chars = chord.chars();
    let offset = match chars.clone().next() {
        Some('b') => {
            chars.next();
            11
        }
        Some('#') => {
            chars.next();
            1
        }
        _ => 0,
    };

    #[allow(unused_parens)]
    let num = match chars.next().unwrap_or('?') {
        '1' => 0,
        '2' => 2,
        '3' => (if maj { 4 } else { 3 }),
        '4' => 5,
        '5' => 7,
        '6' => (if maj { 9 } else { 8 }),
        '7' => (if maj { 11 } else { 10 }),
        c => panic!("Bad nashville chord base {c} in {chord:?}"),
    };
    ((num + offset) % 12, chars.as_str())
}

static NOTE: [[&str; 12]; 2] = [
    [
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ],
    [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ],
];
