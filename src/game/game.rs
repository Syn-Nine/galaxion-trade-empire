use crate::mgfw;
use super::ui;
use super::state;

//use std::fs::File;
//use std::io::{self, BufRead, Write};


pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    pub state_data: state::StateHeap,
    pub battle_data: state::BattleHeap,
}

impl Default for GameDataHeap {
    fn default() -> Self {
        GameDataHeap {
            state_data: state::StateHeap::new(),
            battle_data: state::BattleHeap::new(),
        }
    }
}


// cache data
pub struct GameData {
    // system
    pub heap: *mut GameDataHeap,
    pub frame: u8,
    ready: bool,
    pub quit_requested: bool,
    //
    pub uidata: ui::UIData,
    pub state_cache: state::StateCache,
    pub battle_cache: state::BattleCache,
}


#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("assets/world.dat");

    ui::initialize(cache, heap, world);
    state::initialize(cache, heap, world);

    ui::update_galaxy_zoom(cache, world);
    
}


pub fn quit_requested(cache: &mut GameData) -> bool {
    cache.quit_requested
}

pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();

    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}


// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {
    // pass through to the UI
    ui::event(cache, heap, world, event_id)
}


// this gets called by MGFW at 1200hz
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    
    let mut expect_blown = false;
    cache.frame = (cache.frame + 1) % 128;
    //let dt = 1.0 / 1200.0;

    if !cache.ready {
        if 127 == cache.frame {
            cache.ready = true;
        }
        return false;
    }

    expect_blown |= ui::update(cache, heap, world);

    expect_blown
}


