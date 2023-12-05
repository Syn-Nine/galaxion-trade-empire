#![allow(dead_code)]

use super::*;
use super::super::*;
use std::collections::HashMap;

pub const RECT_AUTO: i32 = 0xFFFFFF;
pub const RECT_CENTER: i32 = 0xFFFFFE;
pub const RECT_RIGHT: i32 = 0xFFFFFD;


pub const WIDGET_INVALID: i32 = 0;
pub const WIDGET_PANEL: i32 = 1;
pub const WIDGET_PANEL_B: i32 = 2;
pub const WIDGET_LABEL: i32 = 3;
pub const WIDGET_BUTTON: i32 = 4;
pub const WIDGET_INPUT: i32 = 5;
pub const WIDGET_IMAGE: i32 = 6;
pub const WIDGET_SEPARATOR: i32 = 7;

const RENDERER_INVALID: u8 = 0;
const RENDERER_BILLBOARD: u8 = 1;
const RENDERER_TEXT: u8 = 2;

const TEX_HANDLE_INVALID: u32 = 0xFFFFFFFF;

#[derive(Default, Clone)]
struct Object {
    tex: u32,
    vao: u32,
    vbo: u32,
    renderer: u8,
    num_chars: usize,
    color: Color,
}


#[derive(Default, Clone)]
pub struct Rect {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

#[derive(Default, Clone)]
pub struct Rectf {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

#[derive(Default)]
struct ClickRegion {
    rect: Rect,
    clicked: bool,
    pressing: bool,
    selected: bool,
    age: i32,
}

#[derive(Default)]
struct InputRegion {
    rect: Rect,
    modified: bool,
    value: String,
    age: i32,
    input_numeric: bool,
    input_alpha: bool,
}

#[derive(Clone)]
pub struct Widget {
    pub name: String,
    pub handle: String,
    pub value: String,
    pub rect: Rect,
    pub class: i32,
    pub same_line: bool,
    pub input_label_width: i32,
    pub show_label: bool,
    pub expand: bool,
    pub position_absolute: bool,
    pub input_numeric: bool,
    pub input_alpha: bool,
    pub show_image: bool,
    pub image_tex: u32,
    pub visible: bool,
    pub image_uv: Rectf,
    pub color: Color,
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            name: String::from(""),
            handle: String::from(""),
            value: String::from(""),
            class: WIDGET_INVALID,
            rect: Rect { x0: 0, y0: 0, x1: RECT_AUTO, y1: RECT_AUTO },
            same_line: false,
            input_label_width: RECT_AUTO,
            show_label: true,
            expand: true,
            position_absolute: false,
            input_numeric: true,
            input_alpha: true,
            show_image: false,
            image_tex: 0,
            visible: true,
            image_uv: Rectf { x0: 0.0, y0: 0.0, x1: 1.0, y1: 1.0 },
            color: Color { r: 161.0 / 255.0, g: 164.0 / 255.0, b: 239.0 / 255.0, a: 1.0 },
        }
    }
}

impl Widget {
    pub fn center(self: Self) -> Widget {
        let mut ret = self;
        ret.rect.x0 = RECT_CENTER;
        ret
    }

    pub fn color(self: Self, c: &Color) -> Widget {
        let mut ret = self;
        ret.color = c.clone();
        ret
    }

    pub fn expand(self: Self) -> Widget {
        self.expand_explicit(true)
    }

    pub fn expand_explicit(self: Self, expanded: bool) -> Widget {
        let mut ret = self;
        ret.expand = expanded;
        ret
    }

    pub fn hide(self: Self) -> Widget {
        self.visible_explicit(false)
    }

    pub fn rect(self: Self, x0: i32, y0: i32, x1: i32, y1: i32) -> Widget {
        let mut ret = self;
        ret.rect = Rect { x0, y0, x1, y1 };
        ret
    }

    pub fn right(self: Self) -> Widget {
        let mut ret = self;
        ret.rect.x0 = RECT_RIGHT;
        ret
    }

    pub fn same_line(self: Self) -> Widget {
        let mut ret = self;
        ret.same_line = true;
        ret
    }

    pub fn same_line_explicit(self: Self, sameline: bool) -> Widget {
        let mut ret = self;
        ret.same_line = sameline;
        ret
    }

    pub fn visible_explicit(self: Self, visibility: bool) -> Widget {
        let mut ret = self;
        ret.visible = visibility;
        ret
    }

    pub fn x0(self: Self, x0: i32) -> Widget {
        let mut ret = self;
        ret.rect.x0 = x0;
        ret
    }

    pub fn y0(self: Self, y0: i32) -> Widget {
        let mut ret = self;
        ret.rect.y0 = y0;
        ret
    }
    
    pub fn x1(self: Self, x1: i32) -> Widget {
        let mut ret = self;
        ret.rect.x1 = x1;
        ret
    }

    pub fn y1(self: Self, y1: i32) -> Widget {
        let mut ret = self;
        ret.rect.y1 = y1;
        ret
    }
    
    pub fn position_absolute(self: Self) -> Widget {
    	let mut ret = self;
	ret.position_absolute = true;
	ret
    }

}

pub fn image(name: String, image_tex: u32) -> Widget {
    Widget {
        class: WIDGET_IMAGE,
        name,
        image_tex,
        .. Default::default()
    }
}


pub fn button(name: String) -> Widget {
    Widget {
        class: WIDGET_BUTTON,
        name,
        .. Default::default()
    }
}


pub fn panel(name: String, x0: i32, y0: i32, x1: i32, y1: i32) -> Widget {
    Widget {
        class: WIDGET_PANEL,
        name,
        rect: Rect { x0, y0, x1, y1 },
        .. Default::default()
    }
}

pub fn panel_b(name: String, x0: i32, y0: i32, x1: i32, y1: i32) -> Widget {
    Widget {
        class: WIDGET_PANEL_B,
        name,
        rect: Rect { x0, y0, x1, y1 },
        .. Default::default()
    }
}

pub fn separator() -> Widget {
    Widget {
        class: WIDGET_SEPARATOR,
        rect: Rect { x0: 0, y0: 0, x1: 0, y1: 0 },
        .. Default::default()
    }
}

pub fn text(name: String) -> Widget {
    Widget {
        class: WIDGET_LABEL,
        name,
        .. Default::default()
    }
}

fn build_billboard(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<f32> {

    let mut vertex_data: Vec<f32> = Vec::new();

    let u0: f32 = 0.0;
    let v0: f32 = 0.0;
    
    let idx = x1 - x0;
    let idy = y1 - y0;
    let idxm = idx as f32 / 8.0;
    let idym = idy as f32 / 8.0;

    let u1 = 1.0 * idxm as f32;
    let v1 = 1.0 * idym as f32;

    let x0 = x0 as f32;
    let y0 = y0 as f32;
    let x1 = x1 as f32;
    let y1 = y1 as f32;

    vertex_data.extend_from_slice(&[x0, y0]); // pos
    vertex_data.extend_from_slice(&[u0, v0]); // uv

    vertex_data.extend_from_slice(&[x0, y1]); // pos
    vertex_data.extend_from_slice(&[u0, v1]); // uv

    vertex_data.extend_from_slice(&[x1, y1]); // pos
    vertex_data.extend_from_slice(&[u1, v1]); // uv

    vertex_data.extend_from_slice(&[x0, y0]); // pos
    vertex_data.extend_from_slice(&[u0, v0]); // uv

    vertex_data.extend_from_slice(&[x1, y1]); // pos
    vertex_data.extend_from_slice(&[u1, v1]); // uv

    vertex_data.extend_from_slice(&[x1, y0]); // pos
    vertex_data.extend_from_slice(&[u1, v0]); // uv

    vertex_data
}

fn build_billboard_full(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<f32> {
    build_billboard_sub_image(x0, y0, x1, y1, &Rectf { x0: 0.0, y0: 0.0, x1: 1.0, y1: 1.0} )
}

fn build_billboard_sub_image(x0: i32, y0: i32, x1: i32, y1: i32, uv: &Rectf) -> Vec<f32> {

    let mut vertex_data: Vec<f32> = Vec::new();

    let u0: f32 = uv.x0;
    let v0: f32 = uv.y0;
    
    let u1: f32 = uv.x1;
    let v1: f32 = uv.y1;

    let x0 = x0 as f32;
    let y0 = y0 as f32;
    let x1 = x1 as f32;
    let y1 = y1 as f32;

    vertex_data.extend_from_slice(&[x0, y0]); // pos
    vertex_data.extend_from_slice(&[u0, v0]); // uv

    vertex_data.extend_from_slice(&[x0, y1]); // pos
    vertex_data.extend_from_slice(&[u0, v1]); // uv

    vertex_data.extend_from_slice(&[x1, y1]); // pos
    vertex_data.extend_from_slice(&[u1, v1]); // uv

    vertex_data.extend_from_slice(&[x0, y0]); // pos
    vertex_data.extend_from_slice(&[u0, v0]); // uv

    vertex_data.extend_from_slice(&[x1, y1]); // pos
    vertex_data.extend_from_slice(&[u1, v1]); // uv

    vertex_data.extend_from_slice(&[x1, y0]); // pos
    vertex_data.extend_from_slice(&[u1, v0]); // uv

    vertex_data
}

pub struct Mgui {
    data: Vec<Widget>,
    frame_ready: bool,
    tick_time: u128,
    constructed: bool,
    initialized: bool,
    tex: Vec<u32>,
    pool: Vec<Object>,
    pool_size: usize,
    pool_idx: usize,
    font: std::boxed::Box<fonts::retro_gaming::Font>,
    texture_handles: std::boxed::Box<HashMap<String, u32>>,
    //vao: u32,
    //vbo: u32,
    //num_chars: usize,
    click_regions: std::boxed::Box<HashMap<String, ClickRegion>>,
    input_regions: std::boxed::Box<HashMap<String, InputRegion>>,
    panel_handle: String,
    delay_load_texture: Vec<String>,
}

impl Mgui {
    pub fn new() -> Self {
        Mgui {
            data: Vec::new(),
            frame_ready: false,
            tick_time: 0,
            constructed: false,
            initialized: false,
            pool: Vec::new(),
            pool_size: 0,
            pool_idx: 0,
            tex: Vec::new(),
            font: Box::new(fonts::retro_gaming::Font::new()),
            texture_handles: Box::new(HashMap::new()),
            delay_load_texture: Vec::new(),
            //vao: 0,
            //vbo: 0,
            click_regions: Box::new(HashMap::new()),
            input_regions: Box::new(HashMap::new()),
            panel_handle: String::from(""),
        }
    }

    pub fn add(self: &mut Self, w: Widget) -> String {

        let handle;

        if WIDGET_PANEL == w.class || WIDGET_PANEL_B == w.class {
            self.panel_handle = w.name.clone();
            handle = self.panel_handle.clone();
        } else {
            handle = format!("{}_{}", self.panel_handle, w.name);
        }

        let mut nw = w.clone();
        nw.handle = handle.clone();

        self.data.push(nw);
        handle
    }


    pub fn clicked(self: &mut Self, handle: & String) -> bool {

        let mut c = false;
        if self.click_regions.contains_key(handle) {
            if self.click_regions.get_mut(handle).unwrap().clicked {
                self.click_regions.get_mut(handle).unwrap().clicked = false;
                //println!("{} age {}", handle, self.click_regions.get_mut(handle).unwrap().age);
                if 0 < self.click_regions.get_mut(handle).unwrap().age {
                    c = true;
                }
            }
        }
        
        //if c { println!("{} clicked!", handle); }
        c
    }

    fn construct(self: &mut Self, gl: &Gl) {

        self.pool_reset();

        if !self.delay_load_texture.is_empty() {
            for i in 0..self.delay_load_texture.len() {
                self.load_texture(gl, self.delay_load_texture[i].clone());
            }
            self.delay_load_texture.clear();
        }

        let mut rect: Rect = Default::default();

        // cursor
        let mut cx: i32 = 0; 
        let mut cy: i32 = 0;
        let mut base_cx: i32 = cx;
        let mut previous_cx: i32 = cx;
        let mut previous_cy: i32 = cy;

        for i in 0..self.data.len() {

            if WIDGET_PANEL == self.data[i].class ||
                WIDGET_PANEL_B == self.data[i].class {

                let x0: i32 = self.data[i].rect.x0;
                let y0: i32 = self.data[i].rect.y0;
                let mut w0: i32 = 128;
                let mut h0: i32 = 128;

                cx = x0 + 8;
                cy = y0 + 8;

                if RECT_AUTO != self.data[i].rect.x1 {
                    w0 = self.data[i].rect.x1 - self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y1 {
                    h0 = self.data[i].rect.y1 - self.data[i].rect.y0;
                }

                rect.x0 = x0;
                rect.y0 = y0;
                rect.x1 = rect.x0 + w0;
                rect.y1 = rect.y0 + h0;

                let x1 = x0 + 8;
                let x3 = x0 + w0;
                let x2 = x3 - 8;

                let y1 = y0 + 8;
                let y3 = y0 + h0;
                let y2 = y3 - 8;

                let mut texmod = 0;
                if WIDGET_PANEL_B == self.data[i].class { texmod = 36; }

                if self.data[i].visible {
                
                    let vertex_data = build_billboard(x0, y0, x1, y1);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[0 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x1, y0, x2, y1);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[1 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x2, y0, x3, y1);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[2 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    //

                    let vertex_data = build_billboard(x0, y1, x1, y2);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[3 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x1, y1, x2, y2);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[4 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x2, y1, x3, y2);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[5 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    //

                    let vertex_data = build_billboard(x0, y2, x1, y3);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[6 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x1, y2, x2, y3);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[7 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                    let vertex_data = build_billboard(x2, y2, x3, y3);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.tex[8 + texmod];
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                }

                previous_cx = cx;
                previous_cy = cy;
                base_cx = cx;

            }
            else if WIDGET_BUTTON == self.data[i].class {

                // same line offset
                cx = base_cx;
                if self.data[i].same_line {
                    cx = previous_cx;
                    cy = previous_cy;
                }

                let mut txt_width = 0;
                let txt = self.data[i].name.clone();

                if self.data[i].show_label {

                    // get text width
                    let bytes = txt.as_bytes();
                    let mut basex: f32 = 0.0;

                    for i in 0..bytes.len() {
                        let idx = bytes[i] as u16;
                        let data = self.font.data[&idx];
                        let advance = data[6] as f32;
                        basex += advance;
                    }

                    txt_width = basex as i32 + 1;
                }

                let mut x0 = cx;
                let mut y0 = cy;
                let mut w0: i32 = (rect.x1 - x0) - 8;
                let mut h0: i32 = 20;

                if RECT_AUTO != self.data[i].rect.x0 {
                    x0 += self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y0 {
                    y0 += self.data[i].rect.y0;
                }

                if RECT_AUTO == self.data[i].rect.x1 && !self.data[i].expand {
                    w0 = txt_width + 16;
                }
                else if RECT_AUTO != self.data[i].rect.x1 {
                    w0 = self.data[i].rect.x1 - self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y1 {
                    h0 = self.data[i].rect.y1 - self.data[i].rect.y0;
                }

                if self.data[i].position_absolute {
                    x0 = self.data[i].rect.x0;
                    y0 = self.data[i].rect.y0;
                }

                let x1 = x0 + 8;
                let x3 = x0 + w0;
                let x2 = x3 - 8;

                let mut y1 = y0 + 8;
                let mut y3 = y0 + h0;
                let mut y2 = y3 - 8;

                let mut pressing = false;

                // update click region
                let handle = self.data[i].handle.clone();
                match self.click_regions.contains_key(&handle) {
                    true => {
                        self.click_regions.get_mut(&handle).unwrap().clicked = false;
                        self.click_regions.get_mut(&handle).unwrap().age = 130;
                        self.click_regions.get_mut(&handle).unwrap().rect = Rect { x0: x0+2, y0: y0+2, x1: x3-2, y1: y3-2 };
                        pressing = self.click_regions.get_mut(&handle).unwrap().pressing;
                    },
                    false => {
                        self.click_regions.insert(handle, ClickRegion { rect: Rect { x0: x0+2, y0: y0+2, x1: x3-2, y1: y3-2 }, selected: false, clicked: false, pressing: false, age: 2 });
                    }
                };

                if pressing { y0 += 1; y1 += 1; y2 += 1; y3 += 1; }
                
                let vertex_data = build_billboard(x0, y0, x1, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[9];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y0, x2, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[10];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y0, x3, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[11];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                //

                let vertex_data = build_billboard(x0, y1, x1, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[12];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y1, x2, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[13];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y1, x3, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[14];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                //

                let vertex_data = build_billboard(x0, y2, x1, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[15];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y2, x2, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[16];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y2, x3, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[17];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                // button image
                if self.data[i].show_image && TEX_HANDLE_INVALID != self.data[i].image_tex {
                    //let vertex_data = build_billboard_full(x0+4, y0+4, x3-4, y3-4);
                    let vertex_data = build_billboard_sub_image(x0+4, y0+4, x3-4, y3-4, &self.data[i].image_uv);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.data[i].image_tex;
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);
                }
                

                // button text

                if self.data[i].show_label {

                    let bcx = x0 + w0 / 2 - txt_width / 2;
                    let bcy = y0 + h0 / 2 - 8;                    

                    let ww = 256.0;
                    let hh = 64.0;
                    let mut vertex_data: Vec<f32> = Vec::new();

                    let bytes = txt.as_bytes();
                    let num_chars = bytes.len();

                    let mut basex: f32 = 0.0;

                    for i in 0..num_chars {
                        let idx = bytes[i] as u16;

                        let data = self.font.data[&idx];
                        let dx = data[0] as f32 / ww;
                        let dy = data[1] as f32 / hh;
                        let dw = data[2] as f32;
                        let dh = data[3] as f32;
                        let dwt = dw as f32 / ww;
                        let dht = dh as f32 / hh;
                        let dxoff = data[4] as f32;
                        let dyoff = data[5] as f32;
                        let advance = data[6] as f32;

                        let p0 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + 0.0 + dyoff, dx, dy];
                        let p1 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + dh + dyoff, dx, dy + dht];
                        let p2 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + dh + dyoff, dx + dwt, dy + dht];
                        let p3 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + 0.0 + dyoff, dx + dwt, dy];

                        for i in 0..4 {
                            vertex_data.push(p0[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p1[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p2[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p0[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p2[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p3[i]);
                        }

                        basex += advance;
                    }

                    

                    let idx = self.pool_get(gl);
                    self.pool[idx].renderer = RENDERER_TEXT;                
                    self.pool[idx].num_chars = num_chars;

                    let data_ptr = vertex_data.as_ptr() as *const _;
                    gl.buffer_font_data(self.pool[idx].vao, self.pool[idx].vbo, num_chars as i32, data_ptr);

                }

                previous_cx = x3 + 4;
                previous_cy = cy;
                cy += h0;

            }
            else if WIDGET_IMAGE == self.data[i].class {

                // same line offset
                cx = base_cx;
                if self.data[i].same_line {
                    cx = previous_cx;
                    cy = previous_cy;
                }

                let x0 = cx + self.data[i].rect.x0;
                let y0 = cy + self.data[i].rect.y0;

                let w0 = self.data[i].rect.x1 - self.data[i].rect.x0;
                let h0 = self.data[i].rect.y1 - self.data[i].rect.y0;

                let x3 = x0 + w0;
                let y3 = y0 + h0;

                //println!("{}, {cx}, {cy}, {x0}, {y0}, {x3}, {y3}", self.data[i].image_tex);

                // update click region
                let handle = self.data[i].handle.clone();
                match self.click_regions.contains_key(&handle) {
                    true => {
                        self.click_regions.get_mut(&handle).unwrap().clicked = false;
                        self.click_regions.get_mut(&handle).unwrap().age = 130;
                        self.click_regions.get_mut(&handle).unwrap().rect = Rect { x0: x0, y0: y0, x1: x3, y1: y3 };
                    },
                    false => {
                        self.click_regions.insert(handle, ClickRegion { rect: Rect { x0: x0, y0: y0, x1: x3, y1: y3 }, selected: false, clicked: false, pressing: false, age: 2 });
                    }
                };
                
                // image
                if TEX_HANDLE_INVALID != self.data[i].image_tex {
                    
                    let vertex_data = build_billboard_sub_image(x0, y0, x3, y3, &self.data[i].image_uv);
                    let idx = self.pool_get(gl);
                    self.pool[idx].tex = self.data[i].image_tex;
                    self.pool[idx].renderer = RENDERER_BILLBOARD;
                    gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);
                }

                previous_cx = x3 + 4;
                previous_cy = cy;
                cy += h0;

            }
            else if WIDGET_INPUT == self.data[i].class {

                // same line offset
                cx = base_cx;
                if self.data[i].same_line {
                    cx = previous_cx;
                    cy = previous_cy;
                }

                let mut bcx = cx;
                let mut bcy = cy + 2;

                if RECT_AUTO != self.data[i].rect.x0 {
                    bcx += self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y0 {
                    bcy += self.data[i].rect.y0;
                }

                let mut txt_width: i32 = 0;

                if self.data[i].show_label {
                    // label text
                    let txt = self.data[i].name.clone();               

                    let ww = 256.0;
                    let hh = 64.0;
                    let mut vertex_data: Vec<f32> = Vec::new();

                    let bytes = txt.as_bytes();
                    let num_chars = bytes.len();

                    let mut basex: f32 = 0.0;

                    for i in 0..num_chars {
                        let idx = bytes[i] as u16;

                        let data = self.font.data[&idx];
                        let dx = data[0] as f32 / ww;
                        let dy = data[1] as f32 / hh;
                        let dw = data[2] as f32;
                        let dh = data[3] as f32;
                        let dwt = dw as f32 / ww;
                        let dht = dh as f32 / hh;
                        let dxoff = data[4] as f32;
                        let dyoff = data[5] as f32;
                        let advance = data[6] as f32;

                        let p0 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + 0.0 + dyoff, dx, dy];
                        let p1 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + dh + dyoff, dx, dy + dht];
                        let p2 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + dh + dyoff, dx + dwt, dy + dht];
                        let p3 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + 0.0 + dyoff, dx + dwt, dy];

                        for i in 0..4 {
                            vertex_data.push(p0[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p1[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p2[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p0[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p2[i]);
                        }
                        for i in 0..4 {
                            vertex_data.push(p3[i]);
                        }

                        basex += advance;
                    }

                    let idx = self.pool_get(gl);
                    self.pool[idx].renderer = RENDERER_TEXT;                
                    self.pool[idx].num_chars = num_chars;

                    let data_ptr = vertex_data.as_ptr() as *const _;
                    gl.buffer_font_data(self.pool[idx].vao, self.pool[idx].vbo, num_chars as i32, data_ptr);

                    txt_width = basex as i32;

                }


                //////////////////////////.
                /// 
                let mut x0 = bcx + txt_width + 4;
                if RECT_AUTO != self.data[i].input_label_width {
                    x0 = cx + self.data[i].input_label_width + 4;
                }

                let mut y0 = bcy;
                let mut w0: i32 = (rect.x1 - x0) - 8;
                let mut h0: i32 = 20;

                if RECT_AUTO != self.data[i].rect.x1 {
                    w0 = self.data[i].rect.x1 - self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y1 {
                    h0 = self.data[i].rect.y1 - self.data[i].rect.y0;
                }

                let x1 = x0 + 8;
                let x3 = x0 + w0;
                let x2 = x3 - 8;

                let mut y1 = y0 + 8;
                let mut y3 = y0 + h0;
                let mut y2 = y3 - 8;

                let mut pressing = false;
                let mut selected = false;

                // update input region
                let handle = self.data[i].handle.clone();
                let value = self.data[i].value.clone();

                match self.input_regions.contains_key(&handle) {
                    true => {
                        if 0 == self.input_regions.get_mut(&handle).unwrap().age {
                            self.input_regions.get_mut(&handle).unwrap().value = value.clone();
                        }
                        self.input_regions.get_mut(&handle).unwrap().modified = false;
                        self.input_regions.get_mut(&handle).unwrap().age = 130;
                        self.input_regions.get_mut(&handle).unwrap().rect = Rect { x0: x0, y0: y0, x1: x3, y1: y3 };
                    },
                    false => {
                        self.input_regions.insert(handle, InputRegion { input_numeric: self.data[i].input_numeric, input_alpha: self.data[i].input_alpha, rect: Rect { x0: x0, y0: y0, x1: x3, y1: y3 }, modified: false, value: value.clone(), age: 130 });
                    }
                };

                // update click region
                let handle = self.data[i].handle.clone();
                
                match self.click_regions.contains_key(&handle) {
                    true => {
                        self.click_regions.get_mut(&handle).unwrap().clicked = false;
                        self.click_regions.get_mut(&handle).unwrap().age = 130;
                        self.click_regions.get_mut(&handle).unwrap().rect = Rect { x0: x0, y0: y0, x1: x3, y1: y3 };
                        selected = self.click_regions.get_mut(&handle).unwrap().selected;
                    },
                    false => {
                        self.click_regions.insert(handle, ClickRegion { rect: Rect { x0: x0, y0: y0, x1: x3, y1: y3 }, selected: false, clicked: false, pressing: false, age: 2 });
                    }
                };

                let mut selmod: usize = 0;
                if selected { selmod = 9; }

                let vertex_data = build_billboard(x0, y0, x1, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[18 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y0, x2, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[19 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y0, x3, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[20 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                //

                let vertex_data = build_billboard(x0, y1, x1, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[21 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y1, x2, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[22 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y1, x3, y2);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[23 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                //

                let vertex_data = build_billboard(x0, y2, x1, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[24 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x1, y2, x2, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[25 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                let vertex_data = build_billboard(x2, y2, x3, y3);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[26 + selmod];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);


                // box text

                let txt = if selected { format!("{}|", value) } else { value.clone() };

                // get text width
                let bytes = txt.as_bytes();
                let mut basex: f32 = 0.0;

                for i in 0..bytes.len() {
                    let idx = bytes[i] as u16;
                    let data = self.font.data[&idx];
                    let advance = data[6] as f32;
                    basex += advance;
                }

                let txt_width = basex as i32;

                let bcx = x0 + 8;
                let bcy = y0 + 2;

                let ww = 256.0;
                let hh = 64.0;
                let mut vertex_data: Vec<f32> = Vec::new();

                let bytes = txt.as_bytes();
                let num_chars = bytes.len();

                let mut basex: f32 = 0.0;

                for i in 0..num_chars {
                    let idx = bytes[i] as u16;

                    let data = self.font.data[&idx];
                    let dx = data[0] as f32 / ww;
                    let dy = data[1] as f32 / hh;
                    let dw = data[2] as f32;
                    let dh = data[3] as f32;
                    let dwt = dw as f32 / ww;
                    let dht = dh as f32 / hh;
                    let dxoff = data[4] as f32;
                    let dyoff = data[5] as f32;
                    let advance = data[6] as f32;

                    let p0 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + 0.0 + dyoff, dx, dy];
                    let p1 = [bcx as f32 + basex + 0.0 + dxoff, bcy as f32 + dh + dyoff, dx, dy + dht];
                    let p2 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + dh + dyoff, dx + dwt, dy + dht];
                    let p3 = [bcx as f32 + basex + dw + dxoff, bcy as f32 + 0.0 + dyoff, dx + dwt, dy];

                    for i in 0..4 {
                        vertex_data.push(p0[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p1[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p2[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p0[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p2[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p3[i]);
                    }

                    basex += advance;
                }

                let idx = self.pool_get(gl);
                self.pool[idx].renderer = RENDERER_TEXT;                
                self.pool[idx].num_chars = num_chars;

                let data_ptr = vertex_data.as_ptr() as *const _;
                gl.buffer_font_data(self.pool[idx].vao, self.pool[idx].vbo, num_chars as i32, data_ptr);

                previous_cx = x3 + 4;
                previous_cy = cy;
                cy += h0;

            }
            else if WIDGET_LABEL == self.data[i].class
            {
                // same line offset
                cx = base_cx;
                if self.data[i].same_line {
                    cx = previous_cx;
                    cy = previous_cy;
                }

                let txt = self.data[i].name.clone();

                // get text width
                let bytes = txt.as_bytes();
                let mut basex: f32 = 0.0;

                for i in 0..bytes.len() {
                    let idx = bytes[i] as u16;
                    let data = self.font.data[&idx];
                    let advance = data[6] as f32;
                    basex += advance;
                }

                let txt_width = basex as i32;

                let ww = 256.0;
                let hh = 64.0;
                let mut vertex_data: Vec<f32> = Vec::new();

                let bytes = txt.as_bytes();
                let num_chars = bytes.len();

                let mut basex: f32 = 0.0;

                let mut tcx = cx;
                let mut tcy = cy;

                if RECT_AUTO != self.data[i].rect.x0 {
                    tcx += self.data[i].rect.x0;
                }
                if RECT_AUTO != self.data[i].rect.y0 {
                    tcy += self.data[i].rect.y0;
                }

                if RECT_CENTER == self.data[i].rect.x0 {
                    tcx = (rect.x1 + rect.x0 - txt_width as i32) / 2;
                }
                if RECT_RIGHT == self.data[i].rect.x0 && self.data[i].expand {
                    tcx = rect.x1 - txt_width as i32 - 8;
                }
                else if RECT_RIGHT == self.data[i].rect.x0 && !self.data[i].expand {
                    tcx = rect.x0 + self.data[i].rect.x1 - txt_width as i32 - 8;
                }

                if RECT_CENTER == self.data[i].rect.y0 {
                    tcy = (rect.y1 + rect.y0 - 14) / 2;
                }

                if self.data[i].position_absolute {
                    tcx = rect.x0 + self.data[i].rect.x0;
                    tcy = rect.y0 + self.data[i].rect.y0;
                }

                for i in 0..num_chars {
                    let idx = bytes[i] as u16;

                    let data = self.font.data[&idx];
                    let dx = data[0] as f32 / ww;
                    let dy = data[1] as f32 / hh;
                    let dw = data[2] as f32;
                    let dh = data[3] as f32;
                    let dwt = dw as f32 / ww;
                    let dht = dh as f32 / hh;
                    let dxoff = data[4] as f32;
                    let dyoff = data[5] as f32;
                    let advance = data[6] as f32;

                    let p0 = [tcx as f32 + basex + 0.0 + dxoff, tcy as f32 + 0.0 + dyoff, dx, dy];
                    let p1 = [tcx as f32 + basex + 0.0 + dxoff, tcy as f32 + dh + dyoff, dx, dy + dht];
                    let p2 = [tcx as f32 + basex + dw + dxoff, tcy as f32 + dh + dyoff, dx + dwt, dy + dht];
                    let p3 = [tcx as f32 + basex + dw + dxoff, tcy as f32 + 0.0 + dyoff, dx + dwt, dy];

                    for i in 0..4 {
                        vertex_data.push(p0[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p1[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p2[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p0[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p2[i]);
                    }
                    for i in 0..4 {
                        vertex_data.push(p3[i]);
                    }

                    basex += advance;
                }

                let idx = self.pool_get(gl);
                self.pool[idx].renderer = RENDERER_TEXT;                
                self.pool[idx].num_chars = num_chars;
                self.pool[idx].color = self.data[i].color;

                let data_ptr = vertex_data.as_ptr() as *const _;
                gl.buffer_font_data(self.pool[idx].vao, self.pool[idx].vbo, num_chars as i32, data_ptr);

                previous_cx = cx + basex.floor() as i32 + 4;
                if RECT_AUTO != self.data[i].rect.x1 {
                    previous_cx = cx + self.data[i].rect.x1;
                }

                previous_cy = tcy;
                //cy += 16;
                cy = tcy + 16;
            }
            else if WIDGET_SEPARATOR == self.data[i].class {

                cx = base_cx;

                let x0 = cx + self.data[i].rect.x0;
                let y0 = cy - 1 + self.data[i].rect.y0;
                let w0: i32 = (rect.x1 - x0 + self.data[i].rect.x1) - 8;
                let h0: i32 = 6;

                let x1 = x0 + w0;
                let y1 = y0 + h0;
                
                let vertex_data = build_billboard(x0, y0, x1, y1);
                let idx = self.pool_get(gl);
                self.pool[idx].tex = self.tex[45];
                self.pool[idx].renderer = RENDERER_BILLBOARD;
                gl.buffer_billboard_data(self.pool[idx].vao, self.pool[idx].vbo, vertex_data.as_ptr() as *const _);

                cy += h0 + self.data[i].rect.y0;
                previous_cy = cy;

            }

        }

        self.constructed = true;
    }

    pub fn event(self: &mut Self, mouse_x: i32, mouse_y: i32, event_id: u8) -> bool {

        match event_id {
            EVENT_INPUT_MOUSE_BUTTON_DOWN => {
                for (key, value) in self.click_regions.iter_mut() {
                    value.pressing = false;
                    if mouse_x >= value.rect.x0 &&
                        mouse_y >= value.rect.y0 &&
                        mouse_x < value.rect.x1 &&
                        mouse_y < value.rect.y1 {
                            //println!("{} mouse down!", key);
                            value.pressing = true;
                    }
                }
            },
            EVENT_INPUT_MOUSE_BUTTON_UP => {
                for (key, value) in self.click_regions.iter_mut() {
                    value.pressing = false;
                    value.selected = false;
                    if mouse_x >= value.rect.x0 &&
                        mouse_y >= value.rect.y0 &&
                        mouse_x < value.rect.x1 &&
                        mouse_y < value.rect.y1 {
                            //println!("{} mouse up!", key);
                            value.clicked = true;
                            value.selected = true;
                    }
                }
            },
            EVENT_INPUT_KEYBOARD_RELEASED_BACKSPACE => {
                for (key, value) in self.input_regions.iter_mut() {
                    if self.click_regions.get(key).unwrap().selected {
                        let sz = value.value.len();
                        if 0 < sz {
                            value.modified = true;
                            value.value = format!("{}", &value.value[0..sz-1]);
                        }
                        break;
                    }
                }
            }
            _ => (),
        }

        if EVENT_INPUT_KEYBOARD_RELEASED_0 <= event_id && EVENT_INPUT_KEYBOARD_RELEASED_9 >= event_id {
            println!("key: {}", (event_id - EVENT_INPUT_KEYBOARD_RELEASED_0));
            for (key, value) in self.input_regions.iter_mut() {
                if self.click_regions.get(key).unwrap().selected && value.input_numeric {
                    value.modified = true;
                    value.value = format!("{}{}", value.value, (event_id - EVENT_INPUT_KEYBOARD_RELEASED_0).to_string());
                    break;
                }
            }
        }

        if EVENT_INPUT_KEYBOARD_RELEASED_MINUS == event_id {
            println!("key: {}", EVENT_INPUT_KEYBOARD_RELEASED_MINUS);
            for (key, value) in self.input_regions.iter_mut() {
                if self.click_regions.get(key).unwrap().selected && !value.input_alpha {
                    value.modified = true;
                    value.value = format!("-{}", value.value);
                    break;
                } else if self.click_regions.get(key).unwrap().selected && value.input_alpha {
                    value.modified = true;
                    value.value = format!("{}-", value.value);
                    break;
                }
            }
        }

        if EVENT_INPUT_KEYBOARD_RELEASED_A <= event_id && EVENT_INPUT_KEYBOARD_RELEASED_Z >= event_id {
            let r: &str = "abcdefghijklmnopqrstuvwxyz";
            let idx = (event_id - EVENT_INPUT_KEYBOARD_RELEASED_A) as usize;
            let s = &r[idx..=idx];

            println!("key: {}, {}", (event_id - EVENT_INPUT_KEYBOARD_RELEASED_A), s);
            for (key, value) in self.input_regions.iter_mut() {
                if self.click_regions.get(key).unwrap().selected && value.input_alpha {
                    value.modified = true;
                    value.value = format!("{}{}", value.value, s);
                    break;
                }
            }
        }

        return false;
    }

    pub fn flush_input_widgets(self: &mut Self) {
        // reduce the age on all click regions
        for (key, value) in self.input_regions.iter_mut() {
            value.value = String::from("");
        }
    }

    pub fn get_texture(self: &mut Self, filename: String) -> u32 {

        let handle: u32 = match self.texture_handles.contains_key(&filename) {
            true => *self.texture_handles.get(&filename).unwrap(),
            false => {
                self.delay_load_texture.push(filename);
                TEX_HANDLE_INVALID
            }
        };

        handle
    }

    fn load_texture(self: &mut Self, gl: &Gl, filename: String) {

        let handle: u32 = match self.texture_handles.contains_key(&filename) {
            true => *self.texture_handles.get(&filename).unwrap(),
            false => {
                let h = gl.load_texture_silent(&filename);
                self.texture_handles.insert(filename, h);
                h
            }
        };

        self.tex.push(handle);
    }

    fn initialize(self: &mut Self, gl: &Gl) {

        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_tl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_t.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_tr.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_l.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_c.png")); //self.tex[4] = gl.self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_c_test.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_r.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_bl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_b.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_br.png"));

        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_tl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_t.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_tr.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_l.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_c.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_r.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_bl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_b.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/button_base_br.png"));

        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_tl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_t.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_tr.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_l.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_c.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_r.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_bl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_b.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_base_br.png"));

        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_tl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_t.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_tr.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_l.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_c.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_r.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_bl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_b.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/input_selected_br.png"));

        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_tl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_t.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_tr.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_l.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_c.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_r.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_bl.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_b.png"));
        self.load_texture(gl, String::from("assets/mgfw/gui/panel_simple_br.png"));

        //

        self.load_texture(gl, String::from("assets/mgfw/gui/panel_base_sep.png"));


        self.initialized = true;
    }

    pub fn is_constructed(self: & Self) -> bool {
        self.constructed
    }

    pub fn modified(self: &mut Self, handle: & String) -> bool {
    
        let mut c = false;
        if self.input_regions.contains_key(handle) {
            if self.input_regions.get_mut(handle).unwrap().modified {
                self.input_regions.get_mut(handle).unwrap().modified = false;
                if 0 < self.input_regions.get_mut(handle).unwrap().age {
                    c = true;
                }
            }
        }
        
        if c { println!("{} modified!", handle); }
        c
    }

    pub fn input_value(self: &mut Self, handle: & String) -> String {
    
        if self.input_regions.contains_key(handle) {
            return self.input_regions.get_mut(handle).unwrap().value.clone();
        }
        String::from("")
    }

    pub fn new_frame(self: &mut Self) -> bool {
        if !self.frame_ready { return false; }
        self.data.clear();
        self.constructed = false;
        self.panel_handle = String::from("");
        true
    }

    fn pool_reset(self: &mut Self) {
        self.pool_idx = 0;
    }

    fn pool_grow(self: &mut Self, gl: &Gl) {
        let mut delta = self.pool_size;
        if 0 == delta { delta = 1; }
        let mut v = vec![Object::default(); delta];
        for i in 0..delta {
            v[i].vao = gl.gen_vao();
            v[i].vbo = gl.gen_vbo();
        }
        self.pool.extend(v);
        self.pool_size = self.pool.len();
    }

    fn pool_get(self: &mut Self, gl: &Gl) -> usize {
        if self.pool_idx == self.pool_size { self.pool_grow(gl); }
        self.pool_idx += 1;
        self.pool_idx - 1
    }

    pub fn render(self: & Self, gl: &Gl) {

        if 0 == self.pool_size { return; }

        let c0 = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
        let c1 = Color { r: 170.0 / 255.0, g: 180.0 / 255.0, b: 230.0 / 255.0, a: 1.0 };
        let c2 = Color { r: c1.r * 0.2, g: c1.g * 0.2, b: c1.b * 0.2, a: 1.0 };

        for i in 0..self.pool_idx {

            if RENDERER_BILLBOARD == self.pool[i].renderer {
                gl.draw_billboard(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.pool[i].vao,
                    self.pool[i].tex as u16,
                    c0,
                    false,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                );
            }
            else if RENDERER_TEXT == self.pool[i].renderer {
                gl.draw_text(
                    1.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.pool[i].vao,
                    self.pool[i].num_chars as i32,
                    c2,
                );

                gl.draw_text(
                    1.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.pool[i].vao,
                    self.pool[i].num_chars as i32,
                    c2,
                );

                gl.draw_text(
                    0.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.pool[i].vao,
                    self.pool[i].num_chars as i32,
                    c2,
                );

                gl.draw_text(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.pool[i].vao,
                    self.pool[i].num_chars as i32,
                    self.pool[i].color,
                );
            }
        }
    }

    pub fn update(self: &mut Self, gl: &Gl, dt: u128) -> bool {

        // lazy initialize
        if !self.initialized { self.initialize(gl); }

        self.tick_time += dt;
        
        // 30hz update tick
        while 33333 <= self.tick_time {
            self.tick_time -= 33333;
            self.frame_ready = true;
        }

        // reduce the age on all click regions
        for (key, value) in self.click_regions.iter_mut() {
            if 0 < value.age {
                value.age -= 1;
            }
        }

        // reduce the age on all click regions
        for (key, value) in self.input_regions.iter_mut() {
            if 0 < value.age {
                value.age -= 1;
            }
        }

        // build the previous frame for rendering
        if !self.is_constructed() {
            self.construct(gl);
            return true;
        }

        false
    }
}
