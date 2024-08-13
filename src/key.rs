use std::io::Result;

/// A key to transpose to and from
pub struct Key {
    /// The actual key, A = 0, Bb = 1, B = 2, etc.
    base: u8,
    notes: &'static [&'static str; 12],
}

impl Key {
    pub fn new(key: &str) -> Result<Key> {
        let base = match key {
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
        Ok(Key { base, notes: &NOTE[if key.contains('b') { 0 } else { 1 } ] })
    }
    pub fn from_nashville(&self, chord: &str) -> String {
        let (n, m) = parse_nashville(chord);
        // TODO: Note names differs on key!
        // An A# in one key is called Bb in another.
        let note = self.notes[usize::from((self.base + n) % 12)];
        format!("{note}{m}")
    }
}

fn parse_nashville(chord: &str) -> (u8, &str) {
    let mut chars = chord.chars();
    let num = match chars.next().unwrap_or('?') {
        '1' => 0,
        '2' => 2,
        '3' => 4,
        '4' => 5,
        '5' => 7,
        '6' => 9,
        '7' => 10, // TODO: Is a nashville 7 big or small? 10 or 11?
        c => todo!("Bad nashville chord base {c} in {chord:?}"),
    };
    (num, chars.as_str())
}

static NOTE: [[&str; 12];2] = [
    [
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ],
    [
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ],
];
