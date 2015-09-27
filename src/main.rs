extern crate pdf;
extern crate regex;

use pdf::{Canvas, Pdf, FontSource};
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io;
use std::vec::Vec;
use std::collections::{BTreeSet, BTreeMap};

fn chordbox<'a>(c: &mut Canvas<'a, File>, left: f32, top: f32,
                name: &str, strings: &Vec<i8>)
                -> io::Result<()> {
    let dx = 5.0;
    let dy = 7.0;
    let right = left + 5.0 * dx;
    let bottom = top - 4.4 * dy;
    let times = c.get_font(FontSource::Times_Roman);
    try!(c.text(|t| {
        try!(t.set_font(times, 12.0));
        try!(t.pos(left, top+dy));
        t.show(name)
    }));
    let barre = strings[0];
    let up =
        if barre < 2 {
            try!(c.set_line_width(1.0));
            try!(c.line(left-0.15, top+0.5, right+0.15, top+0.5));
            try!(c.stroke());
            0.0
        } else {
            let font = c.get_font(FontSource::Helvetica);
            try!(c.text(|t| {
                try!(t.set_font(font, dy));
                try!(t.pos(left - dx, top - 0.9 * dy));
                t.show(&format!("{}", barre))
            }));
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
        chord("Am",  0,  -1,  0, 2, 2, 1, 0);
        chord("C",   0,  -1,  3, 2, 0, 1, 0);
        chord("D",   0,  -1, -1, 0, 2, 3, 2);
        chord("F",   0,   1,  3, 3, 2, 1, 1);
        chord("G",   0,   3,  2, 0, 0, 0, 3);
        chord("Bm7", 2,  -1,  1, 3, 1, 2, 1);
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
        let re = Regex::new(r"\{(?P<cmd>\w+)(?::\s*(?P<arg>.*))?}").unwrap();
        if let Some(caps) = re.captures(line) {
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
    let source = io::BufReader::new(File::open("../chord/c/creedence/DownOnTheCorner.chopro")
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
                    y = y - 30.0;
                    try!(t.set_font(times, 16.0));
                    try!(t.pos(left, y));
                    for (i, part) in s.iter().enumerate() {
                        if i % 2 == 1 {
                            used_chords.insert(part.to_string());
                            try!(t.gsave());
                            try!(t.set_rise(14.0));
                            try!(t.set_font(chordfont, 14.0));
                            try!(t.show(&part));
                            try!(t.grestore());
                        } else {
                            try!(t.show(&part));
                        }
                    }
                    Ok(())
                })
            })
        }
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
