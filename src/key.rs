use std::io::Result;

/// A key to transpose to and from
pub struct Key {
    /// The actual key, A = 0, Bb = 1, B = 2, etc.
    base: u8,
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
            "G" => 10,
            _ => todo!("Unkown key {key:?}"),
        };
        Ok(Key { base })
    }
    pub fn from_nashville(&self, chord: &str) -> String {
        let (n, m) = parse_nashville(chord);
        // TODO: Note names differs on key!
        // An A# in one key is called Bb in another.
        let note = NOTE[usize::from((self.base + n) % 12)];
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
        c => todo!("Bad chord base {c} in {chord:?}"),
    };
    (num, chars.as_str())
}

static NOTE: [&str; 12] = [
    "A", "Bb", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
