#[derive(Copy, Clone)]
pub struct PageDim {
    width: f32,
    height: f32,
    pageno: u32,
}

impl PageDim {
    pub fn a4(pageno: u32) -> PageDim {
        PageDim {
            width: 596.0,
            height: 842.0,
            pageno: pageno,
        }
    }
    pub fn next(&self) -> PageDim {
        PageDim {
            width: self.width,
            height: self.height,
            pageno: self.pageno + 1,
        }
    }
    pub fn is_left(&self) -> bool {
        self.pageno % 2 == 0
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
        if self.is_left() {
            20.0
        } else {
            80.0
        }
    }
    pub fn right(&self) -> f32 {
        if self.is_left() {
            self.width - 75.0
        } else {
            self.width - 15.0
        }
    }
    pub fn top(&self) -> f32 {
        self.height - 20.0
    }
}
