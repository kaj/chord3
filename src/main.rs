extern crate pdf;
extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate clap;

use pdf::{Canvas, Pdf, FontSource};
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io;
use std::vec::Vec;
use std::sync::Mutex;

mod chords;
use ::chords::ChordHolder;

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
            try!(c.right_text(left - 0.4 * dx, top - 0.9 * dy,
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

enum ChordFileExpression {
    Title{s: String},
    SubTitle{s: String},
    Comment{s: String},
    ChordDef{name: String, def: Vec<i8>},
    Chorus{lines: Vec<ChordFileExpression>},
    EndOfChorus,
    Tab{lines: Vec<String>},
    EndOfTab,
    Line{s: Vec<String>}
}

struct ChoproParser<R: io::Read> {
    source: Mutex<io::Lines<io::BufReader<R>>>
}

impl ChoproParser<File> {
    fn open(path: &str) -> io::Result<ChoproParser<File>> {
        let f = try!(File::open(path));
        Ok(ChoproParser::new(f))
    }
}
impl<R: io::Read> ChoproParser<R> {
    fn new(source: R) -> ChoproParser<R> {
        let reader = io::BufReader::new(source);
        ChoproParser {
            source: Mutex::new(reader.lines())
        }
    }

    // Internal: Return the next line that is not a comment
    fn nextline(&mut self) -> Option<String> {
        loop {
            match self.source.lock().unwrap().next() {
                Some(Ok(line)) => {
                    let comment_re = Regex::new(r"^\s*#").unwrap();
                    if !comment_re.is_match(&line) {
                        return Some(line)
                    }
                },
                Some(Err(e)) => {
                    println!("Failed to read source: {}", e);
                    return None
                },
                _ => {
                    return None
                }
            }
        }
    }
}

impl<R: io::Read> Iterator for ChoproParser<R> {
    type Item = ChordFileExpression;

    fn next(&mut self) -> Option<ChordFileExpression> {
        if let Some(line) = self.nextline() {
            let re = Regex::new(r"\{(?P<cmd>\w+)(?::?\s*(?P<arg>.*))?\}").unwrap();
            if let Some(caps) = re.captures(&line) {
                let arg = caps.name("arg").unwrap_or("").to_string();
                match caps.name("cmd").unwrap() {
                    // TODO This should be a caseless match
                    // but since I fail to do that, just allow uppercase
                    // in some more or less random places ...
                    "title" | "t" | "Title"
                        => Some(ChordFileExpression::Title{s: arg}),
                    "subtitle" | "st"
                        => Some(ChordFileExpression::SubTitle{s:arg}),
                    "comment" | "c" | "ci" | "cb"
                        => Some(ChordFileExpression::Comment{s:arg}),
                    "define" => {
                        //println!("Parse chord def '{}'", arg);
                        let re = Regex::new(r"(?i)^([\S]+)\s+base-fret\s+([0-9]+)\s+frets(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))\s*$").unwrap();
                        if let Some(caps) = re.captures(&arg) {
                            let s = |n| {
                                //println!("String {} is {:?}", n,
                                //         caps.at(n as usize+2));
                                match caps.at(n as usize+2) {
                                    Some("x") | Some("X") |
                                    Some("-") | None => -1,
                                    Some(s) => s.parse::<i8>().unwrap(),
                                }
                            };
                            Some(ChordFileExpression::ChordDef {
                                name: caps.at(1).unwrap().to_string(),
                                def: vec!(s(0),
                                          s(1), s(2), s(3),
                                          s(4), s(5), s(6))
                            })
                        } else {
                            let whole = caps.at(0).unwrap();
                            println!("Warning: Bad chord definition {}", whole);
                            Some(ChordFileExpression::Comment{s:whole.to_string()})
                        }
                    },
                    "soc" | "start_of_chorus" => {
                        let mut lines = vec!();
                        while let Some(line) = self.next() {
                            match line {
                                ChordFileExpression::EndOfChorus => { break; },
                                line => lines.push(line)
                            }
                        }
                        Some(ChordFileExpression::Chorus{
                            lines: lines
                        })
                    }
                    "eoc" | "end_of_chorus" =>
                        Some(ChordFileExpression::EndOfChorus),
                    "sot" | "start_of_tab" => {
                        let mut lines = vec!();
                        let end =
                            Regex::new(r"\{(eot|end_of_tab):?\s*").unwrap();
                        while let Some(line) = self.nextline() {
                            if end.is_match(&line) {
                                break;
                            } else {
                                lines.push(line)
                            }
                        }
                        Some(ChordFileExpression::Tab{
                            lines: lines
                        })
                    }
                    "eot" | "end_of_tab" =>
                        Some(ChordFileExpression::EndOfTab),
                    x => {
                        println!("unknown expression {}", x);
                        Some(ChordFileExpression::Comment{s:caps.at(0).unwrap().to_string()})
                    }
                }
            } else {
                let mut s = vec!();
                let re = Regex::new(r"([^\[]*)(?:\[([^\]]*)\])?").unwrap();
                for caps in re.captures_iter(&line.replace("\t", "    ")) {
                    s.push(caps.at(1).unwrap().to_string());
                    if let Some(chord) = caps.at(2) {
                        s.push(chord.to_string());
                    }
                }
                Some(ChordFileExpression::Line{s: s})
            }
        } else {
            None
        }
    }
}


fn main() {
    let args = clap_app!(myapp =>
                         (version: env!("CARGO_PKG_VERSION"))
                         (author: "Rasmus Kaj <rasmus@krats.se>")
                         (about: "Create pdf songbooks from chopro source.")
                         (@arg OUTPUT: -o --output +takes_value
                          "Output PDF file name (default chords.pdf)")
                         (@arg TITLE: --title +takes_value
                          "Title (in metadata) of the output PDF file")
                         (@arg AUTHOR: --author +takes_value
                          "Author (in metadata) of the output PDF file")
                         (@arg INPUT: +required +multiple
                          "Chopro file(s) to parse")
                         ).get_matches();

    let filename = args.value_of("OUTPUT").unwrap_or("chords.pdf");
    let mut file = File::create(filename).unwrap();
    let mut document = Pdf::new(&mut file).unwrap();
    document.set_title(args.value_of("TITLE").unwrap_or("Songbook"));
    if let Some(author) = args.value_of("AUTHOR") {
        document.set_author(author);
    }
    document.set_producer(concat!("chord3 version ",
                                  env!("CARGO_PKG_VERSION"),
                                  "\nhttps://github.com/kaj/chord3"));
    for name in args.values_of("INPUT").unwrap() {
        if let Err(e) = render_song(&mut document, name.to_string()) {
            println!("Failed to handle {}: {}", name, e);
        }
    }
    document.finish().unwrap();
}

fn render_song<'a>(document: &mut Pdf<'a, File>, songfilename: String)
                   -> io::Result<()> {
    let source = try!(ChoproParser::open(&songfilename));
    let (width, height) = (596.0, 842.0);
    document.render_page(width, height, |c| {
        let mut y = height - 20.0;
        let left = 50.0;
        //let times_bold = c.get_font(FontSource::Times_Bold);
        //let times_italic = c.get_font(FontSource::Times_Italic);
        //let times = c.get_font(FontSource::Times_Roman);
        //let chordfont = c.get_font(FontSource::Helvetica_Oblique);
        let mut chords = ChordHolder::new();
        for token in source {
            y = try!(render_token(token, y, left, c, &mut chords));
        }
        let used_chords = chords.get_used();
        let mut x = width - used_chords.len() as f32 * 40.0;
        for (chord, chorddef) in used_chords {
            try!(chordbox(c, x, 80.0, chord, chorddef));
            x = x + 40.0;
        }
        Ok(())
    })
}

fn render_token<'a>(token: ChordFileExpression, y: f32, left: f32,
                    c: &mut Canvas<'a, File>, chords: &mut ChordHolder)
                    -> io::Result<f32> {
    let times_bold = c.get_font(FontSource::Times_Bold);
    let times_italic = c.get_font(FontSource::Times_Italic);
    let times = c.get_font(FontSource::Times_Roman);
    let chordfont = c.get_font(FontSource::Helvetica_Oblique);
    let tabfont = c.get_font(FontSource::Courier);
    match token {
        ChordFileExpression::Title{s} => {
            c.add_outline(&s);
            c.text(|t| {
                let y = y - 18.0;
                try!(t.set_font(&times_bold, 16.0));
                try!(t.pos(left, y));
                try!(t.show(&s));
                Ok(y)
            })
        },
        ChordFileExpression::SubTitle{s} => c.text(|t| {
            let y = y - 16.0;
            try!(t.set_font(&times_italic, 14.0));
            try!(t.pos(left, y));
            try!(t.show(&s));
            Ok(y)
        }),
        ChordFileExpression::Comment{s} => c.text(|t| {
            let y = y - 12.0;
            try!(t.set_font(&times_italic, 12.0));
            try!(t.pos(left, y));
            try!(t.show(&s));
            Ok(y)
        }),
        ChordFileExpression::ChordDef{name, def} => {
            chords.define(name, def);
            Ok(y)
        },
        ChordFileExpression::Chorus{lines} => {
            let mut y2 = y;
            for line in lines {
                y2 = try!(render_token(line, y2, left + 10.0, c, chords));
            }
            y2 = y2 - 4.0;
            try!(c.set_line_width(0.5));
            try!(c.line(left - 6.0, y, left - 6.0, y2));
            try!(c.stroke());
            Ok(y2)
        }
        ChordFileExpression::EndOfChorus => {
            println!("Warning: Stray end of chorus in song!");
            Ok(y)
        }
        ChordFileExpression::Tab{lines} => c.text(|t| {
            let mut y = y;
            try!(t.pos(left, y));
            try!(t.set_font(&tabfont, 10.0));
            try!(t.set_leading(10.0));
            for line in lines {
                y -= 10.0;
                try!(t.show_line(&line));
            }
            Ok(y)
        }),
        ChordFileExpression::EndOfTab => {
            println!("Warning: Stray end of tab in song!");
            Ok(y)
        }
        ChordFileExpression::Line{s} => c.text(|t| {
            let text_size = 12.0;
            let chord_size = 9.0;
            let y = y - 1.1 * ( if s.len() == 1 { text_size }
                                else { text_size + chord_size });
            try!(t.set_font(&times, text_size));
            try!(t.pos(left, y));
            let mut last_chord_width = 0.0;
            for (i, part) in s.iter().enumerate() {
                if i % 2 == 1 {
                    chords.use_chord(part);
                    try!(t.gsave());
                    try!(t.set_rise(text_size * 0.9));
                    try!(t.set_fill_gray(96));
                    try!(t.set_font(&chordfont, chord_size));
                    let chord_width = chordfont.get_width_raw(&part) as i32;
                    try!(t.show_j(&part, chord_width));
                    last_chord_width =
                        (chord_width + 400) as f32 * chord_size / 1000.0;
                    try!(t.grestore());
                } else {
                    let part = { if part.len() > 0 { part.to_string() }
                                 else { " ".to_string() } };
                    let text_width = times.get_width(text_size, &part);
                    if last_chord_width > text_width && i+1 < s.len() {
                        let extra = last_chord_width - text_width;
                        let n_space = part.chars()
                            .filter(|&c| {c == ' '})
                            .count();
                        if n_space > 0 {
                            try!(t.set_word_spacing(extra / n_space as f32));
                        } else {
                            try!(t.set_char_spacing(extra / part.len() as f32));
                        }
                    }
                    try!(t.show(&part));
                    if last_chord_width > text_width {
                        try!(t.set_char_spacing(0.0));
                        try!(t.set_word_spacing(0.0));
                    }
                }
            }
            Ok(y)
        })
    }
}
