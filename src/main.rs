extern crate pdf;

use pdf::{Canvas, Pdf, FontSource};
use std::fs::File;
use std::io;
use std::vec::Vec;
//use std::io::Write;

fn chordbox<'a>(c: &mut Canvas<'a, File>, left: f32, top: f32,
                name: &str, strings: Vec<i8>)
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
            try!(c.move_to(left-0.15, top+0.5));
            try!(c.line_to(right+0.15, top+0.5));
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
        try!(c.move_to(left, y));
        try!(c.line_to(right, y));
    }
    for s in 0..6 {
        let x = left + s as f32 * dx;
        try!(c.move_to(x, top+up));
        try!(c.line_to(x, bottom));
    }
    try!(c.stroke());
    let radius = 1.4;
    let above = top + 2.0 + radius;
    for s in 0..6 {
        let x = left + s as f32 * dx;
        match strings[s+1] {
            -1 => {
                let (l, r) = (x-radius, x+radius);
                let (t, b) = (above-radius, above+radius);
                try!(c.move_to(l, t));
                try!(c.line_to(r, b));
                try!(c.move_to(r, t));
                try!(c.line_to(l, b));
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

fn main() {
    let mut file = File::create("foo.pdf").unwrap();
    let mut document = Pdf::new(&mut file).unwrap();
    document.render_page(200.0, 150.0, |c| {
        try!(chordbox(c,  20.0, 100.0, "Am", vec!(0, -1, 0, 2, 2, 1, 0)));
        try!(chordbox(c,  60.0, 100.0, "G", vec!(0, 3, 2, 0, 0, 0, 3)));
        try!(chordbox(c, 100.0, 100.0, "D", vec!(0, -1, -1, 0, 2, 3, 2)));
        try!(chordbox(c, 140.0, 100.0, "Bm7", vec!(2, -1, 1, 3, 1, 2, 1)));
        Ok(())
    }).unwrap();
    document.finish().unwrap();
}
