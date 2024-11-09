use crate::PageArgs;

#[derive(Copy, Clone)]
pub struct PageDim {
    width: f32,
    height: f32,
    pageno: u32,
    is_duplex: bool,
    show_pageno: bool,
}

impl From<PageArgs> for PageDim {
    fn from(args: PageArgs) -> Self {
        let a4 = (842.0, 596.0);
        let (width, height) = if args.landscape {
            (a4.0, a4.1)
        } else {
            (a4.1, a4.0)
        };
        PageDim {
            width,
            height,
            pageno: 1,
            is_duplex: !args.no_duplex,
            show_pageno: !args.no_pageno,
        }
    }
}

const INNER: f32 = 70.;
const OUTER: f32 = 20.;

impl PageDim {
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
        self.width - (INNER + OUTER)
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn pageno(&self) -> Option<u32> {
        if self.show_pageno {
            Some(self.pageno)
        } else {
            None
        }
    }
    pub fn left(&self) -> f32 {
        if self.is_verso() {
            OUTER
        } else {
            INNER
        }
    }
    pub fn right(&self) -> f32 {
        if self.is_verso() {
            self.width - INNER
        } else {
            self.width - OUTER
        }
    }
    pub fn top(&self) -> f32 {
        self.height - 20.0
    }
}
