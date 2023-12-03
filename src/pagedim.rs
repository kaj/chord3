#[derive(Copy, Clone)]
pub struct PageDim {
    width: f32,
    height: f32,
    pageno: u32,
    is_duplex: bool,
}

impl PageDim {
    pub fn a4(landscape: bool, pageno: u32, is_duplex: bool) -> PageDim {
        let (width, height) = if landscape {
            (842.0, 596.0)
        } else {
            (596.0, 842.0)
        };
        PageDim {
            width,
            height,
            pageno,
            is_duplex,
        }
    }
    pub fn next(&self) -> PageDim {
        PageDim {
            pageno: self.pageno + 1,
            ..*self
        }
    }

    /// A page is verso (left or backside) if duplex is enabled and
    /// the page number is even (otherwise it is recto).
    pub fn is_verso(&self) -> bool {
        self.is_duplex && (self.pageno % 2 == 0)
    }

    pub fn inner_width(&self) -> f32 {
        self.width - 95.0
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn pageno(&self) -> u32 {
        self.pageno
    }
    pub fn left(&self) -> f32 {
        if self.is_verso() {
            20.0
        } else {
            80.0
        }
    }
    pub fn right(&self) -> f32 {
        if self.is_verso() {
            self.width - 75.0
        } else {
            self.width - 15.0
        }
    }
    pub fn top(&self) -> f32 {
        self.height - 20.0
    }
}
