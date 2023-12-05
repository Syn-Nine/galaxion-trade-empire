use super::*;
use super::super::*;

use std::collections::VecDeque;

const WIDGET_INVALID: u8 = 0;
const WIDGET_ROOT: u8 = 1;
const WIDGET_PANEL: u8 = 2;
const WIDGET_LABEL: u8 = 3;
const WIDGET_BUTTON: u8 = 4;
const WIDGET_SCROLLAREA: u8 = 5;
const WIDGET_ICON: u8 = 6;
const WIDGET_TAB_GROUP: u8 = 7;
const WIDGET_TAB: u8 = 8;
const WIDGET_TAB_CONTENTS: u8 = 9;
const WIDGET_TILESET: u8 = 10;
const WIDGET_TILEMAP: u8 = 111;
const WIDGET_SCROLLAREA_MASK: u8 = 12;
const WIDGET_SCROLLAREA_CONTENTS: u8 = 13;
const WIDGET_TREE: u8 = 14;
const WIDGET_SLIDER: u8 = 15;
const WIDGET_PLOTGRID: u8 = 16;
const WIDGET_PLOTDATA: u8 = 17;
const WIDGET_SUNKEN: u8 = 18;

//
pub const ICON_INVALID: u8 = 0;
pub const ICON_SAVE: u8 = 1;
pub const ICON_SAVE_ALL: u8 = 2;
pub const ICON_REVERT: u8 = 3;
pub const ICON_REVERT_ALL: u8 = 4;
pub const ICON_NEW_FILE: u8 = 5;
pub const ICON_TRASH: u8 = 6;
pub const ICON_UNDO: u8 = 7;
pub const ICON_REDO: u8 = 8;
pub const ICON_PENCIL: u8 = 9;
pub const ICON_EYEDROPPER: u8 = 10;
pub const ICON_ERASER: u8 = 11;
pub const ICON_BUCKET: u8 = 12;
pub const ICON_SELECT: u8 = 13;

//
pub const HALIGN_LEFT: u8 = 0;
pub const HALIGN_CENTER: u8 = 1;
pub const HALIGN_RIGHT: u8 = 2;
pub const VALIGN_TOP: u8 = 3;
pub const VALIGN_CENTER: u8 = 4;
pub const VALIGN_BOTTOM: u8 = 5;

//
pub const PLOT_DATA_MARKER_WHITE: u8 = 0;
pub const PLOT_DATA_MARKER_RED: u8 = 1;
pub const PLOT_DATA_MARKER_ORANGE: u8 = 2;
pub const PLOT_DATA_MARKER_GREEN: u8 = 3;
pub const PLOT_DATA_MARKER_GREY: u8 = 4;
pub const PLOT_DATA_MARKER_PURPLE: u8 = 5;
pub const PLOT_DATA_MARKER_YELLOW: u8 = 6;
pub const PLOT_DATA_MARKER_BLUE: u8 = 7;

//
pub const PARENT_WINDOW: usize = 0;
pub const TREE_ROOT: usize = 0;

fn swap_u32(lhs: u32, rhs: u32) -> (u32, u32) {
    (rhs, lhs)
}

fn swap_i32(lhs: i32, rhs: i32) -> (i32, i32) {
    (rhs, lhs)
}

#[derive(Clone)]
pub struct Callback {
    pub enabled: bool,
    //
    pub id: u8,
    pub idata0: i32,
    pub idata1: i32,
    /*pub fdata0: f32,
    pub fdata1: f32,
    pub sdata0: String,
    pub sdata1: String,*/
}

impl Default for Callback {
    fn default() -> Self {
        Callback {
            enabled: false,
            id: 0,
            idata0: 0,
            idata1: 0,
            /*fdata0: 0.0,
            fdata1: 0.0,
            sdata0: String::from(""),
            sdata1: String::from(""),*/
        }
    }
}

struct TreeNode {
    parent: usize,
    children: Vec<usize>,
    value: String,
    open: bool,
    highlighted: bool,
    grid: UIgrid,
    callback: Callback,
}

impl Default for TreeNode {
    fn default() -> Self {
        TreeNode {
            parent: TREE_ROOT,
            children: Vec::new(),
            value: String::from(""),
            open: false,
            highlighted: false,
            grid: uigrid::new(0.0, 0.0, 0.0, 0.0),
            callback: Callback::default(),
        }
    }
}

pub struct Widget {
    grid: UIgrid,
    class: u8,
    value: String,
    tab_offset: i32,
    valign: u8,
    halign: u8,
    highlight: bool,
    highlight_on_reconstruct_tab: bool,
    background: bool,
    enabled: bool,
    icon: u8,
    parent: usize,
    children: Vec<usize>,
    reconstruct: bool,
    gl_data_idx: usize,
    vao_owner: bool,
    callback: Callback,
    toggle: bool,
    image_loaded: bool,
    image_file: String,
    tex_handle: u32,
    tex_width: u16,
    tex_height: u16,
    tex_span: u16,
    tex_count: u16,
    tile_set: usize,
    tile_columns: u16,
    tile_rows: u16,
    tile_width: u16,
    tile_height: u16,
    tile_data: Vec<u16>,
    tree_nodes: Vec<TreeNode>,
    slider_offset: f32,
    plotgrid_divs: (u16, u16),
    plotdata_marker: u8,
    plotdata: Vec<(f32, f32)>,
    visible: bool,
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            grid: new(0.0, 0.0, 0.0, 0.0),
            class: WIDGET_INVALID,
            value: String::from(""),
            tab_offset: 0,
            valign: VALIGN_CENTER,
            halign: HALIGN_CENTER,
            highlight: false,
            highlight_on_reconstruct_tab: false,
            background: false,
            enabled: true,
            icon: ICON_INVALID,
            parent: PARENT_WINDOW,
            children: Vec::new(),
            reconstruct: true,
            gl_data_idx: usize::MAX,
            vao_owner: false,
            callback: Callback::default(),
            toggle: false,
            image_loaded: false,
            image_file: String::from(""),
            tex_handle: 0,
            tex_width: 0,
            tex_height: 0,
            tex_span: 0,
            tex_count: 0,
            tile_set: 0,
            tile_columns: 0,
            tile_rows: 0,
            tile_width: 0,
            tile_height: 0,
            tile_data: Vec::new(),
            tree_nodes: Vec::new(),
            slider_offset: 0.0,
            plotgrid_divs: (1, 1),
            plotdata_marker: 0,
            plotdata: Vec::new(),
            visible: true,
        }
    }
}

pub fn panel(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_PANEL,
        vao_owner: true,
        .. Default::default()
    }
}

pub fn sunken(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_SUNKEN,
        ..Default::default()
    }
}

pub fn scrollarea(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_SCROLLAREA,
        .. Default::default()
    }
}

fn scrollarea_mask(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_SCROLLAREA_MASK,
        vao_owner: true,
        .. Default::default()
    }
}

fn scrollarea_contents(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_SCROLLAREA_CONTENTS,
        vao_owner: true,
        .. Default::default()
    }
}

pub fn tree(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_TREE,
        tree_nodes: vec![TreeNode::default()], // root node
        .. Default::default()
    }
}

pub fn button(grid: &UIgrid, txt: &String) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_BUTTON,
        value: txt.clone(),
        .. Default::default()
    }
}

pub fn label(grid: &UIgrid, txt: &String) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_LABEL,
        value: txt.clone(),
        .. Default::default()
    }
}

pub fn icon(grid: &UIgrid, icon: u8) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_ICON,
        icon,
        .. Default::default()
    }
}

pub fn slider(grid: &UIgrid, offset: f32) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_SLIDER,
        slider_offset: offset,
        .. Default::default()
    }
}

pub fn plotgrid(grid: &UIgrid, mut xdivs: u16, mut ydivs: u16) -> Widget {
    if xdivs < 1 { xdivs = 1; }
    if ydivs < 1 { ydivs = 1; }
    Widget {
        grid: grid.clone(),
        class: WIDGET_PLOTGRID,
        plotgrid_divs: (xdivs, ydivs),
        ..Default::default()
    }
}

pub fn plotdata(grid: &UIgrid, data: &Vec::<(f32, f32)>, marker: u8) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_PLOTDATA,
        plotdata: data.clone(),
        plotdata_marker: marker,
        ..Default::default()
    }
}

pub fn tab(txt: &String) -> Widget {
    Widget {
        class: WIDGET_TAB,
        value: txt.clone(),
        vao_owner: true,
        .. Default::default()
    }
}

pub fn tab_contents() -> Widget {
    Widget {
        class: WIDGET_TAB_CONTENTS,
        vao_owner: true,
        .. Default::default()
    }
}

pub fn tab_group(grid: &UIgrid) -> Widget {
    Widget {
        grid: grid.clone(),
        class: WIDGET_TAB_GROUP,
        .. Default::default()
    }
}

pub fn tileset(image_file: String, tile_width: u16, tile_height: u16) -> Widget {
    Widget {
        class: WIDGET_TILESET,
        image_file,
        tile_width,
        tile_height,
        .. Default::default()
    }
}

pub fn tilemap(grid: &UIgrid, tile_set: usize, tile_columns: u16, data: &Vec<u16>) -> Widget {
    let n = data.len() as u16;
    assert!(0 != tile_columns);
    assert!(0 != n);
    assert!(0 == n % tile_columns);
    let tile_rows = (n - (n % tile_columns)) / tile_columns;
    assert!(data.len() as u16 == tile_rows * tile_columns);

    Widget {
        grid: grid.clone(),
        class: WIDGET_TILEMAP,
        tile_set,
        tile_columns,
        tile_rows,
        tile_data: data.clone(),
        vao_owner: true,
        .. Default::default()
    }
}

impl Widget {
    pub fn valign(self: Self, val: u8) -> Widget {
        let mut ret = self;
        ret.valign = val;
        ret
    }

    pub fn halign(self: Self, val: u8) -> Widget {
        let mut ret = self;
        ret.halign = val;
        ret
    }

    pub fn highlight(self: Self, val: bool) -> Widget {
        let mut ret = self;
        ret.highlight = val;
        if WIDGET_TAB == ret.class { ret.highlight_on_reconstruct_tab = val; }
        ret
    }

    pub fn background(self: Self, val: bool) -> Widget {
        let mut ret = self;
        ret.background = val;
        ret
    }

    pub fn callback(self: Self, id: u8) -> Widget {
        let mut ret = self;
        ret.callback = Callback { id, enabled: true, .. Default::default() };
        ret
    }

    pub fn callback_i0(self: Self, id: u8, idata0: i32) -> Widget {
        let mut ret = self;
        ret.callback = Callback { id, enabled: true, idata0, .. Default::default() };
        ret
    }

    pub fn callback_i1(self: Self, id: u8, idata0: i32, idata1: i32) -> Widget {
        let mut ret = self;
        ret.callback = Callback { id, enabled: true, idata0, idata1, .. Default::default() };
        ret
    }

    pub fn enabled(self: Self, enabled: bool) -> Widget {
        let mut ret = self;
        ret.enabled = enabled;
        ret
    }

    pub fn visible(self: Self, visible: bool) -> Widget {
        let mut ret = self;
        ret.visible = visible;
        ret
    }
}

fn add_geometry(vertex_data: &mut Vec<f32>, x0: i32, y0: i32, x1: i32, y1: i32, u0: i32, v0: i32, u1: i32, v1: i32) {

    let tx = 56.0;
    let ty = 48.0;

    let u0 = u0 as f32 / tx;
    let u1 = u1 as f32 / tx;
    let v0 = v0 as f32 / ty;
    let v1 = v1 as f32 / ty;
    
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
}

fn add_icon(vertex_data: &mut Vec<f32>, icon: u8, grid: &mut UIgrid, halign: u8, valign: u8, highlight: bool, toggle: bool) {

    let tx = 56.0;
    let ty = 48.0;

    // default center alignment
    let mut x0 = ((grid.x0 + grid.x1) / 2.0).floor() - 6.0;
    let mut y0 = ((grid.y0 + grid.y1) / 2.0).floor() - 6.0;

    if HALIGN_LEFT == halign { x0 = grid.x0.floor(); }
    if HALIGN_RIGHT == halign { x0 = grid.x1.floor() - 12.0; }
    if VALIGN_TOP == valign { y0 = grid.y0.floor(); }
    if VALIGN_BOTTOM == valign { y0 = grid.y1.floor() - 12.0 - 1.0; }

    let x1 = x0 + 12.0;
    let y1 = y0 + 12.0;

    grid.x0 = x0 as f32;
    grid.x1 = x1 as f32;
    grid.y0 = y0 as f32;
    grid.y1 = y1 as f32;

    let mut u0 = 0;
    let mut v0 = 36;
    
    if toggle {
        u0 = 12;
        v0 = 24;
    }
    if highlight {
        u0 = 12;
        v0 = 36;
    }

    let u1 = u0 + 12;
    let v1 = v0 + 12;

    let u0 = u0 as f32 / tx;
    let u1 = u1 as f32 / tx;
    let v0 = v0 as f32 / ty;
    let v1 = v1 as f32 / ty;
    
    // background
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

    let mut u0 = 0;
    let mut u1 = 0;
    let mut v0 = 0;
    let mut v1 = 0;

    match icon {
        ICON_SAVE => {
            u0 = 40;
            u1 = u0 + 8;
            v0 = 32;
            v1 = v0 + 8;
        },
        ICON_SAVE_ALL => {
            u0 = 40;
            u1 = u0 + 8;
            v0 = 24;
            v1 = v0 + 8;
        },
        ICON_REVERT => {
            u0 = 32;
            u1 = u0 + 8;
            v0 = 32;
            v1 = v0 + 8;
        },
        ICON_REVERT_ALL => {
            u0 = 32;
            u1 = u0 + 8;
            v0 = 24;
            v1 = v0 + 8;
        },
        ICON_NEW_FILE => {
            u0 = 40;
            u1 = u0 + 8;
            v0 = 40;
            v1 = v0 + 8;
        },
        ICON_TRASH => {
            u0 = 32;
            u1 = u0 + 8;
            v0 = 40;
            v1 = v0 + 8;
        },
        ICON_UNDO => {
            u0 = 32;
            u1 = u0 + 8;
            v0 = 16;
            v1 = v0 + 8;
        },
        ICON_REDO => {
            u0 = 40;
            u1 = u0 + 8;
            v0 = 16;
            v1 = v0 + 8;
        },
        ICON_PENCIL => {
            u0 = 32;
            u1 = u0 + 8;
            v0 = 8;
            v1 = v0 + 8;
        },
        ICON_EYEDROPPER => {
            u0 = 24;
            u1 = u0 + 8;
            v0 = 24;
            v1 = v0 + 8;
        },
        ICON_ERASER => {
            u0 = 40;
            u1 = u0 + 8;
            v0 = 8;
            v1 = v0 + 8;
        },
        ICON_BUCKET => {
            u0 = 24;
            u1 = u0 + 8;
            v0 = 32;
            v1 = v0 + 8;
        },
        ICON_SELECT => {
            u0 = 24;
            u1 = u0 + 8;
            v0 = 40;
            v1 = v0 + 8;
        },
        _ => ()
    }

    let u0 = u0 as f32 / tx;
    let u1 = u1 as f32 / tx;
    let v0 = v0 as f32 / ty;
    let v1 = v1 as f32 / ty;
    
    let x0 = x0 as f32 + 1.0;
    let y0 = y0 as f32 + 2.0;
    let x1 = x0 + 8.0;
    let y1 = y0 + 8.0;

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
}


fn add_tilemap(vertex_data: &mut Vec<f32>, grid: & UIgrid, tile_columns: u16, map: &Vec<u16>, tileset: &Widget) -> i32 {

    let cols = tile_columns as usize;

    let mut num_tiles = 0;

    let uscale = tileset.tile_width as f32 / tileset.tex_width as f32;
    let vscale = tileset.tile_height as f32 / tileset.tex_height as f32;
    let usub = uscale * 0.0; //(0.2 / tileset.tile_width as f32);
    let vsub = uscale * 0.0; //(0.2 / tileset.tile_height as f32);

    for i in 0..map.len() {
        let t0 = map[i];
        if EMPTY_TILE == t0 as u16 || tileset.tex_count < t0 {
            continue;
        }
        let t0 = t0 - 1;

        let u0 = (t0 % tileset.tex_span) as f32 * uscale + usub;
        let v0 = ((t0 - (t0 % tileset.tex_span)) / tileset.tex_span) as f32 * vscale + vsub;
        let u1 = u0 + uscale - usub;
        let v1 = v0 + vscale - vsub;

        let x0 = grid.x0 + ((i % cols) as f32) * tileset.tile_width as f32;// * 2.0;
        let y0 = grid.y0 + (1.0 * (((i - (i % cols)) / cols) as f32)) * tileset.tile_height as f32;// * 2.0;

        let x1 = x0 + tileset.tile_width as f32;// * 2.0;
        let y1 = y0 + tileset.tile_height as f32;// * 2.0;

        vertex_data.push(x0);
        vertex_data.push(y0);
        vertex_data.push(u0);
        vertex_data.push(v0);
        vertex_data.push(x0);
        vertex_data.push(y1);
        vertex_data.push(u0);
        vertex_data.push(v1);
        vertex_data.push(x1);
        vertex_data.push(y1);
        vertex_data.push(u1);
        vertex_data.push(v1);

        vertex_data.push(x0);
        vertex_data.push(y0);
        vertex_data.push(u0);
        vertex_data.push(v0);
        vertex_data.push(x1);
        vertex_data.push(y1);
        vertex_data.push(u1);
        vertex_data.push(v1);
        vertex_data.push(x1);
        vertex_data.push(y0);
        vertex_data.push(u1);
        vertex_data.push(v0);

        num_tiles += 1;
    }

    num_tiles

}

#[derive(Clone)]
struct GLData {
    tex_vao: u32,
    tex_vbo: u32,
    tex_vertex_data: Vec<f32>,
    tex_count: i32,
    font_vao: u32,
    font_vbo: u32,
    font_vertex_data: Vec<f32>,
    num_chars: i32,
    font_vao_highlight: u32,
    font_vbo_highlight: u32,
    font_vertex_data_highlight: Vec<f32>,
    num_chars_highlight: i32,
    font_vao_disabled: u32,
    font_vbo_disabled: u32,
    font_vertex_data_disabled: Vec<f32>,
    num_chars_disabled: i32,
    //
    alt_tex_vao: u32,
    alt_tex_vbo: u32,
    alt_tex_count: i32,
    alt_font_vao: u32,
    alt_font_vbo: u32,
    alt_num_chars: i32,
    alt_font_vao_highlight: u32,
    alt_font_vbo_highlight: u32,
    alt_num_chars_highlight: i32,
    alt_font_vao_disabled: u32,
    alt_font_vbo_disabled: u32,
    alt_num_chars_disabled: i32,
    //
    swap_after_render: bool,
    //
    tex_handle: u32,
}

pub struct Ugui {
    constructed: bool,
    reconstruct: bool,
    initialized: bool,
    tex_handle: u32,
    data: Vec<Widget>,
    font: std::boxed::Box<fonts::retro_gaming::Font>,
    mouse_x: i32,
    mouse_y: i32,
    tick_time: u128,
    gl_data: Vec<GLData>,
    gl_data_release: Vec<GLData>,
    callbacks: VecDeque<Callback>,
    /*
    frame_ready: bool,
    tex: Vec<u32>,
    pool: Vec<Object>,
    pool_size: usize,
    pool_idx: usize,
    texture_handles: std::boxed::Box<HashMap<String, u32>>,
    //vao: u32,
    //vbo: u32,
    //num_chars: usize,
    click_regions: std::boxed::Box<HashMap<String, ClickRegion>>,
    input_regions: std::boxed::Box<HashMap<String, InputRegion>>,
    panel_handle: String,
    delay_load_texture: Vec<String>,*/
}

impl Ugui {
    pub fn new() -> Self {
        let mut ret = Ugui {
            constructed: false,
            reconstruct: false,
            initialized: false,
            tex_handle: 0,
            data: Vec::new(),
            font: Box::new(fonts::retro_gaming::Font::new()),
            mouse_x: -1,
            mouse_y: -1,
            tick_time: 0,
            gl_data: Vec::new(),
            gl_data_release: Vec::new(),
            callbacks: VecDeque::new(),
            /*frame_ready: false,
            pool: Vec::new(),
            pool_size: 0,
            pool_idx: 0,
            tex: Vec::new(),
            texture_handles: Box::new(HashMap::new()),
            delay_load_texture: Vec::new(),
            //vao: 0,
            //vbo: 0,
            click_regions: Box::new(HashMap::new()),
            input_regions: Box::new(HashMap::new()),
            panel_handle: String::from(""),*/
        };
        
        // create root element
        ret.add_root_element();
        ret
    }

    // 
    pub fn add(self: &mut Self, parent: usize, mut widget: Widget) -> usize {
        let mut idx = self.data.len();
        assert!(parent < idx);
        widget.parent = parent;
        
        if WIDGET_SCROLLAREA == widget.class {
            let grid = widget.grid.scroll_pad();
            self.data.push(widget);
            self.data[parent].children.push(idx);
            return self.add(idx, scrollarea_mask(&grid));
        
        } else if WIDGET_SCROLLAREA_MASK == widget.class {
            let grid = widget.grid.clone();
            self.data.push(widget);
            self.data[parent].children.push(idx);
            return self.add(idx, scrollarea_contents(&grid));

        } else if WIDGET_TREE == widget.class {
            let grid = widget.grid.clone();
            let parent = self.add(parent, scrollarea(&grid));
            widget.parent = parent;
            widget.grid = widget.grid.scroll_pad();
            idx = self.data.len();
            self.data.push(widget);
            self.data[parent].children.push(idx);

        } else if WIDGET_TAB == widget.class {
            // set grid to parent grid
            widget.grid = self.data[widget.parent].grid.clone();
            // calculate tab offset based on current tabs in group
            let mut ofs = 0;
            for i in 0..self.data[widget.parent].children.len() {
                if WIDGET_TAB == self.data[self.data[widget.parent].children[i]].class {
                    ofs += 16 + self.get_text_width(&self.data[self.data[widget.parent].children[i]].value);
                }
            }
            widget.tab_offset = ofs;
            self.data.push(widget);
            self.data[parent].children.push(idx);
            return self.add(idx, tab_contents());

        } else {
            self.data.push(widget);
            self.data[parent].children.push(idx);
        }
        idx
    }

    fn add_root_element(self: &mut Self) {
        self.data.push(Widget { class: WIDGET_ROOT, vao_owner: true, .. Default::default() });
    }

    pub fn add_tree_node(self: &mut Self, tree_idx: usize, node_parent: usize, value: &String) -> usize {
        assert!(tree_idx < self.data.len());
        assert!(WIDGET_TREE == self.data[tree_idx].class);
        assert!(node_parent < self.data[tree_idx].tree_nodes.len());
        let node = TreeNode {
            parent: node_parent,
            value: value.clone(),
            ..Default::default()
        };
        let idx = self.data[tree_idx].tree_nodes.len();
        self.data[tree_idx].tree_nodes.push(node);
        self.data[tree_idx].tree_nodes[node_parent].children.push(idx);
        idx
    }

    pub fn add_tree_node_callback(self: &mut Self, tree_idx: usize, node_parent: usize, value: &String, id: u8) -> usize {
        let ret = self.add_tree_node(tree_idx, node_parent, value);
        self.data[tree_idx].tree_nodes[ret].callback = Callback { id, enabled: true, .. Default::default() };
        ret
    }

    pub fn clear(self: &mut Self) {
        self.constructed = false;

        // clear data
        self.gl_data_release = self.gl_data.clone();
        self.gl_data.clear();
        self.data.clear();
        
        // create root element
        self.add_root_element();
    }

    fn add_text(self: &Self, font_vertex_data: &mut Vec<f32>, txt: &String, grid: &UIgrid, halign: u8, valign: u8) -> i32 {
                
        let txt_width = self.get_text_width(&txt);
        let bytes = txt.as_bytes();
        let num_chars = bytes.len();

        let mut basex: f32 = 0.0;
        let ww = 256.0;
        let hh = 64.0;
        
        // default center alignment
        let mut tcx = ((grid.x0 + grid.x1 - txt_width as f32) / 2.0).floor();
        let mut tcy = ((grid.y0 + grid.y1 - 16.0) / 2.0).floor();

        if HALIGN_LEFT == halign { tcx = grid.x0.floor(); }
        if HALIGN_RIGHT == halign { tcx = grid.x1.floor() - txt_width as f32 - 1.0; }
        if VALIGN_TOP == valign { tcy = grid.y0.floor(); }
        if VALIGN_BOTTOM == valign { tcy = grid.y1.floor() - 16.0; }

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
                font_vertex_data.push(p0[i]);
            }
            for i in 0..4 {
                font_vertex_data.push(p1[i]);
            }
            for i in 0..4 {
                font_vertex_data.push(p2[i]);
            }
            for i in 0..4 {
                font_vertex_data.push(p0[i]);
            }
            for i in 0..4 {
                font_vertex_data.push(p2[i]);
            }
            for i in 0..4 {
                font_vertex_data.push(p3[i]);
            }

            basex += advance;
        }

        num_chars as i32
    }  

    
    fn add_text_tree_recursive(self: &mut Self, vertex_data: &mut Vec<f32>, font_vertex_data: &mut Vec<f32>, font_vertex_data_highlight: &mut Vec<f32>, tree_idx: usize, node_idx: usize, x0: f32, y0: &mut f32) -> (i32, i32, i32) {
        let mut ntex = 0;
        let mut nc = 0;
        let mut nch = 0;
        let mut x1 = x0;
        let txt_height = 12.0;
        let tab_width = 12.0;

        if TREE_ROOT != node_idx {
            self.data[tree_idx].tree_nodes[node_idx].grid = uigrid::new(x0, *y0, self.data[tree_idx].grid.x1 - x0, txt_height);
            let mut txt = format!("{}", self.data[tree_idx].tree_nodes[node_idx].value);
            if !self.data[tree_idx].tree_nodes[node_idx].children.is_empty() {
                if self.data[tree_idx].tree_nodes[node_idx].open {
                    txt = format!("+ {txt}");
                    x1 += tab_width;
                } else {
                    txt = format!("- {txt}");
                }
            }

            if self.data[tree_idx].tree_nodes[node_idx].highlighted {
                // background highlight
                let x0 = self.data[tree_idx].tree_nodes[node_idx].grid.x0.floor() as i32;
                let x1 = self.data[tree_idx].tree_nodes[node_idx].grid.x1.floor() as i32;
                let y0 = self.data[tree_idx].tree_nodes[node_idx].grid.y0.floor() as i32;
                let y1 = self.data[tree_idx].tree_nodes[node_idx].grid.y1.floor() as i32;
                let u0 = 24;
                let u1 = u0 + 4;
                let v0 = 4;
                let v1 = v0 + 4;
                add_geometry(vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                ntex += 1;
            }

            if self.data[tree_idx].tree_nodes[node_idx].highlighted || self.data[tree_idx].tree_nodes[node_idx].open {
                nch += self.add_text(font_vertex_data_highlight, &txt, &self.data[tree_idx].tree_nodes[node_idx].grid, HALIGN_LEFT, VALIGN_CENTER);
            } else {
                nc += self.add_text(font_vertex_data, &txt, &self.data[tree_idx].tree_nodes[node_idx].grid, HALIGN_LEFT, VALIGN_CENTER);
            }
            *y0 += txt_height;
        }

        if TREE_ROOT == node_idx || self.data[tree_idx].tree_nodes[node_idx].open {
            for i in 0..self.data[tree_idx].tree_nodes[node_idx].children.len() {
                let idx = self.data[tree_idx].tree_nodes[node_idx].children[i];
                let (tntex, tnc, tnch) = self.add_text_tree_recursive(vertex_data, font_vertex_data, font_vertex_data_highlight, tree_idx, idx, x1, y0);
                ntex += tntex;
                nc += tnc;
                nch += tnch;
            }
        }

        (ntex, nc, nch)
    }

    fn add_text_tree(self: &mut Self, vertex_data: &mut Vec<f32>, font_vertex_data: &mut Vec<f32>, font_vertex_data_highlight: &mut Vec<f32>, tree_idx: usize) -> (i32, i32, i32) {
        let x0 = self.data[tree_idx].grid.x0;
        let mut y0 = self.data[tree_idx].grid.y0;

        self.add_text_tree_recursive(
            vertex_data,
            font_vertex_data,
            font_vertex_data_highlight,
            tree_idx,
            TREE_ROOT,
            x0,
            &mut y0,
        )
    }

    pub fn get_num_chars(self: & Self, txt: &String) -> usize {
        let bytes = txt.as_bytes();
        bytes.len()
    }

    pub fn get_slider_offset(self: & Self, idx: usize) -> f32 {
        assert!(idx < self.data.len());
        self.data[idx].slider_offset
    }

    pub fn set_slider_offset(self: &mut Self, idx: usize, val: f32) {
        assert!(idx < self.data.len());
        self.data[idx].slider_offset = f32::min(1.0, f32::max(0.0, val));
    }

    pub fn set_callback(self: &mut Self, idx: usize, c: Callback) {
        assert!(idx < self.data.len());
        self.data[idx].callback = c;
    }

    pub fn set_enabled(self: &mut Self, idx: usize, val: bool) {
        assert!(idx < self.data.len());
        self.data[idx].enabled = val;
    }

    pub fn set_highlight(self: &mut Self, idx: usize, val: bool) {
        assert!(idx < self.data.len());
        self.data[idx].highlight = val;
    }

    pub fn set_text(self: &mut Self, idx: usize, val: &String) {
        assert!(idx < self.data.len());
        self.data[idx].value = val.clone();
    }

    pub fn set_tileset(self: &mut Self, idx: usize, image_file: String, tile_width: u16, tile_height: u16) {
        assert!(idx < self.data.len());
        self.data[idx].image_file = image_file;
        self.data[idx].tile_width = tile_width;
        self.data[idx].tile_height = tile_height;
        self.data[idx].image_loaded = false;
    }
    
    pub fn set_plotdata(self: &mut Self, idx: usize, data: &Vec<(f32, f32)>) {
        assert!(idx < self.data.len());
        self.data[idx].plotdata = data.clone();
    }

    pub fn set_visible(self: &mut Self, idx: usize, val: bool) {
        assert!(idx < self.data.len());
        self.data[idx].visible = val;
    }

    pub fn get_text_width(self: & Self, txt: &String) -> i32 {
        let bytes = txt.as_bytes();
        let num_chars = bytes.len();
        let mut basex: f32 = 0.0;
        for i in 0..num_chars {
            let idx = bytes[i] as u16;
            let data = self.font.data[&idx];
            let advance = data[6] as f32;
            basex += advance;
        }
        basex as i32
    }

    pub fn is_constructed(self: & Self) -> bool {
        self.constructed
    }

    fn find_my_gldata_idx(self: & Self, idx: usize) -> usize {
        if self.data[idx].vao_owner { return self.data[idx].gl_data_idx; }
        self.find_my_gldata_idx(self.data[idx].parent)
    }

    fn construct(self: &mut Self, gl: &Gl, idx: usize) {

        let pidx = self.find_parent_container(idx);
        if self.data[idx].reconstruct || self.data[pidx].reconstruct {

            // is element a container?
            if self.data[idx].vao_owner {
                // has the gl_data been created?
                if usize::MAX == self.data[idx].gl_data_idx {
                    // hasn't been created yet, so do it now
                    self.data[idx].gl_data_idx = self.gl_data.len();
                    self.gl_data.push(GLData {
                        tex_vao: gl.gen_vao(),
                        tex_vbo: gl.gen_vbo(),
                        tex_vertex_data: Vec::new(),
                        tex_count: 0,
                        font_vao: gl.gen_vao(),
                        font_vbo: gl.gen_vbo(),
                        font_vertex_data: Vec::new(),
                        num_chars: 0,
                        font_vao_highlight: gl.gen_vao(),
                        font_vbo_highlight: gl.gen_vbo(),
                        font_vertex_data_highlight: Vec::new(),
                        num_chars_highlight: 0,
                        font_vao_disabled: gl.gen_vao(),
                        font_vbo_disabled: gl.gen_vbo(),
                        font_vertex_data_disabled: Vec::new(),
                        num_chars_disabled: 0,
                        //
                        alt_tex_vao: gl.gen_vao(),
                        alt_tex_vbo: gl.gen_vbo(),
                        alt_tex_count: 0,
                        alt_font_vao: gl.gen_vao(),
                        alt_font_vbo: gl.gen_vbo(),
                        alt_num_chars: 0,
                        alt_font_vao_highlight: gl.gen_vao(),
                        alt_font_vbo_highlight: gl.gen_vbo(),
                        alt_num_chars_highlight: 0,
                        alt_font_vao_disabled: gl.gen_vao(),
                        alt_font_vbo_disabled: gl.gen_vbo(),
                        alt_num_chars_disabled: 0,
                        //
                        swap_after_render: false,
                        //
                        tex_handle: self.tex_handle,
                    });
                }

                // reset counts to zero
                self.gl_data[self.data[idx].gl_data_idx].tex_vertex_data.clear();
                self.gl_data[self.data[idx].gl_data_idx].tex_count = 0;
                self.gl_data[self.data[idx].gl_data_idx].font_vertex_data.clear();
                self.gl_data[self.data[idx].gl_data_idx].num_chars = 0;
                self.gl_data[self.data[idx].gl_data_idx].font_vertex_data_highlight.clear();
                self.gl_data[self.data[idx].gl_data_idx].num_chars_highlight = 0;
                self.gl_data[self.data[idx].gl_data_idx].font_vertex_data_disabled.clear();
                self.gl_data[self.data[idx].gl_data_idx].num_chars_disabled = 0;
            }

            // construct me ////////////////////  
            if usize::MAX == self.data[idx].gl_data_idx { 
                self.data[idx].gl_data_idx = self.find_my_gldata_idx(idx);
            }
            let glidx = self.data[idx].gl_data_idx;
            
            // temporary vectors / counts
            let mut tex_vertex_data: Vec<f32> = Vec::new();
            let mut font_vertex_data: Vec<f32> = Vec::new();
            let mut font_vertex_data_highlight: Vec<f32> = Vec::new();
            let mut font_vertex_data_disabled: Vec<f32> = Vec::new();
            let mut tex_count = 0;
            let mut num_chars = 0;
            let mut num_chars_highlight = 0;
            let mut num_chars_disabled = 0;

            if self.data[idx].visible {
                if WIDGET_PANEL == self.data[idx].class {
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 0;
                    let u1 = 4;
                    let v1 = 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32 - 4;
                    let u0 = 0;
                    let v0 = 4;
                    let u1 = 4;
                    let v1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32;
                    let u0 = 0;
                    let v0 = 8;
                    let u1 = 4;
                    let v1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                }
                else if WIDGET_SCROLLAREA == self.data[idx].class {
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 12;
                    let u1 = 4;
                    let v1 = 16;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32 - 4;
                    let u0 = 0;
                    let v0 = 16;
                    let u1 = 4;
                    let v1 = 20;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32;
                    let u0 = 0;
                    let v0 = 20;
                    let u1 = 4;
                    let v1 = 24;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // scroll bars
                    // h background
                    let x0 = self.data[idx].grid.x0.floor() as i32 + 4;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let y0 = self.data[idx].grid.y1.floor() as i32 - 8;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let u1 = u0 + 4;
                    let v0 = 32;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // h bar ///////////////////
                    let h_width = 33;
                    let x0 = self.data[idx].grid.x0.floor() as i32 + 4;
                    let x1 = x0 + 4;
                    let y0 = self.data[idx].grid.y1.floor() as i32 - 8;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let u1 = u0 + 4;
                    let v0 = 24;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let x0 = x1;
                    let x1 = x0 + h_width - 8;
                    let u0 = 4;
                    let u1 = u0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let x0 = x1;
                    let x1 = x0 + 4;
                    let u0 = 8;
                    let u1 = u0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // v background
                    let x0 = self.data[idx].grid.x1.floor() as i32 - 8;
                    let x1 = x0 + 4;
                    let y0 = self.data[idx].grid.y0.floor() as i32 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32 - 8;
                    let u0 = 0;
                    let u1 = u0 + 4;
                    let v0 = 32;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // v bar ///////////////////
                    let v_height = 58;
                    let x0 = self.data[idx].grid.x1.floor() as i32 - 8;
                    let x1 = x0 + 4;
                    let y0 = self.data[idx].grid.y0.floor() as i32 + 4;
                    let y1 = y0 + 4;
                    let u0 = 12;
                    let u1 = u0 + 4;
                    let v0 = 12;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let y0 = y1;
                    let y1 = y0 + v_height - 8;
                    let v0 = 16;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let y0 = y1;
                    let y1 = y0 + 4;
                    let v0 = 20;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                }
                else if WIDGET_SCROLLAREA_MASK == self.data[idx].class {
                    // mid mid
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let y1 = self.data[idx].grid.y1.floor() as i32;
                    let u0 = 4;
                    let u1 = u0 + 4;
                    let v0 = 16;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                }
                else if WIDGET_SUNKEN == self.data[idx].class {
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 12;
                    let u1 = 4;
                    let v1 = 16;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32 - 4;
                    let u0 = 0;
                    let v0 = 16;
                    let u1 = 4;
                    let v1 = 20;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32;
                    let u0 = 0;
                    let v0 = 20;
                    let u1 = 4;
                    let v1 = 24;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;
                }
                else if WIDGET_BUTTON == self.data[idx].class {

                    let hlight = self.data[idx].highlight && self.data[idx].enabled;
                    
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let mut u0 = 12;
                    let v0 = 0;
                    if hlight { u0 = 20; }
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let mut u0 = 4;
                    if hlight { u0 = 24; }
                    let u1 = u0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let mut u0 = 16;
                    if hlight { u0 = 28; }
                    let u1 = u0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32 - 4;
                    let mut u0 = 0;
                    if hlight { u0 = 20; }
                    let v0 = 4;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let mut u0 = 4;
                    let mut v0 = 4;
                    if hlight { u0 = 16; v0 = 40; }
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let mut u0 = 8;
                    let v0 = 4;
                    if hlight { u0 = 28; }
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = self.data[idx].grid.y1.floor() as i32;
                    let mut u0 = 12;
                    let mut v0 = 4;
                    if hlight { u0 = 20; v0 = 8; }
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let mut u0 = 4;
                    if hlight { u0 = 24; }
                    let v0 = 8;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let mut u0 = 16;
                    let mut v0 = 4;
                    if hlight { u0 = 28; v0 = 8; }
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    if !hlight {
                        if self.data[idx].enabled {
                            num_chars += self.add_text(&mut font_vertex_data, &self.data[idx].value.clone(), &self.data[idx].grid, HALIGN_CENTER, VALIGN_CENTER);
                        } else {
                            num_chars_disabled += self.add_text(&mut font_vertex_data_disabled, &self.data[idx].value.clone(), &self.data[idx].grid, HALIGN_CENTER, VALIGN_CENTER);
                        }

                    } else {
                        num_chars_highlight += self.add_text(&mut font_vertex_data_highlight, &self.data[idx].value.clone(), &self.data[idx].grid, HALIGN_CENTER, VALIGN_CENTER);
                    }
                }
                else if WIDGET_SLIDER == self.data[idx].class {
                    
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = ((self.data[idx].grid.y0 + self.data[idx].grid.y1) * 0.5 - 4.0).floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 0;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 4;
                    let v0 = 32;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let v0 = 0;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot left
                    let x0 = self.data[idx].grid.x0.floor() as i32;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 8;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot mid
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                    let u0 = 8;
                    let v0 = 32;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // bot right
                    let x0 = x1;
                    let x1 = self.data[idx].grid.x1.floor() as i32;
                    let u0 = 8;
                    let v0 = 8;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // slider //////////////////////////////////////////////////////////////////////////

                    if !self.data[idx].highlight {
                        let m = self.data[idx].grid.x1 - 12.0 - self.data[idx].grid.x0;
                        let m = (m * self.data[idx].slider_offset).floor() as i32;

                        let x0 = self.data[idx].grid.x0.floor() as i32 + m;
                        let y0 = ((self.data[idx].grid.y0 + self.data[idx].grid.y1) * 0.5 - 6.0).floor() as i32;
                        let x1 = x0 + 12;
                        let y1 = y0 + 12;
                        let u0 = 20;
                        let v0 = 12;
                        let u1 = u0 + 12;
                        let v1 = v0 + 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    
                    } else {
                        let m = self.data[idx].grid.x1 - 8.0 - self.data[idx].grid.x0;
                        let m = (m * self.data[idx].slider_offset).floor() as i32;

                        let x0 = self.data[idx].grid.x0.floor() as i32 + m;
                        let y0 = ((self.data[idx].grid.y0 + self.data[idx].grid.y1) * 0.5 - 6.0).floor() as i32;
                        let x1 = x0 + 8;
                        let y1 = y0 + 12;
                        let u0 = 48;
                        let v0 = 12;
                        let u1 = u0 + 8;
                        let v1 = v0 + 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                    }


                }
                else if WIDGET_TAB == self.data[idx].class {

                    let txt = self.data[idx].value.clone();
                    let txt_width = self.get_text_width(&txt) + 8;
                    let tab_offset = self.data[idx].tab_offset;
                    let tab_height = 12;

                    // tab geometry
                    // top left
                    let x0 = self.data[idx].grid.x0.floor() as i32 + tab_offset;
                    let y0 = self.data[idx].grid.y0.floor() as i32;
                    let x1 = x0 + 4;
                    let y1 = y0 + 4;
                    let u0 = 0;
                    let v0 = 0;
                    let u1 = 4;
                    let v1 = 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top mid - tab
                    let x0 = x1;
                    let x1 = x0 + txt_width;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // top right - tab
                    let x0 = x1;
                    let x1 = x0 + 4;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid left - tab
                    let x0 = self.data[idx].grid.x0.floor() as i32 + tab_offset;
                    let y0 = y1;
                    let x1 = x0 + 4;
                    let y1 = y0 + tab_height;
                    let u0 = 0;
                    let v0 = 4;
                    let u1 = 4;
                    let v1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid mid - tab
                    let x0 = x1;
                    let x1 = x0 + txt_width;
                    let y1 = y0 + tab_height;
                    let u0 = 4;
                    let u1 = 8;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // mid right - tab
                    let x0 = x1;
                    let x1 = x0 + 4;
                    let y1 = y0 + tab_height;
                    let u0 = 8;
                    let u1 = 12;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    if self.data[idx].highlight_on_reconstruct_tab {
                        // inner bottom left corner - tab
                        let x0 = self.data[idx].grid.x0.floor() as i32 + tab_offset;
                        let x1 = x0 + 4;
                        let y0 = y1;
                        let y1 = y0 + 4;
                        let mut u0 = 12;
                        let mut u1 = 16;
                        let mut v0 = 8;
                        let mut v1 = 12;
                        if 0 == tab_offset {
                            u0 = 0;
                            u1 = 4;
                            v0 = 4;
                            v1 = 8;
                        }
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // top mid - body
                        let x0 = x1;
                        let x1 = x0 + txt_width;
                        let u0 = 4;
                        let u1 = u0 + 4;
                        let v0 = 4;
                        let v1 = v0 + 4;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // inner bottom right corner - tab
                        let x0 = x1;
                        let x1 = x0 + 4;
                        let u0 = 16;
                        let u1 = 20;
                        let v0 = 8;
                        let v1 = 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        if 0 < tab_offset {
                            // top left - body
                            let x0 = self.data[idx].grid.x0.floor() as i32;
                            let x1 = x0 + 4;
                            let u0 = 0;
                            let v0 = 0;
                            let u1 = 4;
                            let v1 = 4;
                            add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                            tex_count += 1;

                            // top mid - body left
                            let x0 = x1;
                            let x1 = x0 + tab_offset - 4;
                            let u0 = 4;
                            let u1 = 8;
                            let v0 = 0;
                            let v1 = 4;
                            add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                            tex_count += 1;
                        }

                        // top mid - body right
                        let x0 = self.data[idx].grid.x0.floor() as i32 + tab_offset + txt_width + 8;
                        let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                        let u0 = 4;
                        let u1 = 8;
                        let v0 = 0;
                        let v1 = 4;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // top right - body
                        let x0 = x1;
                        let x1 = x0 + 4;
                        let u0 = 8;
                        let u1 = 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // mid left - body
                        let x0 = self.data[idx].grid.x0.floor() as i32;
                        let x1 = x0 + 4;
                        let y0 = y1;
                        let y1 = self.data[idx].grid.y1.floor() as i32 - 4;
                        let u0 = 0;
                        let v0 = 4;
                        let u1 = 4;
                        let v1 = 8;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // mid mid - body
                        let x0 = x0 + 4;
                        let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                        let u0 = 4;
                        let u1 = 8;
                        let v0 = 4;
                        let v1 = 8;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // mid right - body
                        let x0 = x1;
                        let x1 = x0 + 4;
                        let u0 = 8;
                        let u1 = 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // bot left
                        let x0 = self.data[idx].grid.x0.floor() as i32;
                        let y0 = y1;
                        let x1 = x0 + 4;
                        let y1 = self.data[idx].grid.y1.floor() as i32;
                        let u0 = 0;
                        let v0 = 8;
                        let u1 = 4;
                        let v1 = 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // bot mid
                        let x0 = x1;
                        let x1 = self.data[idx].grid.x1.floor() as i32 - 4;
                        let u0 = 4;
                        let u1 = 8;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;

                        // bot right
                        let x0 = x1;
                        let x1 = self.data[idx].grid.x1.floor() as i32;
                        let u0 = 8;
                        let u1 = 12;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    }

                    let mut txt_grid = self.data[idx].grid.clone();
                    txt_grid.x0 = txt_grid.x0 + 4.0 + tab_offset as f32;
                    txt_grid.x1 = txt_grid.x0 + txt_width as f32;
                    txt_grid.y0 = txt_grid.y0 + 4.0;
                    txt_grid.y1 = txt_grid.y0 + tab_height as f32 - 2.0;

                    if !self.data[idx].highlight_on_reconstruct_tab {
                        num_chars += self.add_text(&mut font_vertex_data, &self.data[idx].value.clone(), &txt_grid, HALIGN_CENTER, VALIGN_CENTER);
                    } else {
                        num_chars_highlight += self.add_text(&mut font_vertex_data_highlight, &self.data[idx].value.clone(), &txt_grid, HALIGN_CENTER, VALIGN_CENTER);
                    }

                }
                else if WIDGET_LABEL == self.data[idx].class {

                    if self.data[idx].background {
                        // background highlight
                        let x0 = self.data[idx].grid.x0.floor() as i32;
                        let x1 = self.data[idx].grid.x1.floor() as i32;
                        let y0 = self.data[idx].grid.y0.floor() as i32;
                        let y1 = self.data[idx].grid.y1.floor() as i32;
                        let u0 = 24;
                        let u1 = u0 + 4;
                        let v0 = 4;
                        let v1 = v0 + 4;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    }
                
                    if !self.data[idx].highlight {
                        if self.data[idx].enabled {
                            num_chars += self.add_text(&mut font_vertex_data, &self.data[idx].value.clone(), &self.data[idx].grid, self.data[idx].halign, self.data[idx].valign);
                        } else {
                            num_chars_disabled += self.add_text(&mut font_vertex_data_disabled, &self.data[idx].value.clone(), &self.data[idx].grid, self.data[idx].halign, self.data[idx].valign);
                        }

                    } else {
                        num_chars_highlight += self.add_text(&mut font_vertex_data_highlight, &self.data[idx].value.clone(), &self.data[idx].grid, self.data[idx].halign, self.data[idx].valign);
                    }

                }
                else if WIDGET_PLOTGRID == self.data[idx].class {
                    let bx0 = self.data[idx].grid.x0.floor() as i32;
                    let bx1 = self.data[idx].grid.x1.floor() as i32;
                    let by0 = self.data[idx].grid.y0.floor() as i32;
                    let by1 = self.data[idx].grid.y1.floor() as i32;

                    // background
                    let x0 = bx0;
                    let x1 = bx1;
                    let y0 = by0;
                    let y1 = by1;
                    let u0 = 0;
                    let v0 = 32;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    // horizontal bars
                    let x0 = bx0;
                    let x1 = bx1;
                    let y0 = by0;
                    let y1 = y0 + 4;
                    let u0 = 48;
                    let v0 = 4;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let y0 = by1;
                    let y1 = y0 + 4;
                    let u0 = 48;
                    let v0 = 4;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let w = (self.data[idx].grid.y1 - self.data[idx].grid.y0) / self.data[idx].plotgrid_divs.1 as f32;
                    for i in 0..self.data[idx].plotgrid_divs.1 {
                        let y0 = (self.data[idx].grid.y0 + w * i as f32).floor() as i32;
                        let y1 = y0 + 4;
                        let u0 = 48;
                        let v0 = 4;
                        let u1 = u0 + 4;
                        let v1 = v0 + 4;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    }

                    // vertical bars
                    let x0 = bx0;
                    let x1 = x0 + 4;
                    let y0 = by0;
                    let y1 = by1;
                    let u0 = 52;
                    let v0 = 4;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let x0 = bx1 - 1;
                    let x1 = x0 + 4;
                    let u0 = 52;
                    let v0 = 4;
                    let u1 = u0 + 4;
                    let v1 = v0 + 4;
                    add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                    tex_count += 1;

                    let w = (self.data[idx].grid.x1 - self.data[idx].grid.x0) / self.data[idx].plotgrid_divs.0 as f32;
                    for i in 0..self.data[idx].plotgrid_divs.0 {
                        let x0 = (self.data[idx].grid.x0 + w * i as f32).floor() as i32;
                        let x1 = x0 + 4;
                        let y0 = by0;
                        let y1 = by1;
                        let u0 = 52;
                        let v0 = 4;
                        let u1 = u0 + 4;
                        let v1 = v0 + 4;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    }
                }
                else if WIDGET_PLOTDATA == self.data[idx].class {
                    
                    let w0 = self.data[idx].grid.x1 - self.data[idx].grid.x0 - 2.0;
                    let h0 = self.data[idx].grid.y1 - self.data[idx].grid.y0 - 1.0;

                    let bx0 = self.data[idx].grid.x0;
                    let by0 = self.data[idx].grid.y0;

                    for i in 0..self.data[idx].plotdata.len() {
                        let x0 = (bx0 + self.data[idx].plotdata[i].0 * w0).round() as i32;
                        let y0 = (by0 + self.data[idx].plotdata[i].1 * h0).round() as i32;
                        let x1 = x0 + 2;
                        let y1 = y0 + 2;
                        let (u0, v0) = match self.data[idx].plotdata_marker {
                            PLOT_DATA_MARKER_WHITE => (48, 0),
                            PLOT_DATA_MARKER_RED => (50, 0),
                            PLOT_DATA_MARKER_ORANGE => (52, 0),
                            PLOT_DATA_MARKER_GREEN => (54, 0),
                            PLOT_DATA_MARKER_GREY => (48, 2),
                            PLOT_DATA_MARKER_PURPLE => (50, 2),
                            PLOT_DATA_MARKER_YELLOW => (52, 2),
                            PLOT_DATA_MARKER_BLUE => (54, 2),
                            _ => (0, 0)
                        };
                        let u1 = u0 + 2;
                        let v1 = v0 + 2;
                        add_geometry(&mut tex_vertex_data, x0, y0, x1, y1, u0, v0, u1, v1);
                        tex_count += 1;
                    }
                }
                else if WIDGET_TREE == self.data[idx].class {
                    let (ntex, nc, nch) = self.add_text_tree(&mut tex_vertex_data, &mut font_vertex_data, &mut font_vertex_data_highlight, idx);
                    tex_count += ntex;
                    num_chars += nc;
                    num_chars_highlight += nch;

                }
                else if WIDGET_ICON == self.data[idx].class {
                    let halign = self.data[idx].halign;
                    let valign = self.data[idx].valign;
                    let highlight = self.data[idx].highlight;
                    let toggle = self.data[idx].toggle;
                    add_icon(&mut tex_vertex_data, self.data[idx].icon, &mut self.data[idx].grid, halign, valign, highlight, toggle);
                    tex_count += 2;
                }
                else if WIDGET_TILESET == self.data[idx].class {
                    // if image lot loaded, load it now
                    if !self.data[idx].image_loaded {
                        println!("loading tileset: {}", self.data[idx].image_file.clone());
                        let (tex, tex_w, tex_h) = gl.load_texture_ext(&self.data[idx].image_file);
                        self.data[idx].tex_handle = tex;
                        self.data[idx].tex_width = tex_w;
                        self.data[idx].tex_height = tex_h;
                        if 0 == self.data[idx].tile_width { self.data[idx].tile_width = tex_w; }
                        if 0 == self.data[idx].tile_height { self.data[idx].tile_height = tex_h; }
                        let tile_width = self.data[idx].tile_width;
                        let tile_height = self.data[idx].tile_height;
                        self.data[idx].tex_span = (tex_w - (tex_w % tile_width)) / tile_width;
                        self.data[idx].tex_count = self.data[idx].tex_span * (tex_h - (tex_h % tile_height)) / tile_height;
                        self.data[idx].image_loaded = true;
                    }
                }
                else if WIDGET_TILEMAP == self.data[idx].class {

                    let tile_set = self.data[idx].tile_set;
                    assert!(tile_set < self.data.len());
                    //tex_count += add_tilemap(&mut tex_vertex_data, &self.data[self.data[idx].parent].grid, self.data[idx].tile_columns, &self.data[idx].tile_data, &self.data[tile_set]);
                    tex_count += add_tilemap(&mut tex_vertex_data, &self.data[idx].grid, self.data[idx].tile_columns, &self.data[idx].tile_data, &self.data[tile_set]);
                    self.gl_data[glidx].tex_handle = self.data[tile_set].tex_handle;
                }
            }

            // add temporary data to containers & consume
            if 0 < tex_count {
                self.gl_data[glidx].tex_vertex_data.append(&mut tex_vertex_data);
                self.gl_data[glidx].tex_count += tex_count;
            }
            if 0 < num_chars {
                self.gl_data[glidx].font_vertex_data.append(&mut font_vertex_data);
                self.gl_data[glidx].num_chars += num_chars;
            }
            if 0 < num_chars_highlight {
                self.gl_data[glidx].font_vertex_data_highlight.append(&mut font_vertex_data_highlight);        
                self.gl_data[glidx].num_chars_highlight += num_chars_highlight;
            }
            if 0 < num_chars_disabled {
                self.gl_data[glidx].font_vertex_data_disabled.append(&mut font_vertex_data_disabled);        
                self.gl_data[glidx].num_chars_disabled += num_chars_disabled;
            }
        }

        // construct children
        for i in 0..self.data[idx].children.len() {
            self.construct(gl, self.data[idx].children[i]);
        }

        if self.data[idx].reconstruct {
            let glidx = self.data[idx].gl_data_idx;
            // done constructing everything, push to gl        
            if 0 < self.gl_data[glidx].tex_count {
                gl.buffer_billboard_array_data(self.gl_data[glidx].tex_vao, self.gl_data[glidx].tex_vbo, self.gl_data[glidx].tex_vertex_data.as_ptr() as *const _, self.gl_data[glidx].tex_count);
            }
            if 0 < self.gl_data[glidx].num_chars {
                gl.buffer_font_data(self.gl_data[glidx].font_vao, self.gl_data[glidx].font_vbo, self.gl_data[glidx].num_chars, self.gl_data[glidx].font_vertex_data.as_ptr() as *const _);
            }
            if 0 < self.gl_data[glidx].num_chars_highlight {
                gl.buffer_font_data(self.gl_data[glidx].font_vao_highlight, self.gl_data[glidx].font_vbo_highlight, self.gl_data[glidx].num_chars_highlight, self.gl_data[glidx].font_vertex_data_highlight.as_ptr() as *const _);
            }
            if 0 < self.gl_data[glidx].num_chars_disabled {
                gl.buffer_font_data(self.gl_data[glidx].font_vao_disabled, self.gl_data[glidx].font_vbo_disabled, self.gl_data[glidx].num_chars_disabled, self.gl_data[glidx].font_vertex_data_disabled.as_ptr() as *const _);
            }

            self.gl_data[glidx].swap_after_render = true;
            self.data[idx].reconstruct = false;
        }

        self.constructed = true;
        self.reconstruct = false;
    }

    pub fn event(self: &mut Self, mouse_x: i32, mouse_y: i32, event_id: u8) -> bool {
        
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;

        match event_id {
            EVENT_INPUT_MOUSE_BUTTON_UP => {
                self.click_scan();
                return true;
            }
            _ => (),
        }

        false
    }

    fn initialize(self: &mut Self, gl: &Gl) {

        self.tex_handle = gl.load_texture_silent(&String::from("assets/mgfw/ugui.png"));
        self.initialized = true;
        self.constructed = false;
    }

    fn render_tree(self: &mut Self, gl: &Gl, idx: usize) {

        let glidx = self.data[idx].gl_data_idx;
        //println!("render tree, idx={}, class={}, gl_data_idx={}", idx, self.data[idx].class, glidx);

        //println!("render tree: {}, {}", self.data[idx].class, self.data[idx].value);

        if WIDGET_SCROLLAREA_MASK == self.data[idx].class {
            gl.enable(gl::STENCIL_TEST);
            gl.start_stencil_mask();
        }

        if self.data[idx].visible {
            // do actual rendering
            if 0 < self.gl_data[glidx].alt_tex_count {
                let c0 = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
                gl.draw_billboard_array(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_tex_vao,
                    self.gl_data[glidx].tex_handle as u16,
                    c0,
                    false,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_tex_count
                );
            }
            if 0 < self.gl_data[glidx].alt_num_chars {
                // background
                let c1 = Color { r: 45.0 / 255.0, g: 51.0 / 255.0, b: 65.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    1.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao,
                    self.gl_data[glidx].alt_num_chars,
                    c1,
                );
                gl.draw_text(
                    1.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao,
                    self.gl_data[glidx].alt_num_chars,
                    c1,
                );
                gl.draw_text(
                    0.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao,
                    self.gl_data[glidx].alt_num_chars,
                    c1,
                );
                // foreground
                let c1 = Color { r: 141.0 / 255.0, g: 174.0 / 255.0, b: 182.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao,
                    self.gl_data[glidx].alt_num_chars,
                    c1,
                );
            }
            if 0 < self.gl_data[glidx].alt_num_chars_highlight {
                // background
                let c2 = Color { r: 45.0 / 255.0, g: 51.0 / 255.0, b: 65.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    1.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_highlight,
                    self.gl_data[glidx].alt_num_chars_highlight,
                    c2,
                );
                gl.draw_text(
                    0.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_highlight,
                    self.gl_data[glidx].alt_num_chars_highlight,
                    c2,
                );
                gl.draw_text(
                    1.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_highlight,
                    self.gl_data[glidx].alt_num_chars_highlight,
                    c2,
                );
                // foreground
                let c2 = Color { r: 228.0 / 255.0, g: 228.0 / 255.0, b: 228.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_highlight,
                    self.gl_data[glidx].alt_num_chars_highlight,
                    c2,
                );
            }
            if 0 < self.gl_data[glidx].alt_num_chars_disabled {
                // background
                let c2 = Color { r: 110.0 / 255.0, g: 107.0 / 255.0, b: 116.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    1.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_disabled,
                    self.gl_data[glidx].alt_num_chars_disabled,
                    c2,
                );
                gl.draw_text(
                    0.0,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_disabled,
                    self.gl_data[glidx].alt_num_chars_disabled,
                    c2,
                );
                gl.draw_text(
                    1.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_disabled,
                    self.gl_data[glidx].alt_num_chars_disabled,
                    c2,
                );
                // foreground
                let c2 = Color { r: 35.0 / 255.0, g: 44.0 / 255.0, b: 46.0 / 255.0, a: 1.0 };
                gl.draw_text(
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                    1.0,
                    self.gl_data[glidx].alt_font_vao_disabled,
                    self.gl_data[glidx].alt_num_chars_disabled,
                    c2,
                );
            }
        }

        if WIDGET_SCROLLAREA_MASK == self.data[idx].class {
            gl.stop_stencil_mask();
        }

        // render children
        let skip = WIDGET_TAB == self.data[idx].class && !self.data[idx].highlight;
        if !skip {
            for i in 0..self.data[idx].children.len() {
                self.render_tree(gl, self.data[idx].children[i]);
            }
        } else {
            //println!("-- skip children");
        }

        if WIDGET_SCROLLAREA_MASK == self.data[idx].class {
            gl.disable(gl::STENCIL_TEST);
        }

        // update double buffers
        if self.gl_data[glidx].swap_after_render {
            (self.gl_data[glidx].tex_vao, self.gl_data[glidx].alt_tex_vao) = swap_u32(self.gl_data[glidx].tex_vao, self.gl_data[glidx].alt_tex_vao);
            (self.gl_data[glidx].tex_vbo, self.gl_data[glidx].alt_tex_vbo) = swap_u32(self.gl_data[glidx].tex_vbo, self.gl_data[glidx].alt_tex_vbo);
            (self.gl_data[glidx].tex_count, self.gl_data[glidx].alt_tex_count) = swap_i32(self.gl_data[glidx].tex_count, self.gl_data[glidx].alt_tex_count);
            //
            (self.gl_data[glidx].font_vao, self.gl_data[glidx].alt_font_vao) = swap_u32(self.gl_data[glidx].font_vao, self.gl_data[glidx].alt_font_vao);
            (self.gl_data[glidx].font_vbo, self.gl_data[glidx].alt_font_vbo) = swap_u32(self.gl_data[glidx].font_vbo, self.gl_data[glidx].alt_font_vbo);
            (self.gl_data[glidx].num_chars, self.gl_data[glidx].alt_num_chars) = swap_i32(self.gl_data[glidx].num_chars, self.gl_data[glidx].alt_num_chars);
            //
            (self.gl_data[glidx].font_vao_highlight, self.gl_data[glidx].alt_font_vao_highlight) = swap_u32(self.gl_data[glidx].font_vao_highlight, self.gl_data[glidx].alt_font_vao_highlight);
            (self.gl_data[glidx].font_vbo_highlight, self.gl_data[glidx].alt_font_vbo_highlight) = swap_u32(self.gl_data[glidx].font_vbo_highlight, self.gl_data[glidx].alt_font_vbo_highlight);
            (self.gl_data[glidx].num_chars_highlight, self.gl_data[glidx].alt_num_chars_highlight) = swap_i32(self.gl_data[glidx].num_chars_highlight, self.gl_data[glidx].alt_num_chars_highlight);
            //
            (self.gl_data[glidx].font_vao_disabled, self.gl_data[glidx].alt_font_vao_disabled) = swap_u32(self.gl_data[glidx].font_vao_disabled, self.gl_data[glidx].alt_font_vao_disabled);
            (self.gl_data[glidx].font_vbo_disabled, self.gl_data[glidx].alt_font_vbo_disabled) = swap_u32(self.gl_data[glidx].font_vbo_disabled, self.gl_data[glidx].alt_font_vbo_disabled);
            (self.gl_data[glidx].num_chars_disabled, self.gl_data[glidx].alt_num_chars_disabled) = swap_i32(self.gl_data[glidx].num_chars_disabled, self.gl_data[glidx].alt_num_chars_disabled);
            //
            self.gl_data[glidx].swap_after_render = false;
            //
            if WIDGET_TAB == self.data[idx].class {
                self.data[idx].highlight = self.data[idx].highlight_on_reconstruct_tab;
            }
        }

    }

    pub fn render(self: &mut Self, gl: &Gl) {
        self.render_tree(gl, PARENT_WINDOW);
        //println!("");
    }

    // this is called at 1200 hz
    pub fn update(self: &mut Self, gl: &Gl, mouse_x: i32, mouse_y: i32, dt: u128) -> bool {

        // lazy initialize
        if !self.initialized { self.initialize(gl); }

        // early exit if it hasn't been 1/30hz since last update time
        self.tick_time += dt;
        let rate = 16000;//33333;
        if rate > self.tick_time { return false; }
        while rate <= self.tick_time { self.tick_time -= rate; }
        
        // process update
        self.mouse_x = mouse_x;
        self.mouse_y = mouse_y;

        // check for mouse hover over any buttons
        self.button_highlight_scan();

        // reconstruct if necessary
        if !self.is_constructed() || self.reconstruct {
            self.construct(gl, PARENT_WINDOW);
            return true;
        }

        false
    }

    fn find_parent_container(self: & Self, idx: usize) -> usize {
        if self.data[idx].vao_owner { return idx; }
        self.find_parent_container(self.data[idx].parent)
    }

    fn is_mouse_inside(self: &Self, idx: usize) -> bool {
        assert!(idx < self.data.len());
        self.data[idx].grid.is_inside(self.mouse_x, self.mouse_y)
    }

    fn is_mouse_inside_tab(self: &Self, idx: usize) -> bool {
        assert!(idx < self.data.len());
        let mut grid = self.data[idx].grid.clone();
        grid.x0 += self.data[idx].tab_offset as f32;
        grid.x1 = grid.x0 + self.get_text_width(&self.data[idx].value) as f32 + 8.0;
        grid.y1 = grid.y0 + 16.0;
        grid.is_inside(self.mouse_x, self.mouse_y)
    }

    fn button_highlight_scan(self: &mut Self) {
        for idx in 0..self.data.len() {
            if WIDGET_ICON == self.data[idx].class {
                let highlighted = self.data[idx].highlight;
                let inside = self.is_mouse_inside(idx);

                if (!inside && highlighted) || (inside && !highlighted) {
                    self.data[idx].highlight = !self.data[idx].highlight;
                    self.force_reconstruct(idx);
                }
            } else if WIDGET_BUTTON == self.data[idx].class {
                let highlighted = self.data[idx].highlight;
                let inside = self.is_mouse_inside(idx);

                if (!inside && highlighted) || (inside && !highlighted) {
                    self.data[idx].highlight = !self.data[idx].highlight;
                    self.force_reconstruct(idx);
                }
            }
        }
    }

    fn click_scan(self: &mut Self) {
        for idx in 0..self.data.len() {

            if !self.data[idx].visible { continue; }
            
            // icon click
            if WIDGET_ICON == self.data[idx].class && self.is_mouse_inside(idx) {
                if self.data[idx].callback.enabled {
                    self.callbacks.push_back(self.data[idx].callback.clone());
                }
            
            // button click
            } else if WIDGET_BUTTON == self.data[idx].class && self.is_mouse_inside(idx) {
                if self.data[idx].enabled && self.data[idx].callback.enabled {
                    self.callbacks.push_back(self.data[idx].callback.clone());
                }
            
            // slider click
            } else if WIDGET_SLIDER == self.data[idx].class && self.is_mouse_inside(idx) && self.data[idx].enabled {
                let lhs = self.data[idx].grid.x0 + 6.0;
                let rhs = self.data[idx].grid.x1 - 6.0;
                let delta = rhs - lhs;
                let m = f32::min(1.0, f32::max(0.0, (self.mouse_x as f32 - lhs) / delta));
                self.data[idx].slider_offset = m;
                self.force_reconstruct(idx);

                if self.data[idx].callback.enabled {
                    self.callbacks.push_back(self.data[idx].callback.clone());
                }
        
            // tree navigation
            } else if WIDGET_TREE == self.data[idx].class && self.is_mouse_inside(idx) {
                self.click_scan_tree(idx, TREE_ROOT);
                self.force_reconstruct(idx);
            
            // tab switching
            } else if WIDGET_TAB == self.data[idx].class && !self.data[idx].highlight && self.is_mouse_inside_tab(idx) {
                // remove highlight from all tabs in group, set this idx to highlighted
                let parent = self.data[idx].parent;
                for i in 0..self.data[parent].children.len() {
                    let child = self.data[parent].children[i];
                    if WIDGET_TAB == self.data[child].class {
                        self.data[child].highlight_on_reconstruct_tab = false;
                        self.force_reconstruct(child);
                    }
                }
                self.data[idx].highlight_on_reconstruct_tab = true;
                self.force_reconstruct(idx);
            }
        }
    }

    fn click_scan_tree(self: &mut Self, tree_idx: usize, node_idx: usize) {

        if TREE_ROOT != node_idx {

            // reset everything to not selected
            self.data[tree_idx].tree_nodes[node_idx].highlighted = false;

            let mut grid = self.data[tree_idx].tree_nodes[node_idx].grid.clone();
            grid.x0 -= 7.0;
            if grid.is_inside(self.mouse_x, self.mouse_y) {
                
                // do we want to open/close?
                let mut open_close = false;
                if !self.data[tree_idx].tree_nodes[node_idx].children.is_empty() {
                    grid.x1 = grid.x0 + 20.0;
                    if grid.is_inside(self.mouse_x, self.mouse_y) { open_close = true; }
                }
                
                if open_close {
                    // open/close event
                    self.data[tree_idx].tree_nodes[node_idx].open = !self.data[tree_idx].tree_nodes[node_idx].open;
                } else {
                    // selection event
                    self.data[tree_idx].tree_nodes[node_idx].highlighted = true;
                }

                // callback
                if self.data[tree_idx].tree_nodes[node_idx].callback.enabled && self.data[tree_idx].tree_nodes[node_idx].highlighted {
                    self.callbacks.push_back(self.data[tree_idx].tree_nodes[node_idx].callback.clone());
                }
            }
        }

        if TREE_ROOT == node_idx || self.data[tree_idx].tree_nodes[node_idx].open {
            for i in 0..self.data[tree_idx].tree_nodes[node_idx].children.len() {
                let idx = self.data[tree_idx].tree_nodes[node_idx].children[i];
                self.click_scan_tree(tree_idx, idx);
            }
        }

    }

    pub fn pop_callback(self: &mut Self) -> Option<Callback> {
        self.callbacks.pop_front()
    }

    pub fn toggle(self: &mut Self, idx: usize, val: bool) {
        assert!(idx < self.data.len());
        let curr = self.data[idx].toggle;
        if curr != val {
            self.data[idx].toggle = val;
            self.force_reconstruct(idx);
        }
    }

    pub fn force_reconstruct(self: &mut Self, idx: usize) {
        assert!(idx < self.data.len());
        let pidx = self.find_parent_container(idx);
        self.data[pidx].reconstruct = true;
        self.reconstruct = true;
    }


}