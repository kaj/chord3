extern crate pdf;
extern crate regex;

use pdf::{Canvas, Pdf, FontSource};
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io;
use std::vec::Vec;
use std::collections::{BTreeSet, BTreeMap};
use std::env;

fn chordbox<'a>(c: &mut Canvas<'a, File>, left: f32, top: f32,
                name: &str, strings: &Vec<i8>)
                -> io::Result<()> {
    let dx = 5.0;
    let dy = 7.0;
    let right = left + 5.0 * dx;
    let bottom = top - 4.4 * dy;
    try!(c.center_text(left + 2.0 * dx, top + dy,
                       FontSource::Helvetica_Oblique, 12.0, name));
    let barre = strings[0];
    let up =
        if barre < 2 {
            try!(c.set_line_width(1.0));
            try!(c.line(left-0.15, top+0.5, right+0.15, top+0.5));
            try!(c.stroke());
            0.0
        } else {
            try!(c.right_text(left - 0.2 * dx, top - 0.9 * dy,
                              FontSource::Helvetica, dy, &format!("{}", barre)));
            1.6
        };
    try!(c.set_line_width(0.3));
    for b in 0..5 {
        let y = top - b as f32 * dy;
        try!(c.line(left, y, right, y));
    }
    for s in 0..6 {
        let x = left + s as f32 * dx;
        try!(c.line(x, top+up, x, bottom));
    }
    try!(c.stroke());
    let radius = 1.4;
    let above = top + 2.0 + radius;
    for s in 0..6 {
        let x = left + s as f32 * dx;
        match strings[s+1] {
            -2 => (), // No-op for unknown chord
            -1 => {
                let (l, r) = (x-radius, x+radius);
                let (t, b) = (above-radius, above+radius);
                try!(c.line(l, t, r, b));
                try!(c.line(r, t, l, b));
                try!(c.stroke());
            }
            0 => {
                try!(c.circle(x, above, radius));
                try!(c.stroke());
            }
            y => {
                let y = top - (y as f32 - 0.5) * dy;
                try!(c.circle(x, y, radius+0.4));
                try!(c.fill());
            }
        }
    }
    Ok(())
}

fn get_known_chords() -> BTreeMap<String, Vec<i8>> {
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

enum ChordFileExpression {
    Title{s: String},
    SubTitle{s: String},
    Comment{s: String},
    Line{s: Vec<String>}
}

impl ChordFileExpression {
    fn parse(line: &str) -> ChordFileExpression {
        let comment_re = Regex::new(r"^\s*#").unwrap();
        let re = Regex::new(r"\{(?P<cmd>\w+)(?::\s*(?P<arg>.*))?}").unwrap();
        if comment_re.is_match(line) {
            ChordFileExpression::Comment{s: "".to_string()}
        } else if let Some(caps) = re.captures(line) {
            let arg = caps.name("arg").unwrap_or("").to_string();
            match caps.name("cmd").unwrap() {
                "t" | "title" => ChordFileExpression::Title{s: arg},
                "st" | "subtitle" => ChordFileExpression::SubTitle{s:arg},
                "c" => ChordFileExpression::Comment{s:arg},
                x => {
                    println!("unknown expression {}", x);
                    ChordFileExpression::Comment{s:caps.at(0).unwrap().to_string()}
                }
            }
        } else {
            let mut s = vec!();
            let re = Regex::new(r"([^\[]*)(?:\[([^\]]*)\])?").unwrap();
            for caps in re.captures_iter(line) {
                s.push(caps.at(1).unwrap().to_string());
                if let Some(chord) = caps.at(2) {
                    s.push(chord.to_string());
                }
            }
            ChordFileExpression::Line{s: s}
        }
    }
}

fn main() {
    let mut file = File::create("foo.pdf").unwrap();
    let source = io::BufReader::new(File::open(env::args().nth(1).unwrap_or("../chord/c/creedence/DownOnTheCorner.chopro".to_string()))
        .unwrap());
    let mut document = Pdf::new(&mut file).unwrap();
    let (width, height) = (596.0, 842.0);
    let known_chords = get_known_chords();
    document.render_page(width, height, |c| {
        let mut y = height - 30.0;
        let left = 50.0;
        let times_bold = c.get_font(FontSource::Times_Bold);
        let times_italic = c.get_font(FontSource::Times_Italic);
        let times = c.get_font(FontSource::Times_Roman);
        let chordfont = c.get_font(FontSource::Helvetica_Oblique);
        let mut used_chords : BTreeSet<String> = BTreeSet::new();
        for line in source.lines() {
            let token = ChordFileExpression::parse(&line.unwrap());
            try!(match token {
                ChordFileExpression::Title{s} => c.text(|t| {
                    y = y - 20.0;
                    try!(t.set_font(times_bold, 18.0));
                    try!(t.pos(left, y));
                    t.show(&s)
                }),
                ChordFileExpression::SubTitle{s} => c.text(|t| {
                    y = y - 18.0;
                    try!(t.set_font(times_italic, 16.0));
                    try!(t.pos(left, y));
                    t.show(&s)
                }),
                ChordFileExpression::Comment{s} => c.text(|t| {
                    y = y - 14.0;
                    try!(t.set_font(times_italic, 14.0));
                    try!(t.pos(left, y));
                    t.show(&s)
                }),
                ChordFileExpression::Line{s} => c.text(|t| {
                    let text_size = 14.0;
                    let chord_size = 10.0;
                    y = y - 1.4 * (text_size + chord_size);
                    try!(t.set_font(times, text_size));
                    try!(t.pos(left, y));
                    let (mut last_chord, mut last_text) = (-600, 0);
                    for (i, part) in s.iter().enumerate() {
                        if i % 2 == 1 {
                            used_chords.insert(part.to_string());
                            try!(t.gsave());
                            try!(t.set_rise(14.0));
                            try!(t.set_font(chordfont, chord_size));
                            let ahead = last_text - last_chord - 278;
                            // TODO One show_j should be enough, but sometimes
                            // with two args and sometimes with three!
                            if ahead < 0 {
                                try!(t.show_j("", ahead));
                            }
                            last_chord = FontSource::Helvetica_Oblique
                                .get_width_raw(&part) as i32;
                            try!(t.show_j(&part, last_chord));
                            try!(t.grestore());
                        } else {
                            try!(t.show(&part));
                            last_text =
                                (FontSource::Times_Roman
                                 .get_width_raw(&part) as f32
                                 * text_size / chord_size) as i32;
                        }
                    }
                    Ok(())
                })
            })
        }
        // Remove non-chords that are displayed like chords above the text.
        used_chords.remove("%");
        used_chords.remove("");
        let mut x = width - used_chords.len() as f32 * 40.0;
        for chord in used_chords.iter() {
            if let Some(chorddef) = known_chords.get(chord) {
                try!(chordbox(c, x, 80.0, chord, chorddef));
            } else {
                println!("Warning: Unknown chord '{}'.", chord);
                try!(chordbox(c, x, 80.0, chord, &vec!(0,-2,-2,-2,-2,-2,-2,-2)));
            }
            x = x + 40.0;
        }
        Ok(())
    }).unwrap();
    document.finish().unwrap();
}
