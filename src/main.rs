extern crate pdf;

use pdf::{Canvas, Pdf};
use std::fs::File;
use std::io;
use std::vec::Vec;
//use std::io::Write;

fn chordbox<'a>(c: &mut Canvas<'a, File>, name: &str, strings: Vec<i8>)
                -> io::Result<()> {
    let dx = 5.0;
    let dy = 7.0;
    let left = 15.0;
    let right = left + 5.0 * dx;
    let top = 100.0;
    let bottom = top - 4.4 * dy;
    try!(c.text(|t| {
        try!(t.set_font(12.0));
        try!(t.pos(left, top+dy));
        t.show(name)
    }));
    let barre = strings[0];
    let up =
        if barre < 2 {
            try!(c.move_to(left-0.15, top+0.5));
            try!(c.line_to(right+0.15, top+0.5));
            try!(c.stroke());
            0.0
        } else {
            try!(c.text(|t| {
                try!(t.set_font(dy));
                try!(t.pos(left - dx, top - 0.9 * dy));
                t.show(&format!("{}", barre))
            }));
            1.5
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
    document.render_page(120.0, 150.0, |c| {
        chordbox(c, "Am", vec!(0, -1, 0, 2, 2, 1, 0))
    }).unwrap();
    document.render_page(120.0, 150.0, |c| {
        chordbox(c, "G", vec!(0, 3, 2, 0, 0, 0, 3))
    }).unwrap();
    document.render_page(120.0, 150.0, |c| {
        chordbox(c, "D", vec!(0, -1, -1, 0, 2, 3, 2))
    }).unwrap();
    document.render_page(120.0, 150.0, |c| {
        chordbox(c, "Bm7", vec!(2, -1, 1, 3, 1, 2, 1))
    }).unwrap();
    document.finish().unwrap();
}
