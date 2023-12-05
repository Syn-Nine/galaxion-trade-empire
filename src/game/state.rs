use crate::game::game::{GameData, GameDataHeap};
use crate::mgfw;
use super::ui;
use super::enums::*;
use std::collections::hash_set::HashSet;
use std::collections::vec_deque::VecDeque;
use rand;
use rand::prelude::*;

use std::fs::File;
use std::io::{self, BufRead};



pub const HOUSE_PLAYER: u8 = 1;
pub const HOUSE_NONE: u8 = 0;
//
pub const STATION_BUYING: u8 = 0;
pub const STATION_SELLING: u8 = 1;
//
pub const BATTLE_TURN_PLAYER: u8 = 0;
pub const BATTLE_TURN_ENEMY: u8 = 1;


#[derive(Default)]
pub struct Planet {
    pub cur_farm: i32,
    pub cur_labor: i32,
    pub cur_sci: i32,
    pub cur_pop: i32,
    pub frac_farm: f32,
    pub frac_labor: f32,
    pub frac_sci: f32,
    //
    pub cur_bios: i32,
    pub cur_fuel: i32,
    pub cur_ice: i32,
    pub cur_mach: i32,
    pub cur_meds: i32,
    pub frac_bios: f32,
    pub frac_fuel: f32,
    pub frac_ice: f32,
    pub frac_mach: f32,
    pub frac_meds: f32,
    //
    pub mfg_level_bios: u8,
    pub mfg_level_fuel: u8,
    pub mfg_level_ice: u8,
    pub mfg_level_mach: u8,
    pub mfg_level_meds: u8,
    //
    pub sci_level: u8,
    pub sci_researching: u8,
    pub sci_research_percent: f32,
    //
    pub food_rate: u8,
    //
    pub researching: u8,
    pub research_status: Vec<u8>,
    //
    pub fighter_rate: u8,
    pub fighters: i32,
    pub fighters_rem: f32,
}

impl Planet {
    fn new(planet_class: u8, rng: &mut ThreadRng) -> Planet {

        let mfg_level_bios;
        let mfg_level_fuel;
        let mfg_level_ice;
        let mfg_level_mach;
        let mfg_level_meds;

        let cur_bios;
        let cur_fuel;
        let cur_ice;
        let cur_mach;
        let cur_meds;

        let mut research_status: Vec<u8> = vec![RESEARCH_NOT_STARTED; RESEARCH_COUNT];
        research_status[RESEARCH_AGRICULTURE_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_BIOLOGY_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_ENERGY_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_KNOWLEDGE_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_MEDICINE_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_MILITARY_II as usize] = RESEARCH_NOT_AVAILABLE;
        research_status[RESEARCH_TECHNOLOGY_II as usize] = RESEARCH_NOT_AVAILABLE;

        match planet_class {
            PLANET_CLASS_LUSH => {
                mfg_level_bios = (10 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_fuel = (3 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_ice = (5 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_mach = (3 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_meds = (5 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                //
                cur_bios = 100 + (rng.gen::<f32>() * 20.0 - 10.0) as i32;
                cur_fuel = 20 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_ice = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_mach = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_meds = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
            }
            PLANET_CLASS_WATER => {
                mfg_level_bios = (5 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_fuel = (3 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_ice = (10 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_mach = (3 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_meds = (4 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                //
                cur_bios = 50 + (rng.gen::<f32>() * 10.0 - 5.0) as i32;
                cur_fuel = 20 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_ice = 100 + (rng.gen::<f32>() * 20.0 - 10.0) as i32;
                cur_mach = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_meds = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
            }
            PLANET_CLASS_DESERT => {
                mfg_level_bios = 2;
                mfg_level_fuel = (15 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_ice = 1;
                mfg_level_mach = (5 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_meds = 1;
                //
                cur_bios = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_fuel = 1000 + (rng.gen::<f32>() * 200.0 - 100.0) as i32;
                cur_ice = 0;
                cur_mach = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_meds = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
            }
            PLANET_CLASS_VOLCANO => {
                mfg_level_bios = 1;
                mfg_level_fuel = (8 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_ice = 1;
                mfg_level_mach = (15 + (rng.gen::<f32>() * 2.0 - 1.0) as i8) as u8;
                mfg_level_meds = 1;
                //
                cur_bios = 0;
                cur_fuel = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_ice = 0;
                cur_mach = 10 + (rng.gen::<f32>() * 5.0 - 2.0) as i32;
                cur_meds = 0;
            }
            _ => {
                mfg_level_bios = 1;
                mfg_level_fuel = 1;
                mfg_level_ice = 1;
                mfg_level_mach = 1;
                mfg_level_meds = 1;
                //
                cur_bios = 0;
                cur_fuel = 0;
                cur_ice = 0;
                cur_mach = 0;
                cur_meds = 0;
            }
        }

        Planet {
            // set initial planet states
            cur_farm: 800,
            cur_labor: 150,
            cur_sci: 50,
            cur_pop: 1000,

            frac_farm: 0.98,
            frac_labor: 0.01,
            frac_sci: 0.01,

            cur_bios,
            cur_fuel,
            cur_ice,
            cur_mach,
            cur_meds,

            frac_bios: 0.2,
            frac_fuel: 0.2,
            frac_ice: 0.2,
            frac_mach: 0.2,
            frac_meds: 0.2,

            mfg_level_bios,
            mfg_level_fuel,
            mfg_level_ice,
            mfg_level_mach,
            mfg_level_meds,

            sci_level: 20,
            sci_researching: 0,
            sci_research_percent: 0.0,

            food_rate: 3,
            
            fighter_rate: 1,
            fighters: 0,
            fighters_rem: 0.0,

            research_status,
            
            ..Default::default()
        }
    }
}

pub struct Station {
    pub nav_init: u8,
    pub nav_local: bool,
    pub nav_path_0: bool,
    pub nav_path_0_idx: usize,
    pub nav_path_1: bool,
    pub nav_path_1_idx: usize,
    //
    pub buysell_bios: u8,
    pub buysell_fuel: u8,
    pub buysell_ice: u8,
    pub buysell_mach: u8,
    pub buysell_meds: u8,
    //
    pub trade_price_bios: i32,
    pub trade_price_fuel: i32,
    pub trade_price_ice: i32,
    pub trade_price_mach: i32,
    pub trade_price_meds: i32,
    //
    pub merc_price: i32,
    pub merc_available: u8,
}

pub struct Sector {
    pub x: f32,
    pub y: f32,
    pub planet_class: u8,
    pub station_class: u8,
    pub known: bool,
    pub planet_data: Planet,
    pub station_data: Station,
    pub house: u8,
    pub enemy_aggressive: bool,
    pub tribute_multiplier: f32,
}

pub struct StateCache {
    pub location: u8,
    pub year: i32,
    pub trade_qty_bios: i32,
    pub trade_qty_fuel: i32,
    pub trade_qty_ice: i32,
    pub trade_qty_mach: i32,
    pub trade_qty_meds: i32,
    pub merc_to_buy: i32,
    pub cash: i32,
    num_planets: u8,
}

#[derive(Default)]
pub struct Ship {
    pub location: u8,
    pub hold_colonists: i32,
    pub hold_bios: i32,
    pub hold_fuel: i32,
    pub hold_ice: i32,
    pub hold_mach: i32,
    pub hold_meds: i32,
    pub holds: i32,
    pub fighters: i32,
    //
    pub effective_price_bios: f32,
    pub effective_price_fuel: f32,
    pub effective_price_ice: f32,
    pub effective_price_mach: f32,
    pub effective_price_meds: f32,
    //
    pub nav_deque: VecDeque<usize>,
    pub animating: bool,
    pub animating_ratio: f32,
}
pub struct StateHeap {
    pub sectors: Vec<Sector>,
    jumps: Vec<(usize, usize)>,
    pub ships: Vec<Ship>,
    pub house_names: Vec<String>,
    pub house_colors: Vec<mgfw::ecs::Color>,
    pub planet_names: Vec<String>,
    pub house_avatars: Vec<String>,
    pub dead_houses: Vec<u8>,
    pub last_house: u8,
}

impl StateHeap {
    pub fn new() -> StateHeap {
        StateHeap {
            sectors: Vec::new(),
            jumps: Vec::new(),
            ships: Vec::new(),
            house_names: Vec::new(),
            planet_names: Vec::new(),
            house_avatars: Vec::new(),
            house_colors: Vec::new(),
            dead_houses: Vec::new(),
            last_house: 0,
        }
    }
}

#[derive(Default)]
pub struct BattleUnit {
    pub x: u8,
    pub y: u8,
    pub class: u8,
    pub shields: i32,
    pub animating: bool,
}
pub struct BattleCache {
    pub player_units: [BattleUnit; 16],
    pub player_units_count: u8,
    pub enemy_units: [BattleUnit; 16],
    pub selected_unit: u8,
    pub selected_unit_flash: u8,
    pub selected_unit_flash_visibile: bool,
    pub explosion_counter: u8,
    pub explosion_screen_location: (i32, i32),
    pub explosion_frame: u8,
    pub turn_order: [u8; 32],
    pub turn_order_index: u8,
    pub turn_count: u8,
    pub whos_turn_is_it: u8,
    pub auto_battle: bool,
    pub trigger_enemy_surrender: bool,
    pub trigger_player_surrender: bool,
    pub tribute_multiplier: f32,
    //
    pub enemy_tribute_cash: i32,
    pub enemy_tribute_colonists: i32,
    pub enemy_tribute_bios: i32,
    pub enemy_tribute_fuel: i32,
    pub enemy_tribute_ice: i32,
    pub enemy_tribute_mach: i32,
    pub enemy_tribute_meds: i32,
    pub enemy_tribute_fighters: i32,

}


pub struct BattleHeap {
    pub battle_map_to_screen_lookup: Vec<(i32, i32)>,
    pub battle_map_move_options: Vec<usize>,
    pub battle_map_attack_options: Vec<usize>,
}

impl BattleHeap {
    pub fn new() -> BattleHeap {
        BattleHeap {
            battle_map_to_screen_lookup: Vec::new(),
            battle_map_move_options: Vec::new(),
            battle_map_attack_options: Vec::new(),
        }
    }
}

pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    println!("Initializing Game State");
    heap.state_data.house_names = generate_names(world, 255);
    heap.state_data.planet_names = generate_planet_names(world, 200);
    heap.state_data.house_avatars = generate_avatar_strings(world);
    heap.state_data.house_colors = generate_house_colors(world, 200);

    heap.state_data.house_names[HOUSE_PLAYER as usize] = String::from("Player1");
    heap.state_data.house_avatars[HOUSE_PLAYER as usize] = String::from("06_sm.png");

    gen_galaxy(heap, world);
    println!("Generated {} Sectors", heap.state_data.sectors.len());

    for i in 0..heap.state_data.sectors.len() {
        if heap.state_data.sectors[i].planet_class != PLANET_CLASS_INVALID {
            cache.state_cache.num_planets += 1;
        }
    }

    // find starting point
    // find all green planets
    let mut greens: Vec<usize> = Vec::new();
    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_LUSH == heap.state_data.sectors[i].planet_class {
            greens.push(i);
        }
    }

    // no station
    let mut nostat: Vec<usize> = Vec::new();
    for i in 0..greens.len() {
        if STATION_CLASS_INVALID == heap.state_data.sectors[greens[i]].station_class {
            nostat.push(greens[i]);
        }
    }

    // nearest station
    let mut near: Vec<(usize, usize, i32)> = Vec::new();
    let mut idx3 = -1;
    let mut idx4 = 0;
    for i in 0..nostat.len() {
        let (idx, dist) = find_nearest_station(heap, nostat[i]);
        println!("{} is near station {}, d={}", nostat[i], idx, dist);
        near.push((nostat[i], idx, dist));
        if 3 == dist {
            idx3 = nostat[i] as i32;
            idx4 = idx;
            break;
        }
    }

    // furthest first station from near set, or first to dist = 3
    if -1 == idx3 {
        let mut maxdist = 0;
        for i in 0..near.len() {
            if near[i].2 > maxdist {
                maxdist = near[i].2;
                idx3 = near[i].0 as i32;
                idx4 = near[i].1;
            }
        }
    }
    if -1 == idx3 {
        idx3 = greens[0] as i32;
        println!("Failed to find valid home base, forcing to: {idx3}");
        heap.state_data.sectors[idx3 as usize].station_class = STATION_CLASS_INVALID;
        let (idx, dist) = find_nearest_station(heap, idx3 as usize);
        idx4 = idx;
    }

    let home = idx3 as usize;
    let station = idx4;
    
    println!("Selected home base: {home}, station: {station}");

    cache.state_cache.location = home as u8;
    cache.uidata.curr_sector = cache.state_cache.location;
    heap.state_data.sectors[home].house = HOUSE_PLAYER; // player house

    let s = Ship { location: home as u8, holds: 200, fighters: 12, ..Default::default() };
    heap.state_data.ships.push(s);

    // find path to station
    learn_path(heap, home, station, true);
    update_galaxy_geometry(cache, heap, world);

    // update camera for station location
    cache.uidata.zoom_cam_x = heap.state_data.sectors[home].x;
    cache.uidata.zoom_cam_y = heap.state_data.sectors[home].y;
    cache.uidata.zoom_level = 8;

    //
    cache.state_cache.year = 0;
    cache.state_cache.cash = 1000;
    

}


pub fn claim_planet(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    // set pop based on player's other planets
    let pplanets = get_player_planet_vec(heap);
    if pplanets.is_empty() { return; }

    let mut avg_pop = 0;
    for i in 0..pplanets.len() {
        avg_pop += heap.state_data.sectors[pplanets[i]].planet_data.cur_pop;
    }

    let avg_pop: f32 = avg_pop as f32 / pplanets.len() as f32;
    let pop = (0.8 + world.rnd() * 0.4) * avg_pop;
    
    let sidx = cache.state_cache.location as usize;
    heap.state_data.sectors[sidx].house = HOUSE_PLAYER;

    let cur_farm = i32::max(0, (pop * heap.state_data.sectors[sidx].planet_data.frac_farm).floor() as i32);
    let cur_labor = i32::max(0, (pop * heap.state_data.sectors[sidx].planet_data.frac_labor).floor() as i32);
    let cur_sci = i32::max(0, (pop * heap.state_data.sectors[sidx].planet_data.frac_sci).floor() as i32);
    let cur_pop = cur_farm + cur_labor + cur_sci;

    heap.state_data.sectors[sidx].planet_data.cur_farm = cur_farm;
    heap.state_data.sectors[sidx].planet_data.cur_labor = cur_labor;
    heap.state_data.sectors[sidx].planet_data.cur_sci = cur_sci;
    heap.state_data.sectors[sidx].planet_data.cur_pop = cur_pop;

    let frac_bios = world.rnd_range(1..10) as f32;
    let frac_fuel = world.rnd_range(1..10) as f32;
    let frac_ice = world.rnd_range(1..10) as f32;
    let frac_mach = world.rnd_range(1..10) as f32;
    let frac_meds = world.rnd_range(1..10) as f32;

    let tot = frac_bios + frac_fuel + frac_ice + frac_mach + frac_meds;
    heap.state_data.sectors[sidx].planet_data.frac_bios = frac_bios / tot;
    heap.state_data.sectors[sidx].planet_data.frac_fuel = frac_fuel / tot;
    heap.state_data.sectors[sidx].planet_data.frac_ice = frac_ice / tot;
    heap.state_data.sectors[sidx].planet_data.frac_mach = frac_mach / tot;
    heap.state_data.sectors[sidx].planet_data.frac_meds = frac_meds / tot;

    // set research based on qty of planets
    let nplanets = pplanets.len();

    for _ in 0..nplanets {
        if world.rnd() < 0.5 {
            // find first non-researched item
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.food_rate += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.mfg_level_bios += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.mfg_level_fuel += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.sci_level += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.mfg_level_meds += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.fighter_rate += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_I as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_I as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_II as usize] = RESEARCH_NOT_STARTED;
                heap.state_data.sectors[sidx].planet_data.mfg_level_mach += 1;
                continue;
            }
            //
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.food_rate += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.mfg_level_bios += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.mfg_level_fuel += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.sci_level += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.mfg_level_meds += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.fighter_rate += 1;
                continue;
            }
            if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_II as usize] {
                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_II as usize] = RESEARCH_COMPLETE;
                heap.state_data.sectors[sidx].planet_data.mfg_level_mach += 1;
                continue;
            }
            break;
        }
    }
    
}
    

pub fn get_player_planet_vec(heap: & GameDataHeap) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class &&
            HOUSE_PLAYER == heap.state_data.sectors[i].house {
            ret.push(i);
        }
    }
    ret
}


pub fn get_known_station_vec(heap: & GameDataHeap) -> Vec<usize> {
    let mut ret: Vec<usize> = Vec::new();
    for i in 0..heap.state_data.sectors.len() {
        if STATION_CLASS_INVALID != heap.state_data.sectors[i].station_class &&
            heap.state_data.sectors[i].known {
            ret.push(i);
        }
    }
    ret
}


pub fn find_local_sectors(heap: &mut GameDataHeap, start: usize) {

    let aconn = heap.state_data.jumps.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();
    for _ in 0..=heap.state_data.sectors.len() {
        visited.push(false);
        dist.push(0);
    }

    let mut sptr = 0;
    stack.push(start);
    visited[start] = true;
    dist[start] = 0;
    
    println!("Finding local sectors");
    loop {
        let i0 = stack[sptr];
        
        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == i0 {
                if !visited[idx1] {
                    visited[idx1] = true;
                    dist[idx1] = dist[i0] + 1;
                    if STATION_CLASS_INVALID == heap.state_data.sectors[idx1].station_class {
                        stack.push(idx1);
                    }
                }
            } else if idx1 == i0 {
                if !visited[idx0] {
                    visited[idx0] = true;
                    dist[idx0] = dist[i0] + 1;
                    if STATION_CLASS_INVALID == heap.state_data.sectors[idx0].station_class {
                        stack.push(idx0);
                    }
                }
            }
        }
        sptr += 1;
        if sptr == stack.len() { break; }
    
    }

    // for all sectors with distance < 4, mark as known
    for i in 0..dist.len() {
        if visited[i] && dist[i] > 0 && dist[i] < 4 && !heap.state_data.sectors[i].known {
            heap.state_data.sectors[i].known = true;
        }
    }

}

pub fn find_local_stations(heap: &mut GameDataHeap, start: usize) -> Vec<usize> {

    let mut ret: Vec<usize> = Vec::new();
    let aconn = heap.state_data.jumps.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();
    for _ in 0..=heap.state_data.sectors.len() {
        visited.push(false);
        dist.push(0);
    }

    let mut sptr = 0;
    stack.push(start);
    visited[start] = true;
    dist[start] = 0;
    
    println!("Finding local stations");
    loop {
        let i0 = stack[sptr];
        
        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == i0 {
                if !visited[idx1] {
                    visited[idx1] = true;
                    dist[idx1] = dist[i0] + 1;

                    if STATION_CLASS_INVALID != heap.state_data.sectors[idx1].station_class && !heap.state_data.sectors[idx1].known {
                        ret.push(idx1);
                        if 2 == ret.len() { return ret; }
                    } else {
                        if dist[i0] < 5 {
                            stack.push(idx1);
                        }
                    }
                }
            } else if idx1 == i0 {
                if !visited[idx0] {
                    visited[idx0] = true;
                    dist[idx0] = dist[i0] + 1;

                    if STATION_CLASS_INVALID != heap.state_data.sectors[idx0].station_class && !heap.state_data.sectors[idx1].known {
                        ret.push(idx0);
                        if 2 == ret.len() { return ret; }
                    } else {
                        if dist[i0] < 5 {
                            stack.push(idx0);
                        }
                    }
                }
            }
        }
        sptr += 1;
        if sptr == stack.len() { break; }
    
    }

    ret

}

fn find_nearest_station(heap: &mut GameDataHeap, start: usize) -> (usize, i32) {

    let aconn = heap.state_data.jumps.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();
    for _ in 0..=heap.state_data.sectors.len() {
        visited.push(false);
        dist.push(0);
    }

    let mut sptr = 0;
    stack.push(start);
    visited[start] = true;
    dist[start] = 0;
    
    println!("Finding nearest station");
    loop {
        let i0 = stack[sptr];
        
        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == i0 {
                if !visited[idx1] {
                    visited[idx1] = true;
                    dist[idx1] = dist[i0] + 1;
                    stack.push(idx1);

                    if STATION_CLASS_INVALID != heap.state_data.sectors[idx1].station_class {
                        return (idx1, dist[idx1]);
                    }
                }
            } else if idx1 == i0 {
                if !visited[idx0] {
                    visited[idx0] = true;
                    dist[idx0] = dist[i0] + 1;
                    stack.push(idx0);

                    if STATION_CLASS_INVALID != heap.state_data.sectors[idx0].station_class {
                        return (idx0, dist[idx0]);
                    }
                }
            }
        }
        sptr += 1;
        if sptr == stack.len() { break; }
    
    }

    (start, 0)
}

fn find_nearest_enemy_planet(heap: &mut GameDataHeap, start: usize, house: u8) -> i32 {

    let aconn = heap.state_data.jumps.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut dist: Vec<i32> = Vec::new();
    for _ in 0..=heap.state_data.sectors.len() {
        visited.push(false);
        dist.push(0);
    }

    let mut sptr = 0;
    stack.push(start);
    visited[start] = true;
    dist[start] = 0;
    
    loop {
        let i0 = stack[sptr];
        
        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == i0 {
                if !visited[idx1] {
                    visited[idx1] = true;
                    dist[idx1] = dist[i0] + 1;
                    stack.push(idx1);

                    if PLANET_CLASS_INVALID != heap.state_data.sectors[idx1].planet_class && heap.state_data.sectors[idx1].house != house {
                        return idx1 as i32;
                    }
                }
            } else if idx1 == i0 {
                if !visited[idx0] {
                    visited[idx0] = true;
                    dist[idx0] = dist[i0] + 1;
                    stack.push(idx0);

                    if PLANET_CLASS_INVALID != heap.state_data.sectors[idx0].planet_class && heap.state_data.sectors[idx0].house != house {
                        return idx0 as i32;
                    }
                }
            }
        }
        sptr += 1;
        if sptr == stack.len() { break; }
    
    }

    -1
}

pub fn learn_path(heap: &mut GameDataHeap, start_idx: usize, end_idx: usize, anypath: bool) {
    let path = find_path(heap, start_idx, end_idx, anypath);
    assert!(!path.is_empty());

    for i in 0..path.len() {
        heap.state_data.sectors[path[i]].known = true;
    }
}

pub fn find_path(heap: &mut GameDataHeap, start_idx: usize, end_idx: usize, anypath: bool) -> Vec<usize> {
    
    let aconn = heap.state_data.jumps.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();
    let mut dist: Vec<f32> = Vec::new();
    for _ in 0..=heap.state_data.sectors.len() {
        visited.push(false);
        dist.push(0.0);
    }

    let mut sptr = 0;
    stack.push(start_idx);
    visited[start_idx] = true;
    dist[start_idx] = 1.0;
    let eps = 1.0e-6;

    println!("Finding path");
    loop {
        let i0 = stack[sptr];
        if i0 == end_idx { break; }

        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == i0 && (heap.state_data.sectors[idx1].known || anypath){
                if !visited[idx1] {
                    visited[idx1] = true;
                    let dx = heap.state_data.sectors[idx1].x - heap.state_data.sectors[end_idx].x;
                    let dy = heap.state_data.sectors[idx1].y - heap.state_data.sectors[end_idx].y;
                    let m = (dx * dx + dy * dy).sqrt() / 960.0;
                    dist[idx1] = dist[i0] + 1.0 + m;
                    stack.push(idx1);
                }
            } else if idx1 == i0 && (heap.state_data.sectors[idx0].known || anypath) {
                if !visited[idx0] {
                    visited[idx0] = true;
                    let dx = heap.state_data.sectors[idx0].x - heap.state_data.sectors[end_idx].x;
                    let dy = heap.state_data.sectors[idx0].y - heap.state_data.sectors[end_idx].y;
                    let m = (dx * dx + dy * dy).sqrt() / 960.0;
                    dist[idx0] = dist[i0] + 1.0 + m;
                    stack.push(idx0);
                }
            }
        }
        sptr += 1;
        if sptr == stack.len() { break; }
    }

    if dist[end_idx] < eps { println!("No valid path"); return Vec::new(); }

    let mut path: Vec<usize> = Vec::new();
    path.push(end_idx);

    let mut mindist = dist[end_idx];
    let mut minidx = end_idx;
    let mut cur_idx = end_idx;

    println!("Walking backward from end");
    loop {
        for i in 0..aconn.len() {
            let idx0 = aconn[i].0;
            let idx1 = aconn[i].1;
            if idx0 == cur_idx {
                if eps < dist[idx1] && dist[idx1] < mindist {
                    mindist = dist[idx1];
                    minidx = idx1;
                }
            } else if idx1 == cur_idx {
                if eps < dist[idx0] && dist[idx0] < mindist {
                    mindist = dist[idx0];
                    minidx = idx0;
                }
            }
        }

        path.push(minidx);
        cur_idx = minidx;
        if start_idx == cur_idx { break; }
    }

    path
}


pub fn get_hold_availibility(heap: &GameDataHeap) -> f32 {
    let hold_colonists = heap.state_data.ships[0].hold_colonists;
    let hold_bios = heap.state_data.ships[0].hold_bios;
    let hold_fuel = heap.state_data.ships[0].hold_fuel;
    let hold_ice = heap.state_data.ships[0].hold_ice;
    let hold_mach = heap.state_data.ships[0].hold_mach;
    let hold_meds = heap.state_data.ships[0].hold_meds;
    //
    let ratio_colonists = 0.005;
    let ratio_bios = 0.003;
    let ratio_fuel = 0.002;
    let ratio_ice = 0.004;
    let ratio_mach = 0.003;
    let ratio_meds = 0.001;

    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

    hold_tot

}

pub fn get_num_planets(cache: &GameData) -> usize {
    cache.state_cache.num_planets as usize
}

pub fn get_num_planets_house(heap: &GameDataHeap, house: u8) -> usize {
    let mut pplanets = 0;
    for i in 0..heap.state_data.sectors.len() {
        if heap.state_data.sectors[i].planet_class != PLANET_CLASS_INVALID {
            if house == heap.state_data.sectors[i].house {
                pplanets += 1;
            }
        }
    }
    pplanets
}

pub fn get_num_planets_player(heap: &GameDataHeap) -> usize {
    get_num_planets_house(heap, HOUSE_PLAYER)
}

pub fn get_num_territories_house(heap: &GameDataHeap, house: u8) -> usize {
    let mut nterr = 0;
    for i in 0..heap.state_data.sectors.len() {
        if heap.state_data.sectors[i].known && heap.state_data.sectors[i].house == house {
            nterr += 1;
        }
    }
    nterr
}

fn gen_galaxy(heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    println!("Generating Galaxy");
    let mut loc: Vec<(f32, f32)> = Vec::new();

    let w = 40.0;
    let w2 = w * 0.7;
    let w4 = w * 0.15;
    let cx = 960.0 / 2.0;
    let cy = 540.0 / 2.0;

    let mut house_counter = 1;

    let mut rng: ThreadRng = rand::thread_rng();

    for y in 0..14 {
        for x in 0..24 {
            let xx = (x as f32 * w + w4 + world.rnd() * w2).floor() + 0.5;
            let yy = (y as f32 * w + w4 + world.rnd() * w2).floor() + 0.5;;
            let d = ((cx - xx) * (cx - xx) + (cy - yy) * (cy - yy) * 1.7 * 1.7).sqrt();
            if d < 350.0 && d > 100.0 {
                loc.push((xx, yy));

                let mut pc = PLANET_CLASS_INVALID;
                if  world.rnd() < 0.5 {
                    pc = world.rnd_range(1..5) as u8;
                }
                let mut sc = STATION_CLASS_INVALID;
                if world.rnd() < 0.25 {
                    sc = STATION_CLASS_PORT;
                }

                let mut v = vec![0, 0, 1, 1];
                v.push(world.rnd().round() as u8);
                for _ in 0..20 {
                    let i0 = world.rnd_range(0..5);
                    let i1 = world.rnd_range(0..5);
                    let temp = v[i0];
                    v[i0] = v[i1];
                    v[i1] = temp;
                }

                let aggro = if world.rnd() < 0.25 || PLANET_CLASS_VOLCANO == pc { true } else { false };

                let house = if pc != PLANET_CLASS_INVALID {
                    house_counter += 1;
                    house_counter
                } else {
                    HOUSE_NONE
                };

                let s = Sector {
                    x: xx,
                    y: yy,
                    planet_class: pc,
                    station_class: sc,
                    known: false,
                    planet_data: Planet::new(PLANET_CLASS_LUSH, &mut rng),
                    house: house,
                    tribute_multiplier: 1.0,
                    enemy_aggressive: aggro,
                    station_data: Station {
                        nav_init: 0,
                        nav_local: false,
                        nav_path_0: false,
                        nav_path_0_idx: 0,
                        nav_path_1: false,
                        nav_path_1_idx: 0,
                        //
                        buysell_bios: v[0],
                        buysell_fuel: v[1],
                        buysell_ice: v[2],
                        buysell_mach: v[3],
                        buysell_meds: v[4],
                        //
                        trade_price_bios: (10.0 * (0.8 + 0.2 * world.rnd())).round() as i32,
                        trade_price_fuel: (20.0 * (0.8 + 0.2 * world.rnd())).round() as i32,
                        trade_price_ice: (30.0 * (0.8 + 0.2 * world.rnd())).round() as i32,
                        trade_price_mach: (40.0 * (0.8 + 0.2 * world.rnd())).round() as i32,
                        trade_price_meds: (50.0 * (0.8 + 0.2 * world.rnd())).round() as i32,
                        //
                        merc_price: ((0.9 + 0.3 * world.rnd()) * 1000.0).round() as i32,
                        merc_available: (world.rnd() * 3.0).ceil() as u8,
                    }
                };
                
                heap.state_data.sectors.push(s);
            }
        }
    }

    heap.state_data.last_house = house_counter;

    // find federation stations
    let nx = 500.0;
    let ny = 370.0;
    let mut nearidx = 0;
    let mut neardist = 1.0e+9;
    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class { continue; }
        
        let dx = heap.state_data.sectors[i].x - nx;
        let dy = heap.state_data.sectors[i].y - ny;
        let d = dx * dx + dy * dy;
        if d < neardist {
            neardist = d;
            nearidx = i;
        }
    }
    heap.state_data.sectors[nearidx].station_class = STATION_CLASS_FEDERAL;

    let nx = 312.0;
    let ny = 212.0;
    let mut nearidx = 0;
    let mut neardist = 1.0e+9;
    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class { continue; }
        
        let dx = heap.state_data.sectors[i].x - nx;
        let dy = heap.state_data.sectors[i].y - ny;
        let d = dx * dx + dy * dy;
        if d < neardist {
            neardist = d;
            nearidx = i;
        }
    }
    heap.state_data.sectors[nearidx].station_class = STATION_CLASS_FEDERAL;

    let nx = 600.0;
    let ny = 190.0;
    let mut nearidx = 0;
    let mut neardist = 1.0e+9;
    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class { continue; }
        
        let dx = heap.state_data.sectors[i].x - nx;
        let dy = heap.state_data.sectors[i].y - ny;
        let d = dx * dx + dy * dy;
        if d < neardist {
            neardist = d;
            nearidx = i;
        }
    }
    heap.state_data.sectors[nearidx].station_class = STATION_CLASS_FEDERAL;


    let mut conn: Vec<(usize, usize)> = Vec::new();

    for i in 0..loc.len() {
        for j in (i+1)..loc.len() {
            let dx = (loc[i].0 - loc[j].0).abs();
            let dy = (loc[i].1 - loc[j].1).abs();
            if dx < 50.0 && dy < 50.0 {
                conn.push((i, j));
            }
        }
    }

    // shuffle conn list
    for _ in 0..100000 {
        let i0 = world.rnd_range(0..conn.len());
        let i1 = world.rnd_range(0..conn.len());
        let temp = conn[i0].clone();
        conn[i0] = conn[i1].clone();
        conn[i1] = temp;
    }

    // starting idx
    let mut visited: HashSet<usize> = HashSet::new();
    let idx = conn[world.rnd_range(0..conn.len())].0;
    visited.insert(idx);
    let mut stack: Vec<usize> = Vec::new();
    stack.push(idx);
    let mut aconn: Vec<(usize, usize)> = Vec::new();

    let mut sptr = 0;

    println!("Updating Jumps");
    loop {
        let idx = stack[sptr];
        // find all neighbors
        for i in 0..conn.len() {
            let i0 = conn[i].0;
            let i1 = conn[i].1;
            if i0 == idx {
                // potential neighbor
                // check if i1 is already visited
                if !visited.contains(&i1) {
                    visited.insert(i1);
                    stack.push(i1);
                    aconn.push(conn[i].clone());
                } else {
                    if world.rnd() < 0.05 {
                        aconn.push(conn[i].clone());
                    }
                }

            } else if i1 == idx {
                // potential neighbor
                if !visited.contains(&i0) {
                    visited.insert(i0);
                    stack.push(i0);
                    aconn.push(conn[i].clone());
                } else {
                    if world.rnd() < 0.05 {
                        aconn.push(conn[i].clone());
                    }
                }
            }
        }

        sptr += 1;
        if sptr == stack.len() { break; }
    }

    heap.state_data.jumps = aconn.clone();

    
}


pub fn update_galaxy_geometry(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    
    let cw = mgfw::ecs::Color::new(1.0, 1.0, 1.0, 0.7);
    let cb = mgfw::ecs::Color::new(0.1, 0.0, 0.2, 0.7);
    let c1 = mgfw::ecs::Color::new(0.5, 0.0, 1.0, 0.7);
    let cd = mgfw::ecs::Color::new(0.2, 0.0, 0.4, 0.7);

    let aconn = heap.state_data.jumps.clone();


    // figure out uninhabbited territories
    let mut inv: Vec<usize> = Vec::new();

    for i in 0..heap.state_data.sectors.len() {
        if PLANET_CLASS_INVALID == heap.state_data.sectors[i].planet_class {
            heap.state_data.sectors[i].house = HOUSE_NONE;
            inv.push(i);
        }
    }

    // scan connections to determine house
    // todo make this more efficient
    let mut keep_going;
    loop {
        keep_going = false;
        for i in 0..inv.len() {
            if heap.state_data.sectors[inv[i]].house == HOUSE_NONE {
                for j in 0..aconn.len() {
                    if aconn[j].0 == inv[i] && heap.state_data.sectors[aconn[j].1].house != HOUSE_NONE {
                        heap.state_data.sectors[inv[i]].house = heap.state_data.sectors[aconn[j].1].house;
                        keep_going = true;
                    }
                    if aconn[j].1 == inv[i] && heap.state_data.sectors[aconn[j].0].house != HOUSE_NONE {
                        heap.state_data.sectors[inv[i]].house = heap.state_data.sectors[aconn[j].0].house;
                        keep_going = true;
                    }
                }
            }
        }
        if !keep_going { break; }
    }

    
    ////////// LINES
    
    let mut pdata: Vec<mgfw::ecs::Position> = Vec::new();
    let mut cdata: Vec<mgfw::ecs::Color> = Vec::new();

    // hack to show everything
    /*for i in 0..heap.state_data.sectors.len() {
        heap.state_data.sectors[i].known = true;
    }*/
    
    for i in 0..aconn.len() {
        if heap.state_data.sectors[aconn[i].0].known && heap.state_data.sectors[aconn[i].1].known {
            let x0 = heap.state_data.sectors[aconn[i].0].x;
            let y0 = heap.state_data.sectors[aconn[i].0].y;
            let x1 = heap.state_data.sectors[aconn[i].1].x;
            let y1 = heap.state_data.sectors[aconn[i].1].y;
            for k in 1..3 {
                ui::add_connection(&mut pdata, &mut cdata, x0, y0 + 0.2 * k as f32, x1, y1 + 0.2 * k as f32, &cb);
            }
            ui::add_connection(&mut pdata, &mut cdata, x0, y0, x1, y1, &cd);
        }
    }

    ////////// POLYGONS
    
    let mut tpdata: Vec<mgfw::ecs::Position> = Vec::new();
    let mut tcdata: Vec<mgfw::ecs::Color> = Vec::new();

    let aconn = heap.state_data.jumps.clone();
    
    for i in 0..aconn.len() {
        if heap.state_data.sectors[aconn[i].0].known && heap.state_data.sectors[aconn[i].1].known {
            let x0 = heap.state_data.sectors[aconn[i].0].x;
            let y0 = heap.state_data.sectors[aconn[i].0].y;
            let x1 = heap.state_data.sectors[aconn[i].1].x;
            let y1 = heap.state_data.sectors[aconn[i].1].y;
            
            if heap.state_data.sectors[aconn[i].0].house == heap.state_data.sectors[aconn[i].1].house {
                //PLANET_CLASS_INVALID != heap.state_data.sectors[aconn[i].0].planet_class {
                if HOUSE_PLAYER == heap.state_data.sectors[aconn[i].0].house {
                    ui::add_poly_connection(&mut tpdata, &mut tcdata, x0, y0, x1, y1, &c1, 2.0, 5.0);
                    ui::add_connection(&mut pdata, &mut cdata, x0, y0, x1, y1, &cw);
                } else {
                    ui::add_poly_connection(&mut tpdata, &mut tcdata, x0, y0, x1, y1, &cd, 2.0, 5.0);
                    let cc = heap.state_data.house_colors[heap.state_data.sectors[aconn[i].0].house as usize].clone();
                    ui::add_connection(&mut pdata, &mut cdata, x0, y0, x1, y1, &cc);
                }
            }
        }
    }

    ///////////// MARKERS

    for i in 0..heap.state_data.sectors.len() {
        if heap.state_data.sectors[i].known {
            ui::add_marker(&mut pdata, &mut cdata, heap.state_data.sectors[i].x, heap.state_data.sectors[i].y, heap.state_data.sectors[i].planet_class, heap.state_data.sectors[i].station_class);
            if HOUSE_PLAYER == heap.state_data.sectors[i].house && PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class {
                ui::add_marker(&mut pdata, &mut cdata, heap.state_data.sectors[i].x, heap.state_data.sectors[i].y-0.25, heap.state_data.sectors[i].planet_class, heap.state_data.sectors[i].station_class);
                ui::add_marker(&mut pdata, &mut cdata, heap.state_data.sectors[i].x, heap.state_data.sectors[i].y+0.25, heap.state_data.sectors[i].planet_class, heap.state_data.sectors[i].station_class);
                ui::add_marker(&mut pdata, &mut cdata, heap.state_data.sectors[i].x-0.25, heap.state_data.sectors[i].y, heap.state_data.sectors[i].planet_class, heap.state_data.sectors[i].station_class);
                ui::add_marker(&mut pdata, &mut cdata, heap.state_data.sectors[i].x+0.25, heap.state_data.sectors[i].y, heap.state_data.sectors[i].planet_class, heap.state_data.sectors[i].station_class);
            }
        }
    }

    
    /////////////////////////////

    for i in 0..pdata.len() {
        pdata[i].x = pdata[i].x / 960.0;
        pdata[i].y = pdata[i].y / 540.0;
    }

    
    for i in 0..tpdata.len() {
        tpdata[i].x = tpdata[i].x / 960.0;
        tpdata[i].y = tpdata[i].y / 540.0;
    }

    world.entity_set_triangle_buffer(cache.uidata.galaxy_territory as usize, &tpdata, &tcdata);

    world.entity_set_line_buffer(cache.uidata.galaxy_geom as usize, &pdata, &cdata);

}


pub fn run_sci_sim(cache: &GameData, heap: &GameDataHeap) -> i32 {

    let sidx = cache.state_cache.location as usize;

    let rate = heap.state_data.sectors[sidx].planet_data.frac_sci * heap.state_data.sectors[sidx].planet_data.sci_level as f32;
    if rate < 1.0e-6 {
        return 999999;
    }

    ((100.0 - heap.state_data.sectors[sidx].planet_data.sci_research_percent) / rate) as i32
}


pub fn run_mfg_sim(cache: &GameData, heap: &mut GameDataHeap) -> (i32, i32, i32, i32, i32) {

    let sidx = cache.state_cache.location as usize;

    let mut bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
    let mut fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
    let mut ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
    let mut mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
    let mut meds = heap.state_data.sectors[sidx].planet_data.cur_meds;

    let popdata = run_pop_sim(cache, heap);

    for i in 0..popdata.len() {
        let labor = heap.state_data.sectors[sidx].planet_data.frac_labor * popdata[i] as f32;
        bios += (labor * heap.state_data.sectors[sidx].planet_data.frac_bios * heap.state_data.sectors[sidx].planet_data.mfg_level_bios as f32 * 0.1) as i32;
        fuel += (labor * heap.state_data.sectors[sidx].planet_data.frac_fuel * heap.state_data.sectors[sidx].planet_data.mfg_level_fuel as f32 * 0.1) as i32;
        ice += (labor * heap.state_data.sectors[sidx].planet_data.frac_ice * heap.state_data.sectors[sidx].planet_data.mfg_level_ice as f32 * 0.1) as i32;
        mach += (labor * heap.state_data.sectors[sidx].planet_data.frac_mach * heap.state_data.sectors[sidx].planet_data.mfg_level_mach as f32 * 0.1) as i32;
        meds += (labor * heap.state_data.sectors[sidx].planet_data.frac_meds * heap.state_data.sectors[sidx].planet_data.mfg_level_meds as f32 * 0.1) as i32;
    }

    (bios, fuel, ice, mach, meds)
}


pub fn run_pop_sim(cache: & GameData, heap: &mut GameDataHeap) -> Vec::<i32> {

    let sidx = cache.state_cache.location as usize;
    
    let mut cur_farm = heap.state_data.sectors[sidx].planet_data.cur_farm as i32;
    let mut cur_labor = heap.state_data.sectors[sidx].planet_data.cur_labor as i32;
    let mut cur_sci = heap.state_data.sectors[sidx].planet_data.cur_sci as i32;
    let mut cur_pop = heap.state_data.sectors[sidx].planet_data.cur_pop as i32;

    let food_farm = 3.0;
    let food_labor = 4.0;
    let food_sci = 2.0;

    let pop_rate = 0.01;
    let food_rate = heap.state_data.sectors[sidx].planet_data.food_rate as f32;
    
    let mut pop: Vec<i32> = Vec::new();
    pop.push(cur_pop);

    // next year
    for _ in 0..99 {
        let food_produced = food_rate * cur_farm as f32;
        let food_delta = food_produced - (cur_farm as f32 * food_farm + cur_labor as f32 * food_labor + cur_sci as f32 * food_sci);
        let next_pop = cur_pop as f32 + food_delta * pop_rate;
        cur_farm = (next_pop * heap.state_data.sectors[sidx].planet_data.frac_farm).floor() as i32;
        cur_labor = (next_pop * heap.state_data.sectors[sidx].planet_data.frac_labor).floor() as i32;
        cur_sci = (next_pop * heap.state_data.sectors[sidx].planet_data.frac_sci).floor() as i32;
        cur_pop = i32::max(0, cur_farm + cur_labor + cur_sci);
        pop.push(cur_pop);
    }

    pop
}


pub fn game_step(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, years: i32) {

    let pplanets = get_num_planets_player(heap) as f32;

    for _ in 0..years {
        // sim step
        cache.state_cache.year += 1;
        for i in 0..heap.state_data.sectors.len() {
            // update station
            if STATION_CLASS_INVALID != heap.state_data.sectors[i].station_class {
                heap.state_data.sectors[i].station_data.trade_price_bios += world.rnd().round() as i32;
                heap.state_data.sectors[i].station_data.trade_price_fuel += world.rnd().round() as i32;
                heap.state_data.sectors[i].station_data.trade_price_ice += world.rnd().round() as i32;
                heap.state_data.sectors[i].station_data.trade_price_mach += world.rnd().round() as i32;
                heap.state_data.sectors[i].station_data.trade_price_meds += world.rnd().round() as i32;
            
                // to do - only need to do once
                let md = if STATION_CLASS_PORT == heap.state_data.sectors[i].station_class { 1.5 } else { 0.8 };
                heap.state_data.sectors[i].station_data.merc_price = ((0.9 + 0.3 * world.rnd()) * 1000.0 * pplanets * md).round() as i32;
                let md = if STATION_CLASS_PORT == heap.state_data.sectors[i].station_class { 1.0 } else { 2.0 };
                heap.state_data.sectors[i].station_data.merc_available = ((0.9 + 0.3 * world.rnd()) * pplanets * md).ceil() as u8;
            }

            // update enemy planets
            if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class && HOUSE_PLAYER != heap.state_data.sectors[i].house {
                
                // claim nearest non-house planet
                let idx = find_nearest_enemy_planet(heap, i, heap.state_data.sectors[i].house);
                if -1 != idx && 99 < cache.state_cache.year { // success
                    if HOUSE_PLAYER == heap.state_data.sectors[idx as usize].house {
                        //println!("house {} attacks PLAYER at {}", heap.state_data.sectors[i].house, idx);
                    } else {
                        //println!("house {} attacks house {} at {}", heap.state_data.sectors[i].house, heap.state_data.sectors[idx as usize].house, idx);
                        if world.rnd() < 0.001 {
                            //println!("- win!");
                            if 1 == get_num_planets_house(heap, heap.state_data.sectors[idx as usize].house) {
                                println!("House {} defeated by house {}!", heap.state_data.sectors[idx as usize].house, heap.state_data.sectors[i].house);
                                heap.state_data.dead_houses.push(heap.state_data.sectors[idx as usize].house);
                            }
                            heap.state_data.sectors[idx as usize].house = heap.state_data.sectors[i].house;
                        }
                    }
                }

                // re-build tribute capacity
                if heap.state_data.sectors[i].tribute_multiplier < 0.99 {
                    heap.state_data.sectors[i].tribute_multiplier += (1.0 - heap.state_data.sectors[i].tribute_multiplier) * 0.1;
                }
            }

            // update planet
            if PLANET_CLASS_INVALID != heap.state_data.sectors[i].planet_class && HOUSE_PLAYER == heap.state_data.sectors[i].house {

                let sidx = i;

                // update population
                let mut cur_farm = heap.state_data.sectors[sidx].planet_data.cur_farm as i32;
                let mut cur_labor = heap.state_data.sectors[sidx].planet_data.cur_labor as i32;
                let mut cur_sci = heap.state_data.sectors[sidx].planet_data.cur_sci as i32;
                let mut cur_pop = heap.state_data.sectors[sidx].planet_data.cur_pop as i32;
            
                let food_farm = 3.0;
                let food_labor = 4.0;
                let food_sci = 2.0;
            
                let pop_rate = 0.01;
                let food_rate = heap.state_data.sectors[sidx].planet_data.food_rate as f32;
                let sci_level = heap.state_data.sectors[sidx].planet_data.sci_level as f32;
                
                // next year
                let food_produced = food_rate * cur_farm as f32;
                let food_delta = food_produced - (cur_farm as f32 * food_farm + cur_labor as f32 * food_labor + cur_sci as f32 * food_sci);
                let next_pop = cur_pop as f32 + food_delta * pop_rate;
                cur_farm = i32::max(0, (next_pop * heap.state_data.sectors[sidx].planet_data.frac_farm).floor() as i32);
                cur_labor = i32::max(0, (next_pop * heap.state_data.sectors[sidx].planet_data.frac_labor).floor() as i32);
                cur_sci = i32::max(0, (next_pop * heap.state_data.sectors[sidx].planet_data.frac_sci).floor() as i32);
                cur_pop = cur_farm + cur_labor + cur_sci;
                
                heap.state_data.sectors[sidx].planet_data.cur_pop = cur_pop;
                heap.state_data.sectors[sidx].planet_data.cur_farm = cur_farm;
                heap.state_data.sectors[sidx].planet_data.cur_labor = cur_labor;
                heap.state_data.sectors[sidx].planet_data.cur_sci = cur_sci;

                // update manufacturing
                let mut bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
                let mut fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
                let mut ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
                let mut mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
                let mut meds = heap.state_data.sectors[sidx].planet_data.cur_meds;
            
                let labor = heap.state_data.sectors[sidx].planet_data.frac_labor * cur_pop as f32;
                bios += (labor * heap.state_data.sectors[sidx].planet_data.frac_bios * heap.state_data.sectors[sidx].planet_data.mfg_level_bios as f32 * 0.01) as i32;
                fuel += (labor * heap.state_data.sectors[sidx].planet_data.frac_fuel * heap.state_data.sectors[sidx].planet_data.mfg_level_fuel as f32 * 0.01) as i32;
                ice += (labor * heap.state_data.sectors[sidx].planet_data.frac_ice * heap.state_data.sectors[sidx].planet_data.mfg_level_ice as f32 * 0.01) as i32;
                mach += (labor * heap.state_data.sectors[sidx].planet_data.frac_mach * heap.state_data.sectors[sidx].planet_data.mfg_level_mach as f32 * 0.01) as i32;
                meds += (labor * heap.state_data.sectors[sidx].planet_data.frac_meds * heap.state_data.sectors[sidx].planet_data.mfg_level_meds as f32 * 0.01) as i32;
            
                heap.state_data.sectors[sidx].planet_data.cur_bios = bios;
                heap.state_data.sectors[sidx].planet_data.cur_fuel = fuel;
                heap.state_data.sectors[sidx].planet_data.cur_ice = ice;
                heap.state_data.sectors[sidx].planet_data.cur_mach = mach;
                heap.state_data.sectors[sidx].planet_data.cur_meds = meds;

                // update science
                if RESEARCH_INVALID != heap.state_data.sectors[sidx].planet_data.researching &&
                    ALL_RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.researching {

                    let rate = heap.state_data.sectors[sidx].planet_data.frac_sci * sci_level;
                    heap.state_data.sectors[sidx].planet_data.sci_research_percent += rate;
                    if 100.0 <= heap.state_data.sectors[sidx].planet_data.sci_research_percent {
                        // research complete
                        heap.state_data.sectors[sidx].planet_data.sci_research_percent = 0.0;
                        let ridx = heap.state_data.sectors[sidx].planet_data.researching;
                        heap.state_data.sectors[sidx].planet_data.research_status[ridx as usize] = RESEARCH_COMPLETE;
                        heap.state_data.sectors[sidx].planet_data.researching = RESEARCH_INVALID;
                        match ridx {
                            RESEARCH_AGRICULTURE_I => {
                                heap.state_data.sectors[sidx].planet_data.food_rate += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_AGRICULTURE_II as usize] = RESEARCH_NOT_STARTED;
                            },
                            RESEARCH_AGRICULTURE_II => {
                                heap.state_data.sectors[sidx].planet_data.food_rate += 1;
                            },
                            RESEARCH_KNOWLEDGE_I => {
                                heap.state_data.sectors[sidx].planet_data.sci_level += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_KNOWLEDGE_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_KNOWLEDGE_II => {
                                heap.state_data.sectors[sidx].planet_data.sci_level += 1;
                            }
                            RESEARCH_BIOLOGY_I => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_bios += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_BIOLOGY_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_BIOLOGY_II => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_bios += 1;
                            }
                            RESEARCH_ENERGY_I => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_fuel += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_ENERGY_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_ENERGY_II => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_fuel += 1;
                            }
                            RESEARCH_MEDICINE_I => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_meds += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MEDICINE_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_MEDICINE_II => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_meds += 1;
                            }
                            RESEARCH_TECHNOLOGY_I => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_mach += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_TECHNOLOGY_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_TECHNOLOGY_II => {
                                heap.state_data.sectors[sidx].planet_data.mfg_level_mach += 1;
                            }
                            RESEARCH_MILITARY_I => {
                                heap.state_data.sectors[sidx].planet_data.fighter_rate += 1;
                                heap.state_data.sectors[sidx].planet_data.research_status[RESEARCH_MILITARY_II as usize] = RESEARCH_NOT_STARTED;
                            }
                            RESEARCH_MILITARY_II => {
                                heap.state_data.sectors[sidx].planet_data.fighter_rate += 1;
                            }
                            _ => (),
                        }
                    }
                }

                let mut all_done = true;
                for k in 1..RESEARCH_COUNT {
                    if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[k] {
                        all_done = false;
                        break;
                    }
                }

                if all_done {
                    heap.state_data.sectors[sidx].planet_data.researching = ALL_RESEARCH_COMPLETE;
                }

                // update fighters
                let fighter_rate = heap.state_data.sectors[sidx].planet_data.fighter_rate as f32 * 0.10;
                let frac_mach = heap.state_data.sectors[sidx].planet_data.frac_mach;                
                let labor = heap.state_data.sectors[sidx].planet_data.cur_labor;
                let fighter_prod = frac_mach * labor as f32 * fighter_rate * 0.01;

                heap.state_data.sectors[sidx].planet_data.fighters_rem += fighter_prod;
                let base = heap.state_data.sectors[sidx].planet_data.fighters_rem.floor() as i32;
                if 0 < base {
                    heap.state_data.sectors[sidx].planet_data.fighters_rem -= base as f32;
                    heap.state_data.sectors[sidx].planet_data.fighters += base;
                }

            }
        }
    }

}



pub fn generate_names(world: &mut mgfw::ecs::World, count: usize) -> Vec::<String> {

    let file = File::open("assets/houses.dat").unwrap();
    //let file = File::open("assets/country_names.dat").unwrap();
    
    let reader = io::BufReader::new(file);

    let mut names: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let len = line.len();
        if 2 > len {
            continue;
        }

        names.push(line);
    }

    let mut numvec: Vec<usize> = Vec::new();
    for i in 0..names.len() {
        numvec.push(i);
    }

    for _ in 0..10000 {
        let idx0 = world.rnd_range(0..names.len());
        let idx1 = world.rnd_range(0..names.len());
        let temp = numvec[idx0];
        numvec[idx0] = numvec[idx1];
        numvec[idx1] = temp;
    }

    let mut retvec: Vec<String> = Vec::new();

    for i in 0..names.len() {
        retvec.push(names[numvec[i]].clone());
    }

    /*loop {
        let ret: String;

        loop {

            let idx = world.rnd_range(0..names.len());

            let mut name = names[idx].clone();
            let name = name.as_mut_str();

            let name_backup = names[idx].clone();

            let chars_backup: Vec<char> = name_backup.chars().collect();

            let mut chars: Vec<char> = name.chars().collect();

            let nchar = chars.len();
            let nswaps = (nchar + (nchar % 2)) / 2 - 1;
            
            for _ in 0..nswaps {
                let a = world.rnd_range(0..nchar);
                let mut b = world.rnd_range(0..nchar);
                if a == b { b = world.rnd_range(0..nchar); }
                if a == b { b = world.rnd_range(0..nchar); }

                let t = chars[a];
                chars[a] = chars[b];
                chars[b] = t;
            }

            let mut pass = false;
            for i in 0..nchar {
                if chars[i] != chars_backup[i] {
                    pass = true;
                    break;
                }
            }

            if pass {
                ret = chars.iter().collect::<String>();
                break;
            }
        }

        let v = format!("{}{}", ret.chars().next().unwrap().to_uppercase(), ret.chars().skip(1).collect::<String>());

        let mut found = false;
        for i in 0..retvec.len() {
            if retvec[i].eq(&v) { found = true; break; }
        }

        if !found {
            println!("{}", v.clone());
            retvec.push(v);
            if count == retvec.len() { break; }
        }
    }*/

    retvec
    
}


fn generate_house_colors(world: &mut mgfw::ecs::World, count: usize) -> Vec<mgfw::ecs::Color> {

    let mut ret: Vec<mgfw::ecs::Color> = Vec::new();

    for _ in 0..count {
        let an = world.rnd() * 360.0;
        let (r, g, b) = hsv2rgb(an, 0.8, 0.6);
        let c = mgfw::ecs::Color{ r, g, b, a: 1.0 };
        ret.push(c);
    }

    ret
}

fn hsv2rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {

    if s < 1.0e-6 {
        return (v, v, v);
    }

    let hh = (if 360.0 <= h { 0.0 } else { h }) / 60.0;
    let i = hh.floor() as i32;
    let ff = hh - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - (s * ff));
    let t = v * (1.0 - (s * (1.0 - ff)));

    match i {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

fn generate_planet_names(world: &mut mgfw::ecs::World, count: usize) -> Vec<String> {
    
    let mut base_names: Vec<String> = Vec::new();
    
    base_names.push(String::from("Alpha"));
    base_names.push(String::from("Beta"));
    base_names.push(String::from("Gamma"));
    base_names.push(String::from("Delta"));
    base_names.push(String::from("Epsilon"));
    base_names.push(String::from("Zeta"));
    base_names.push(String::from("Eta"));
    base_names.push(String::from("Theta"));
    base_names.push(String::from("Iota"));
    base_names.push(String::from("Kappa"));
    base_names.push(String::from("Lambda"));
    base_names.push(String::from("Mu"));
    base_names.push(String::from("Nu"));
    base_names.push(String::from("Xi"));
    base_names.push(String::from("Omicron"));
    base_names.push(String::from("Rho"));
    base_names.push(String::from("Sigma"));
    base_names.push(String::from("Tau"));
    base_names.push(String::from("Upsilon"));
    base_names.push(String::from("Phi"));
    base_names.push(String::from("Chi"));
    base_names.push(String::from("Psi"));
    base_names.push(String::from("Omega"));

    let mut pairs: Vec<(usize, usize)> = Vec::new();

    for _ in 0..count {
        loop {
            let i0 = world.rnd_range(0..base_names.len());
            let i1 = world.rnd_range(0..base_names.len());
            if i0 == i1 { continue; }
            if pairs.contains(&(i0, i1)) || pairs.contains(&(i1, i0)) { continue; }
            pairs.push((i0, i1));
            break;
        }
    }
 
    let mut ret: Vec<String> = Vec::new();

    for i in 0..pairs.len() {
        ret.push(format!("{} {}", base_names[pairs[i].0], base_names[pairs[i].1]));
    }

    ret

}


fn generate_avatar_strings(world: &mut mgfw::ecs::World) -> Vec<String> {

    let mut numvec: Vec<u8> = Vec::new();
    for i in 0..74 {
        if 5 == i { continue; }
        numvec.push(i+1);
    }

    let n = numvec.len();

    // shuffle
    for _ in 0..1000 {
        let idx0 = world.rnd_range(0..n);
        let idx1 = world.rnd_range(0..n);
        let temp = numvec[idx0];
        numvec[idx0] = numvec[idx1];
        numvec[idx1] = temp;
    }

    let mut ret: Vec<String> = Vec::new();

    for i in 0..numvec.len() {
        let v = numvec[i];
        let mut s = format!("{}_sm.png", v);
        if v < 10 {
            s = format!("0{s}");
        }

        ret.push(s.clone());
    }

    ret

}