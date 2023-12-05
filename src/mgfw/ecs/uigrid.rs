
pub const SPLIT_H: u8 = 0;
pub const SPLIT_V: u8 = 1;

#[derive(Clone)]
pub struct UIgrid {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub pad: i8,
}

pub fn new(x0: f32, y0: f32, w0: f32, h0: f32) -> UIgrid {
    UIgrid {
        x0,
        y0,
        x1: x0 + w0,
        y1: y0 + h0,
        pad: 4,
    }
}

impl UIgrid {
    pub fn clone(self: &Self) -> UIgrid {
        UIgrid {
            x0: self.x0,
            y0: self.y0,
            x1: self.x1,
            y1: self.y1,
            pad: self.pad,
        }
    }

    pub fn split_vec(self: &Self, dir: u8, v: Vec<f32>) -> Vec<UIgrid> {
        let mut ret: Vec<UIgrid> = Vec::new();

        if SPLIT_V == dir {
            let dy = self.y1 - self.y0;
            let mut ug: UIgrid = self.clone();
            let y0 = ug.y0;
            for i in 0..v.len() {
                ug.y1 = y0 + v[i] * dy;
                ret.push(ug.clone());
                ug.y0 = ug.y1;
            }
            ug.y1 = self.y1;
            ret.push(ug);
        }
        else if SPLIT_H == dir {
            let dx = self.x1 - self.x0;
            let mut ug: UIgrid = self.clone();
            let x0 = ug.x0;
            for i in 0..v.len() {
                ug.x1 = x0 + v[i] * dx;
                ret.push(ug.clone());
                ug.x0 = ug.x1;
            }
            ug.x1 = self.x1;
            ret.push(ug);
        }

        ret
    }

    pub fn split_even(self: &Self, dir: u8, qty: i32) -> Vec<UIgrid> {
        let mut v: Vec<f32> = Vec::new();
        let delta = 1.0 / qty as f32;
        for i in 1..qty {
            v.push(delta * i as f32);
        }
        self.split_vec(dir, v)
    }

    pub fn split(self: &Self, dir: u8, loc: f32) -> Vec<UIgrid> {
        let mut ret: Vec<UIgrid> = Vec::new();

        if SPLIT_V == dir {
            let dy = self.y1 - self.y0;
            let mut ug: UIgrid = self.clone();
            ug.y1 = ug.y0 + loc * dy;
            ret.push(ug.clone());
            ug.y0 = ug.y1;
            ug.y1 = self.y1;
            ret.push(ug);
        }
        else if SPLIT_H == dir {
            let dx = self.x1 - self.x0;
            let mut ug: UIgrid = self.clone();
            ug.x1 = ug.x0 + loc * dx;
            ret.push(ug.clone());
            ug.x0 = ug.x1;
            ug.x1 = self.x1;
            ret.push(ug);
        }

        ret
    }

    pub fn pad(self: &Self) -> UIgrid {
        UIgrid {
            x0: self.x0 + self.pad as f32,
            y0: self.y0 + self.pad as f32,
            x1: self.x1 - self.pad as f32,
            y1: self.y1 - self.pad as f32,
            pad: self.pad,
        }
    }

    pub fn scroll_pad(self: &Self) -> UIgrid {
        UIgrid {
            x0: self.x0 + self.pad as f32,
            y0: self.y0 + self.pad as f32,
            x1: self.x1 - self.pad as f32 - 5.0,
            y1: self.y1 - self.pad as f32 - 5.0,
            pad: self.pad,
        }
    }

    pub fn tab_pad(self: &Self) -> UIgrid {
        UIgrid {
            x0: self.x0 + self.pad as f32,
            y0: self.y0 + self.pad as f32 + 16.0,
            x1: self.x1 - self.pad as f32,
            y1: self.y1 - self.pad as f32,
            pad: self.pad,
        }
    }

    pub fn is_inside(self: &Self, x: i32, y: i32) -> bool {
        (x > self.x0.floor() as i32) &&
        (x < self.x1.floor() as i32) &&
        (y > self.y0.floor() as i32) &&
        (y < self.y1.floor() as i32)
    }
    
}
