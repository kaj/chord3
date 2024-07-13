mod chords;
mod pagedim;

use crate::chords::{ChordHolder, Instrument};
use crate::pagedim::PageDim;
use clap::Parser;
use pdf_canvas::graphicsstate::Color;
use pdf_canvas::{BuiltinFont, Canvas, Pdf};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::exit;
use std::sync::Mutex;

#[derive(Parser)]
#[command(
    about,
    author,
    version,
    mut_arg("input", |i| i.required_unless_present("chords")),
    after_help =
        "At least one INPUT file is required unless the --chords flag is \
         given.\n\n\
         Each INPUT file contains one or more song in the chopro format, \
         which is described at \
         https://github.com/kaj/chord3/blob/master/chopro.md ."
)]
struct Args {
    /// Title (in metadata) of the output PDF file.
    #[arg(long, default_value = "Songbook")]
    title: String,

    /// Author (in metadata) of the output PDF file.
    #[arg(long)]
    author: Option<String>,

    /// Show chord boxes for this instrument.
    #[arg(long, value_enum, default_value_t = Instrument::Guitar)]
    instrument: Instrument,

    /// Add a separate page of chord definitions.
    #[arg(long)]
    chords: bool,

    /// Output PDF file name.
    #[arg(short, long, default_value = "chords.pdf")]
    output: String,

    /// Show name of chopro source file on page.
    #[arg(long)]
    sourcenames: bool,

    /// Base font size, in points (72 points = 1 inch).
    #[arg(long, default_value = "12")]
    base_size: f32,

    #[clap(flatten)]
    page: PageArgs,

    /// Chopro file(s) to parse.
    input: Vec<String>,
}

#[derive(Parser)]
struct PageArgs {
    /// Use landscape orientation for the output.
    #[arg(long)]
    landscape: bool,

    /// Disable duplex printing.
    ///
    /// When duplex is enabled (the default) the margin is larger on
    /// the spine side of pages, and page numbers are put in the outer
    /// corner.
    #[arg(long, short = 'd')]
    no_duplex: bool,

    /// Disable visible page numbers
    ///
    /// Useful e.g. when writing pages to be included in a larger document.
    #[arg(long)]
    no_pageno: bool,
}

fn chordbox(
    c: &mut Canvas<'_>,
    left: f32,
    top: f32,
    name: &str,
    strings: &[i8],
    base_size: f32,
) -> io::Result<()> {
    let n_strings = (strings.len() - 1) as u8;
    let n_bands: u8 = if n_strings == 4 { 8 } else { 4 };

    let (dx, dy) = if n_strings == 4 {
        (0.458_333_34 * base_size, 0.458_333_34 * base_size)
    } else {
        (0.416_666_66 * base_size, 0.583_333_3 * base_size)
    };
    let right = left + f32::from(n_strings - 1) * dx;
    let bottom = top - (f32::from(n_bands) + 0.4) * dy;
    let radius = 1.4;
    c.center_text(
        (left + right) / 2.0,
        top + 2.0 + 4.0 * radius,
        BuiltinFont::Helvetica_Oblique,
        12.0,
        name,
    )?;
    let barre = strings[0];
    let up = if barre < 2 {
        c.set_line_width(1.0)?;
        c.line(left - 0.15, top + 0.5, right + 0.15, top + 0.5)?;
        c.stroke()?;
        for mark in &[5_u8, 7, 10] {
            if n_bands >= *mark {
                c.right_text(
                    left - 0.4 * dx,
                    top - (f32::from(*mark) - 0.1) * dy,
                    BuiltinFont::Helvetica,
                    dy,
                    &format!("{mark}"),
                )?;
            }
        }
        0.0
    } else {
        c.right_text(
            left - 0.4 * dx,
            top - 0.9 * dy,
            BuiltinFont::Helvetica,
            dy,
            &format!("{barre}"),
        )?;
        1.6
    };
    c.set_line_width(0.3)?;
    for b in 0..=n_bands {
        let y = top - f32::from(b) * dy;
        c.line(left, y, right, y)?;
    }
    for s in 0..n_strings {
        let x = left + f32::from(s) * dx;
        c.line(x, top + up, x, bottom)?;
    }
    c.stroke()?;
    let radius = base_size * 0.11;
    let above = top + 2.0 + radius;
    for (string, band) in strings[1..].iter().enumerate() {
        let x = left + string as f32 * dx;
        match *band {
            -2 => (), // No-op for unknown chord
            -1 => {
                let (xl, xr) = (x - radius, x + radius);
                let (yt, yb) = (above - radius, above + radius);
                c.line(xl, yt, xr, yb)?;
                c.line(xr, yt, xl, yb)?;
                c.stroke()?;
            }
            0 => {
                c.circle(x, above, radius)?;
                c.stroke()?;
            }
            band => {
                let y = top - (f32::from(band) - 0.5) * dy;
                c.circle(x, y, radius * 1.2)?;
                c.fill()?;
            }
        }
    }
    Ok(())
}

enum ChordFileExpression {
    Title { s: String },
    SubTitle { s: String },
    Comment { s: String },
    ChordDef { name: String, def: Vec<i8> },
    Chorus { lines: Vec<ChordFileExpression> },
    EndOfChorus,
    Tab { lines: Vec<String> },
    EndOfTab,
    StartColumns { n_columns: u8 },
    ColumnBreak,
    PageBreak,
    NewSong,
    Line { s: Vec<String> },
}

struct ChoproParser<R: io::Read> {
    source: Mutex<io::Lines<io::BufReader<R>>>,
    eof: bool,
}

impl ChoproParser<File> {
    fn open(path: &str) -> io::Result<ChoproParser<File>> {
        let f = File::open(path)?;
        Ok(ChoproParser::new(f))
    }
}
impl<R: io::Read> ChoproParser<R> {
    fn new(source: R) -> ChoproParser<R> {
        let reader = io::BufReader::new(source);
        ChoproParser {
            source: Mutex::new(reader.lines()),
            eof: false,
        }
    }

    // Internal: Return the next line that is not a comment
    fn nextline(&mut self) -> Option<String> {
        loop {
            match self.source.lock().unwrap().next() {
                Some(Ok(line)) => {
                    let comment_re = Regex::new(r"^\s*#").unwrap();
                    if !comment_re.is_match(&line) {
                        return Some(line);
                    }
                }
                Some(Err(e)) => {
                    println!("Failed to read source: {e}");
                    self.eof = true;
                    return None;
                }
                _ => {
                    self.eof = true;
                    return None;
                }
            }
        }
    }

    fn is_eof(&self) -> bool {
        self.eof
    }
}

impl<R: io::Read> Iterator for ChoproParser<R> {
    type Item = ChordFileExpression;

    fn next(&mut self) -> Option<ChordFileExpression> {
        if let Some(line) = self.nextline() {
            let re =
                Regex::new(r"\{(?P<cmd>\w+)(?::?\s*(?P<arg>.*))?\}").unwrap();
            if let Some(caps) = re.captures(&line) {
                let arg =
                    caps.name("arg").map_or("", |m| m.as_str()).to_string();
                match &*caps.name("cmd").unwrap().as_str().to_lowercase() {
                    "title" | "t" => {
                        Some(ChordFileExpression::Title { s: arg })
                    }
                    "subtitle" | "st" => {
                        Some(ChordFileExpression::SubTitle { s: arg })
                    }
                    "comment" | "c" | "ci" | "cb" => {
                        Some(ChordFileExpression::Comment { s: arg })
                    }
                    "define" => {
                        //println!("Parse chord def '{}'", arg);
                        let re = Regex::new(
                            r"(?i)^([\S]+)\s+base-fret\s+([0-9]+)\s+frets(?:\s+([x0-9-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))(?:\s+([x0-5-]))?(?:\s+([x0-5-]))?\s*$").unwrap();
                        if let Some(caps) = re.captures(&arg) {
                            let mut caps = caps.iter().skip(1);
                            let name: String = caps
                                .next()
                                .unwrap()
                                .unwrap()
                                .as_str()
                                .to_string();
                            let def = caps
                                .flatten()
                                .map(|cap| match cap.as_str() {
                                    "x" | "X" | "-" => -1,
                                    s => s.parse::<i8>().unwrap(),
                                })
                                .collect();
                            Some(ChordFileExpression::ChordDef { name, def })
                        } else {
                            let whole = caps.get(0).unwrap().as_str();
                            println!("Warning: Bad chord definition {whole}");
                            Some(ChordFileExpression::Comment {
                                s: whole.to_string(),
                            })
                        }
                    }
                    "soc" | "start_of_chorus" => {
                        let mut lines = vec![];
                        for line in self {
                            match line {
                                ChordFileExpression::EndOfChorus => {
                                    break;
                                }
                                line => lines.push(line),
                            }
                        }
                        Some(ChordFileExpression::Chorus { lines })
                    }
                    "eoc" | "end_of_chorus" => {
                        Some(ChordFileExpression::EndOfChorus)
                    }
                    "sot" | "start_of_tab" => {
                        let mut lines = vec![];
                        let end =
                            Regex::new(r"\{(eot|end_of_tab):?\s*").unwrap();
                        while let Some(line) = self.nextline() {
                            if end.is_match(&line) {
                                break;
                            }
                            lines.push(line);
                        }
                        Some(ChordFileExpression::Tab { lines })
                    }
                    "eot" | "end_of_tab" => Some(ChordFileExpression::EndOfTab),
                    "columns" | "col" => {
                        Some(ChordFileExpression::StartColumns {
                            n_columns: arg.parse::<u8>().unwrap(),
                        })
                    }
                    "colb" => Some(ChordFileExpression::ColumnBreak),
                    "page_break" | "np" => Some(ChordFileExpression::PageBreak),
                    "new_song" => Some(ChordFileExpression::NewSong),
                    x => {
                        println!("unknown expression {x}");
                        Some(ChordFileExpression::Comment {
                            s: caps.get(0).unwrap().as_str().to_string(),
                        })
                    }
                }
            } else {
                let mut s = vec![];
                let re = Regex::new(r"([^\[]*)(?:\[([^\]]*)\])?").unwrap();
                for caps in re.captures_iter(&line.replace('\t', "    ")) {
                    s.push(caps.get(1).unwrap().as_str().to_string());
                    if let Some(chord) = caps.get(2) {
                        s.push(chord.as_str().to_string());
                    }
                }
                Some(ChordFileExpression::Line { s })
            }
        } else {
            None
        }
    }
}

fn main() {
    let args = Args::parse();
    let filename = &args.output;
    let mut document = Pdf::create(filename)
        .map_err(|err| {
            println!("Failed to open {filename}: {err}");
            exit(1);
        })
        .unwrap();
    document.set_title(&args.title);
    if let Some(author) = args.author.as_ref() {
        document.set_author(author);
    }
    document.set_producer(concat!(
        "chord3 version ",
        env!("CARGO_PKG_VERSION"),
        "\nhttps://github.com/kaj/chord3"
    ));

    let show_sourcenames = args.sourcenames;
    let instrument = args.instrument;
    let base_size = args.base_size;
    let mut page = PageDim::from(args.page);

    for name in &args.input {
        match render_song(
            &mut document,
            name,
            show_sourcenames,
            page,
            instrument,
            base_size,
        ) {
            Ok(p) => page = p.next(),
            Err(e) => println!("Failed to handle {name}: {e}"),
        }
    }
    if args.chords {
        render_chordlist(&mut document, page, instrument, base_size)
            .expect("Render chordlist");
    }
    document.finish().unwrap();
}

fn render_chordlist(
    document: &mut Pdf,
    page: PageDim,
    instrument: Instrument,
    base_size: f32,
) -> io::Result<()> {
    let chords = ChordHolder::new_for(instrument);

    document.render_page(page.width(), page.height(), |c| {
        let s = "Chords";
        c.add_outline(s);
        c.left_text(
            page.left(),
            page.top() - 1.5 * base_size,
            BuiltinFont::Times_Bold,
            base_size * 4. / 3.,
            s,
        )?;
        render_chordboxes(c, page, chords.get_all_chords(), base_size)
    })
}

fn render_song(
    document: &mut Pdf,
    songfilename: &str,
    show_sourcename: bool,
    page: PageDim,
    instrument: Instrument,
    base_size: f32,
) -> io::Result<PageDim> {
    let mut source = ChoproParser::open(songfilename)?;
    let mut chords = ChordHolder::new_for(instrument);
    let mut page = page;
    let mut column_top = page.top();
    let mut left = page.left();
    let mut n_cols = 1;
    while !source.is_eof() {
        document.render_page(page.width(), page.height(), |c| {
            let mut y = page.top();
            if show_sourcename {
                if page.is_verso() {
                    c.right_text(
                        page.right(),
                        20.0,
                        BuiltinFont::Helvetica_Oblique,
                        10.0,
                        songfilename,
                    )?;
                } else {
                    c.left_text(
                        page.left(),
                        20.0,
                        BuiltinFont::Helvetica_Oblique,
                        10.0,
                        songfilename,
                    )?;
                }
            }
            write_pageno(c, &page)?;
            for token in source.by_ref() {
                if let ChordFileExpression::StartColumns { n_columns } = token {
                    column_top = y;
                    n_cols = n_columns;
                } else {
                    y = render_token(
                        token,
                        y,
                        left,
                        c,
                        &mut chords,
                        base_size,
                    )?;
                    if y == std::f32::NEG_INFINITY {
                        render_chordboxes(
                            c,
                            page,
                            chords.get_used(),
                            base_size,
                        )?;
                        n_cols = 1;
                        chords = ChordHolder::new_for(instrument);
                        page = page.next();
                        left = page.left();
                        return Ok(());
                    }
                    if y < (2. + 4. * base_size) {
                        left += page.inner_width() / f32::from(n_cols) + 10.0;
                        if left < page.right() {
                            y = column_top;
                        } else {
                            page = page.next();
                            left = page.left();
                            return Ok(());
                        }
                    }
                }
            }
            render_chordboxes(c, page, chords.get_used(), base_size)?;
            Ok(())
        })?;
    }
    Ok(page)
}

fn write_pageno(c: &mut Canvas, page: &PageDim) -> io::Result<()> {
    if let Some(pageno) = page.pageno() {
        let font = BuiltinFont::Times_Italic;
        let pageno = format!("{pageno}");
        if page.is_verso() {
            c.left_text(page.left(), 20.0, font, 12.0, &pageno)?;
        } else {
            c.right_text(page.right(), 20.0, font, 12.0, &pageno)?;
        }
    }
    Ok(())
}

fn render_chordboxes(
    c: &mut Canvas<'_>,
    page: PageDim,
    used_chords: Vec<(&str, &Vec<i8>)>,
    base_size: f32,
) -> io::Result<()> {
    let (box_width, box_height) = match used_chords.first().map(|v| v.1.len()) {
        Some(7) => (base_size * 3.5, base_size * 31. / 6.),
        Some(4) => (base_size * 3., base_size * 19. / 3.),
        Some(0) | None => return Ok(()),
        x => {
            println!("Warning: Unkown kind of chord, {x:?}");
            return Ok(());
        }
    };
    let n_chords = used_chords.len() as u32;
    if n_chords > 0 {
        let n_aside = (page.inner_width() / box_width) as u32;
        let box_width = (page.inner_width() + 7.0) / n_aside as f32;
        let n_height = (n_chords + n_aside - 1) / n_aside;
        let n_first = n_chords - (n_height - 1) * n_aside;
        let mut x = page.right() - n_first as f32 * box_width;
        let mut y = 10.0 + n_height as f32 * box_height;
        for (chord, chorddef) in used_chords {
            chordbox(c, x + base_size * 1.25, y, chord, chorddef, base_size)?;
            x += box_width;
            if x >= page.right() {
                x = page.right() - n_aside as f32 * box_width;
                y -= box_height;
            }
        }
    }
    Ok(())
}

fn render_token(
    token: ChordFileExpression,
    y: f32,
    left: f32,
    c: &mut Canvas<'_>,
    chords: &mut ChordHolder,
    base_size: f32,
) -> io::Result<f32> {
    let times_italic = c.get_font(BuiltinFont::Times_Italic);
    let times = c.get_font(BuiltinFont::Times_Roman);
    let chordfont = c.get_font(BuiltinFont::Helvetica_Oblique);
    let tabfont = c.get_font(BuiltinFont::Courier);
    match token {
        ChordFileExpression::Title { s } => {
            c.add_outline(&s);
            let y = y - 1.5 * base_size;
            c.left_text(
                left,
                y,
                BuiltinFont::Times_Bold,
                base_size * 4. / 3.,
                &s,
            )?;
            Ok(y)
        }
        ChordFileExpression::SubTitle { s } => c.text(|t| {
            let y = y - 16.0;
            t.set_font(&times_italic, 14.0)?;
            t.pos(left, y)?;
            t.show(&s)?;
            Ok(y)
        }),
        ChordFileExpression::Comment { s } => c.text(|t| {
            let y = y - base_size;
            t.set_font(&times_italic, base_size)?;
            t.pos(left, y)?;
            t.show(&s)?;
            Ok(y)
        }),
        ChordFileExpression::ChordDef { name, def } => {
            chords.define(name, def);
            Ok(y)
        }
        ChordFileExpression::Chorus { lines } => {
            let mut y2 = y;
            for line in lines {
                y2 = render_token(line, y2, left + 10.0, c, chords, base_size)?;
            }
            y2 -= 4.0;
            c.set_line_width(0.5)?;
            c.line(left - 6.0, y, left - 6.0, y2)?;
            c.stroke()?;
            Ok(y2)
        }
        ChordFileExpression::EndOfChorus => {
            println!("Warning: Stray end of chorus in song!");
            Ok(y)
        }
        ChordFileExpression::Tab { lines } => c.text(|t| {
            let size = base_size / 1.2;
            let mut y = y;
            t.pos(left, y)?;
            t.set_font(&tabfont, size)?;
            t.set_leading(size)?;
            for line in lines {
                y -= size;
                t.show_line(&line)?;
            }
            Ok(y)
        }),
        ChordFileExpression::EndOfTab => {
            println!("Warning: Stray end of tab in song!");
            Ok(y)
        }
        ChordFileExpression::StartColumns { n_columns } => {
            println!(
                "Warning: StartColumns({n_columns}) should be handled earlier"
            );
            Ok(y)
        }
        ChordFileExpression::ColumnBreak | ChordFileExpression::PageBreak => {
            Ok(0.0)
        }
        ChordFileExpression::NewSong => Ok(std::f32::NEG_INFINITY),
        ChordFileExpression::Line { s } => c.text(|t| {
            let text_size = base_size;
            let chord_size = 0.75 * text_size;
            let y = y - 1.1
                * (if s.len() == 1 {
                    text_size
                } else {
                    text_size + chord_size
                });
            t.set_font(&times, text_size)?;
            t.pos(left, y)?;
            let mut last_chord_width = 0.0;
            for (i, part) in s.iter().enumerate() {
                if i % 2 == 1 {
                    chords.use_chord(part);
                    t.gsave()?;
                    t.set_rise(text_size * 0.9)?;
                    t.set_fill_color(Color::gray(96))?;
                    t.set_font(&chordfont, chord_size)?;
                    let chord_width = chordfont.get_width_raw(part) as i32;
                    t.show_adjusted(&[(part, chord_width)])?;
                    last_chord_width =
                        (chord_width + 400) as f32 * chord_size / 1000.0;
                    t.grestore()?;
                } else {
                    let part = {
                        if part.is_empty() {
                            " "
                        } else {
                            part
                        }
                    };
                    let text_width = times.get_width(text_size, part);
                    if last_chord_width > text_width && i + 1 < s.len() {
                        let extra = last_chord_width - text_width;
                        let n_space =
                            part.chars().filter(|&c| c == ' ').count();
                        if n_space > 0 {
                            t.set_word_spacing(extra / n_space as f32)?;
                        } else {
                            t.set_char_spacing(extra / part.len() as f32)?;
                        }
                    }
                    t.show(part)?;
                    if last_chord_width > text_width {
                        t.set_char_spacing(0.0)?;
                        t.set_word_spacing(0.0)?;
                    }
                }
            }
            Ok(y)
        }),
    }
}
