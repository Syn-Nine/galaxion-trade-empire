#![allow(dead_code)]

use super::*;
use crate::mgfw::log;

struct TilesetBuffer {
    // WARNING: Anything below this line is not in cache!
    image_width: usize,
    image_height: usize,
    tile_width: usize,
    tile_height: usize,
    span: usize,
    count: usize,
}

const MUTATION_NONE: u8 = 0;
const MUTATION_FLIP_HORIZ: u8 = 1;
const MUTATION_FLIP_VERT: u8 = 2;
const MUTATION_ROTATE_CW: u8 = 3;
const MUTATION_ROTATE_CCW: u8 = 4;
const MUTATION_ROTATE_180: u8 = 5;

const TILEMAP_MODE_NORMAL: u8 = 0;
const TILEMAP_MODE_ISOMETRIC: u8 = 1;

struct TileMutation {
    cell: usize,
    mutation: u8,
}

struct TilemapBuffer {
    // WARNING: Anything below this line is not in cache!
    data: Vec<u16>,
    mutations: Vec<TileMutation>,
}

struct TilemapRenderComponentManagerData {
    columns: usize,
    rows: usize,
    constructed: bool,
    reconstruct_needed: bool,
    tileset: usize,
    num_tiles: u16,
    tile_mode: u8,
}

pub struct TilemapRenderComponentManager {
    cache_data: *mut TilemapRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    tileset: std::boxed::Box<Vec<TilesetBuffer>>,
    tilemap: std::boxed::Box<Vec<TilemapBuffer>>,
}

#[allow(dead_code)]
impl TilemapRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> TilemapRenderComponentManager {
        log(format!("Constructing TilemapRenderComponentManager"));

        let mut tileset: Vec<TilesetBuffer> = Vec::new();
        let mut tilemap: Vec<TilemapBuffer> = Vec::new();
        for _i in 0..ENTITY_SZ {
            tileset.push(TilesetBuffer {
                tile_width: 16,
                tile_height: 16,
                image_width: 320,
                image_height: 240,
                span: 16,
                count: 1,
            });
            tilemap.push(TilemapBuffer { data: Vec::new(), mutations: Vec::new() });
        }

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<TilemapRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut TilemapRenderComponentManagerData;

        TilemapRenderComponentManager {
            tileset: Box::new(tileset),
            tilemap: Box::new(tilemap),
            cache_data,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn set_tileset(
        &mut self,
        idx: usize,
        image_width: usize,
        image_height: usize,
        tile_width: usize,
        tile_height: usize,
    ) {
        self.tileset[idx].image_width = image_width;
        self.tileset[idx].image_height = image_height;
        self.tileset[idx].tile_width = tile_width;
        self.tileset[idx].tile_height = tile_height;

        self.tileset[idx].span = (image_width - (image_width % tile_width)) / tile_width;
        self.tileset[idx].count =
            self.tileset[idx].span * (image_height - (image_height % tile_height)) / tile_height;

        println!(
            "tileset {},{},{}",
            idx, self.tileset[idx].span, self.tileset[idx].count
        );
    }

    pub fn set_tilemap(&mut self, idx: usize, tileset_idx: usize, columns: usize, data: &Vec<u16>) {
        let cache_data = self.get_data_ref_mut(idx);
        cache_data.reconstruct_needed = true;
        cache_data.columns = columns;
        cache_data.tileset = tileset_idx;
        cache_data.tile_mode = TILEMAP_MODE_NORMAL;
        let n = data.len();
        assert!(0 != columns);
        assert!(0 != n);
        assert!(0 == n % columns);
        cache_data.rows = (n - (n % columns)) / columns;
        assert!(data.len() == cache_data.rows * cache_data.columns);
        self.tilemap[idx].data = data.clone();
        self.tilemap[idx].mutations.clear();
    }

    pub fn set_iso_tilemap(&mut self, idx: usize, tileset_idx: usize, columns: usize, data: &Vec<u16>) {
        self.set_tilemap(idx, tileset_idx, columns, data);
        self.get_data_ref_mut(idx).tile_mode = TILEMAP_MODE_ISOMETRIC;
    }

    pub fn cell_flip_horizontal(&mut self, idx: usize, cell: usize) {
        self.cell_mutate(idx, cell, MUTATION_FLIP_HORIZ);
    }

    pub fn cell_flip_vertical(&mut self, idx: usize, cell: usize) {
        self.cell_mutate(idx, cell, MUTATION_FLIP_VERT);
    }
    
    pub fn cell_rotate_cw(&mut self, idx: usize, cell: usize) {
        self.cell_mutate(idx, cell, MUTATION_ROTATE_CW);
    }

    pub fn cell_rotate_ccw(&mut self, idx: usize, cell: usize) {
        self.cell_mutate(idx, cell, MUTATION_ROTATE_CCW);
    }

    pub fn cell_rotate_180(&mut self, idx: usize, cell: usize) {
        self.cell_mutate(idx, cell, MUTATION_ROTATE_180);
    }

    fn cell_mutate(&mut self, idx: usize, cell: usize, mutation: u8) {
        let cache_data = self.get_data_ref_mut(idx);
        cache_data.reconstruct_needed = true;
        assert!(cell < cache_data.rows * cache_data.columns);
        cache_data.reconstruct_needed = true;
        self.tilemap[idx].mutations.push(TileMutation { cell, mutation });
    }

    pub fn is_constructed(&self, idx: usize) -> bool {
        self.get_data_ref(idx).constructed
    }

    pub fn reconstruct(&self, idx: usize) -> bool {
        self.get_data_ref(idx).reconstruct_needed
    }

    pub fn get_tileset_idx(&self, idx: usize) -> usize {
        self.get_data_ref(idx).tileset
    }

    pub fn get_num_tiles(&self, idx: usize) -> usize {
        self.get_data_ref(idx).num_tiles as usize
    }

    pub fn construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        let cache_data = self.get_data_ref_mut(idx);
        let cols = cache_data.columns;

        let mut vertex_data: Vec<f32> = Vec::new();

        let map = &self.tilemap[idx].data;

        let mut num_tiles: usize = 0;

        let tileset = &self.tileset[cache_data.tileset];
        let uscale = tileset.tile_width as f32 / tileset.image_width as f32;
        let vscale = tileset.tile_height as f32 / tileset.image_height as f32;
        let usub = uscale * 0.0; //(0.2 / tileset.tile_width as f32);
        let vsub = uscale * 0.0; //(0.2 / tileset.tile_height as f32);

        for i in 0..map.len() {
            let t0 = map[i] as usize;
            if EMPTY_TILE == t0 as u16 || tileset.count < t0 {
                continue;
            }
            let t0 = t0 - 1;

            let u0 = (t0 % tileset.span) as f32 * uscale + usub;
            let v0 = ((t0 - (t0 % tileset.span)) / tileset.span) as f32 * vscale + vsub;
            let u1 = u0 + uscale - usub;
            let v1 = v0 + vscale - vsub;

            let mut x0 = (i % cols) as f32;
            let mut y0 = 1.0 * (((i - (i % cols)) / cols) as f32);
            
            if TILEMAP_MODE_ISOMETRIC == cache_data.tile_mode {
                let xp = x0 - y0;
                let yp = 0.5 * x0 + 0.5 * y0;

                x0 = xp * 0.5 - 0.5;
                y0 = yp * 0.5 - 0.75;
            }

            let x1 = x0 + 1.0;
            let y1 = y0 + 1.0;

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

        // perform mutations
        let mutations = &self.tilemap[idx].mutations;

        for i in 0..mutations.len() {
            let cell = mutations[i].cell;
            let vidx = cell * 24;
            let u0 = vertex_data[vidx + 2];
            let v0 = vertex_data[vidx + 3];
            let u1 = vertex_data[vidx + 10];
            let v1 = vertex_data[vidx + 11];

            match mutations[i].mutation {
                MUTATION_FLIP_HORIZ => {
                    vertex_data[vidx + 2] = u1;
                    vertex_data[vidx + 6] = u1;
                    vertex_data[vidx + 10] = u0;
                    
                    vertex_data[vidx + 14] = u1;
                    vertex_data[vidx + 18] = u0;
                    vertex_data[vidx + 22] = u0;
                },
                MUTATION_ROTATE_CW => {
                    vertex_data[vidx + 2] = u0;
                    vertex_data[vidx + 3] = v1;
                    vertex_data[vidx + 6] = u1;
                    vertex_data[vidx + 7] = v1;
                    vertex_data[vidx + 10] = u1;
                    vertex_data[vidx + 11] = v0;
                    
                    vertex_data[vidx + 14] = u0;
                    vertex_data[vidx + 15] = v1;
                    vertex_data[vidx + 18] = u1;
                    vertex_data[vidx + 19] = v0;
                    vertex_data[vidx + 22] = u0;
                    vertex_data[vidx + 23] = v0;
                },
                MUTATION_ROTATE_180 => {
                    vertex_data[vidx + 2] = u1;
                    vertex_data[vidx + 3] = v1;
                    vertex_data[vidx + 6] = u1;
                    vertex_data[vidx + 7] = v0;
                    vertex_data[vidx + 10] = u0;
                    vertex_data[vidx + 11] = v0;
                    
                    vertex_data[vidx + 14] = u1;
                    vertex_data[vidx + 15] = v1;
                    vertex_data[vidx + 18] = u0;
                    vertex_data[vidx + 19] = v0;
                    vertex_data[vidx + 22] = u0;
                    vertex_data[vidx + 23] = v1;
                },
                _ => (),
            }
        }

        let data_ptr = vertex_data.as_ptr() as *const _;
        gl.buffer_tilemap_data(vao, vbo, num_tiles, data_ptr);

        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
        cache_data.num_tiles = num_tiles as u16;
        //println!("Constructing tilemap {}", idx);
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut TilemapRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &TilemapRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
