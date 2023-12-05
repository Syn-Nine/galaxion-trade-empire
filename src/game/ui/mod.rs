use crate::game::game::{GameData, GameDataHeap};
use crate::mgfw;
use crate::mgfw::ecs::ugui::{self, HALIGN_LEFT, HALIGN_CENTER, HALIGN_RIGHT, PLOT_DATA_MARKER_BLUE, VALIGN_TOP};
use crate::mgfw::ecs::uigrid::UIgrid;
use super::enums::*;
use super::state;

mod planet;
mod galaxy;
mod station;
pub mod battle;

use planet::*;
use station::*;
use galaxy::*;
use battle::*;

const CALLBACK_INVALID: u8 = 0;
//
const CALLBACK_PLANET_MAIN_POPULATION: u8 = 0;
const CALLBACK_PLANET_MAIN_MANUFACTURING: u8 = 1;
const CALLBACK_PLANET_MAIN_RESEARCH: u8 = 2;
const CALLBACK_PLANET_MAIN_MILITARY: u8 = 3;
const CALLBACK_PLANET_MAIN_CARGO: u8 = 4;
const CALLBACK_PLANET_MAIN_HANGAR: u8 = 5;
const CALLBACK_PLANET_MAIN_LIFTOFF: u8 = 6;
const CALLBACK_PLANET_BACK_TO_MAIN: u8 = 7;
const CALLBACK_PLANET_TAKE_MAX_FIGHTERS: u8 = 8;
const CALLBACK_PLANET_MAX_RESOURCES: u8 = 9;
//
const CALLBACK_POPULATION_SLIDER_FARMER: u8 = 10;
const CALLBACK_POPULATION_SLIDER_LABORER: u8 = 11;
const CALLBACK_POPULATION_SLIDER_SCIENTIST: u8 = 12;
//
const CALLBACK_MANUFACTURING_SLIDER_BIOLOGICS: u8 = 13;
const CALLBACK_MANUFACTURING_SLIDER_FUEL: u8 = 14;
const CALLBACK_MANUFACTURING_SLIDER_ICE: u8 = 15;
const CALLBACK_MANUFACTURING_SLIDER_MACHINERY: u8 = 16;
const CALLBACK_MANUFACTURING_SLIDER_MEDICINE: u8 = 17;
//
const CALLBACK_GALAXY_LAND: u8 = 20;
const CALLBACK_GALAXY_DOCK: u8 = 21;
const CALLBACK_GALAXY_STATS: u8 = 22;
const CALLBACK_GALAXY_NAVIGATE: u8 = 23;
const CALLBACK_GALAXY_CLOSE_STATS: u8 = 24;
//
const CALLBACK_STATION_MAIN_TRADE: u8 = 25;
const CALLBACK_STATION_MAIN_BANK: u8 = 26;
const CALLBACK_STATION_MAIN_MARKETLINK: u8 = 27;
const CALLBACK_STATION_MAIN_TERRALINK: u8 = 28;
const CALLBACK_STATION_MAIN_NAV: u8 = 29;
const CALLBACK_STATION_MAIN_HOUSES: u8 = 30;
const CALLBACK_STATION_MAIN_CARGO: u8 = 31;
const CALLBACK_STATION_MAIN_HANGAR: u8 = 32;
const CALLBACK_STATION_MAIN_LIFTOFF: u8 = 33;
const CALLBACK_STATION_BACK_TO_MAIN: u8 = 34;
//
const CALLBACK_STATION_TERRALINK_PAGEUP: u8 = 35;
const CALLBACK_STATION_TERRALINK_PAGEDOWN: u8 = 36;
const CALLBACK_STATION_MARKETLINK_PAGEUP: u8 = 37;
const CALLBACK_STATION_MARKETLINK_PAGEDOWN: u8 = 38;
//
const CALLBACK_STATION_HANGAR_SLIDER: u8 = 39;
const CALLBACK_STATION_TAKE_MAX_FIGHTERS: u8 = 40;
const CALLBACK_STATION_HIRE_FIGHTERS: u8 = 41;
//
const CALLBACK_STATION_NAV_DOWNLOAD_LOCAL: u8 = 42;
const CALLBACK_STATION_NAV_DOWNLOAD_PATH_0: u8 = 43;
const CALLBACK_STATION_NAV_DOWNLOAD_PATH_1: u8 = 44;
const CALLBACK_STATION_NAV_DOWNLOAD_ALL: u8 = 45;
const CALLBACK_STATION_NAV_TO: u8 = 46;
//
const CALLBACK_STATION_HOUSES_PGUP: u8 = 47;
const CALLBACK_STATION_HOUSES_PGDN: u8 = 48;
//
const CALLBACK_CARGO_SLIDER_COLONISTS: u8 = 50;
const CALLBACK_CARGO_SLIDER_BIOLOGICS: u8 = 51;
const CALLBACK_CARGO_SLIDER_FUEL: u8 = 52;
const CALLBACK_CARGO_SLIDER_ICE: u8 = 53;
const CALLBACK_CARGO_SLIDER_MACHINERY: u8 = 54;
const CALLBACK_CARGO_SLIDER_MEDICINE: u8 = 55;
//
const CALLBACK_TRADE_SLIDER_BIOLOGICS: u8 = 56;
const CALLBACK_TRADE_SLIDER_FUEL: u8 = 57;
const CALLBACK_TRADE_SLIDER_ICE: u8 = 58;
const CALLBACK_TRADE_SLIDER_MACHINERY: u8 = 59;
const CALLBACK_TRADE_SLIDER_MEDICINE: u8 = 60;
const CALLBACK_TRADE_RESET: u8 = 61;
const CALLBACK_TRADE_COMMIT: u8 = 62;
const CALLBACK_TRADE_BUY_MAX: u8 = 63;
const CALLBACK_TRADE_SELL_MAX: u8 = 64;
//
const CALLBACK_PLANET_RESEARCH_BUTTON: u8 = 70;
// make sure these follow research enums in enums.rs vvvvvvv
const CALLBACK_RESEARCH_BASE: u8 = 70;
/* not used, but accounted for in enumeration
const CALLBACK_RESEARCH_AGRICULTURE_I: u8 = 71;
const CALLBACK_RESEARCH_BIOLOGY_I: u8 = 72;
const CALLBACK_RESEARCH_ENERGY_I: u8 = 73;
const CALLBACK_RESEARCH_KNOWLEDGE_I: u8 = 74;
const CALLBACK_RESEARCH_MEDICINE_I: u8 = 75;
const CALLBACK_RESEARCH_MILITARY_I: u8 = 76;
const CALLBACK_RESEARCH_TECHNOLOGY_I: u8 = 77;
const CALLBACK_RESEARCH_AGRICULTURE_II: u8 = 78;
const CALLBACK_RESEARCH_BIOLOGY_II: u8 = 79;
const CALLBACK_RESEARCH_ENERGY_II: u8 = 80;
const CALLBACK_RESEARCH_KNOWLEDGE_II: u8 = 81;
const CALLBACK_RESEARCH_MEDICINE_II: u8 = 82;
const CALLBACK_RESEARCH_MILITARY_II: u8 = 83;
const CALLBACK_RESEARCH_TECHNOLOGY_II: u8 = 84;
*/
//
const CALLBACK_BATTLE_DEMAND_TRIBUTE: u8 = 90;
const CALLBACK_BATTLE_DEMAND_SURRENDER: u8 = 91;
const CALLBACK_BATTLE_OFFER_TRIBUTE: u8 = 92;
const CALLBACK_BATTLE_SURRENDER: u8 = 93;
const CALLBACK_BATTLE_CLOSE_BATTLE: u8 = 94;
const CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE_DEMAND: u8 = 95;
const CALLBACK_BATTLE_REJECT_PLAYER_SURRENDER_DEMAND: u8 = 96;
const CALLBACK_BATTLE_ACCEPT_PLAYER_TRIBUTE: u8 = 97;
const CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE: u8 = 98;
const CALLBACK_BATTLE_ACCEPT_ENEMY_TRIBUTE: u8 = 99;
const CALLBACK_BATTLE_REJECT_ENEMY_TRIBUTE: u8 = 100;
const CALLBACK_BATTLE_AUTO_TOGGLE: u8 = 101;
//
const CALLBACK_LAUNCH_BATTLE: u8 = 102;
//
const CALLBACK_QUIT_GAME: u8 = 110;
const CALLBACK_QUIT_GAME_POPUP: u8 = 111;
const CALLBACK_QUIT_GAME_POPUP_CANCEL: u8 = 112;
//
pub const HOVER_SECTOR_INVALID: u8 = 0xFF;
pub const SELECTED_UNIT_INVALID: u8 = 0xFF;
//
const MENU_NONE: u8 = 0;
const MENU_GALAXY_LOCATION: u8 = 1;
const MENU_GALAXY_NAVIGATE: u8 = 2;
const MENU_GALAXY_FREIGHTER_STATUS: u8 = 3;

const MENU_PLANET_MAIN: u8 = 10;
const MENU_PLANET_POPULATION: u8 = 11;
const MENU_PLANET_RESEARCH: u8 = 12;
const MENU_PLANET_MILITARY: u8 = 13;
const MENU_PLANET_CARGO: u8 = 14;
const MENU_PLANET_HANGAR: u8 = 15;
const MENU_PLANET_MANUFACTURING: u8 = 16;

const MENU_STATION_MAIN: u8 = 30;
const MENU_STATION_CARGO: u8 = 31;
const MENU_STATION_NAV: u8 = 32;
const MENU_STATION_TRADE: u8 = 33;
const MENU_STATION_HANGAR: u8 = 34;
const MENU_STATION_TERRALINK: u8 = 35;
const MENU_STATION_MARKETLINK: u8 = 36;
const MENU_STATION_HOUSES: u8 = 37;

const MENU_BATTLE_BASE: u8 = 20;
const MENU_BATTLE_ENEMY_REJECTS_TRIBUTE_DEMAND: u8 = 21;
const MENU_BATTLE_ENEMY_REJECTS_SURRENDER_DEMAND: u8 = 22;
const MENU_BATTLE_ENEMY_SURRENDER: u8 = 23;
const MENU_BATTLE_ENEMY_REJECTS_PLAYER_TRIBUTE: u8 = 24;
const MENU_BATTLE_ENEMY_ACCEPTS_PLAYER_TRIBUTE: u8 = 25;
const MENU_BATTLE_PLAYER_SURRENDER: u8 = 26;
const MENU_BATTLE_ENEMY_OFFERS_TRIBUTE: u8 = 27;
const MENU_BATTLE_GAME_OVER: u8 = 28;
const MENU_BATTLE_GAME_WIN: u8 = 29;

const MENU_QUIT_CONFIRM: u8 = 40;

const MENU_STATS: u8 = 5;


pub struct UIData {
    ui_pop_tab: u8,
    ui_mfg_tab: u8,
    ui_research_tab: u8,
    ui_cargo_tab: u8,
    ui_station_tab: u8,
    //
    ui_pop_slider_farm: u8,
    ui_pop_slider_labor: u8,
    ui_pop_slider_sci: u8,
    ui_pop_farm: u8,
    ui_pop_farm_curr: u8,
    ui_pop_labor: u8,
    ui_pop_labor_curr: u8,
    ui_pop_sci: u8,
    ui_pop_sci_curr: u8,
    ui_pop_end: u8,
    ui_pop_graph: u8,
    ui_mfg_slider_bios: u8,
    ui_mfg_slider_fuel: u8,
    ui_mfg_slider_ice: u8,
    ui_mfg_slider_mach: u8,
    ui_mfg_slider_meds: u8,
    ui_mfg_bios: u8,
    ui_mfg_fuel: u8,
    ui_mfg_ice: u8,
    ui_mfg_mach: u8,
    ui_mfg_meds: u8,
    ui_mfg_end_bios: u8,
    ui_mfg_end_fuel: u8,
    ui_mfg_end_ice: u8,
    ui_mfg_end_mach: u8,
    ui_mfg_end_meds: u8,
    //
    ui_cargo_slider_colonists: u8,
    ui_cargo_slider_bios: u8,
    ui_cargo_slider_fuel: u8,
    ui_cargo_slider_ice: u8,
    ui_cargo_slider_mach: u8,
    ui_cargo_slider_meds: u8,
    //
    ui_cargo_colonists_qty: u8,
    ui_cargo_colonists_avail: u8,
    ui_cargo_colonists_holds: u8,
    ui_cargo_bios_qty: u8,
    ui_cargo_bios_avail: u8,
    ui_cargo_bios_holds: u8,
    ui_cargo_fuel_qty: u8,
    ui_cargo_fuel_avail: u8,
    ui_cargo_fuel_holds: u8,
    ui_cargo_ice_qty: u8,
    ui_cargo_ice_avail: u8,
    ui_cargo_ice_holds: u8,
    ui_cargo_mach_qty: u8,
    ui_cargo_mach_avail: u8,
    ui_cargo_mach_holds: u8,
    ui_cargo_meds_qty: u8,
    ui_cargo_meds_avail: u8,
    ui_cargo_meds_holds: u8,
    ui_cargo_holds_tot: u8,
    //
    ui_trade_bios_qty: u8,
    ui_trade_bios_holds: u8,
    ui_trade_bios_price: u8,
    ui_trade_bios_net: u8,
    ui_trade_bios_eff_price: u8,
    ui_trade_slider_bios: u8,
    ui_trade_fuel_qty: u8,
    ui_trade_fuel_holds: u8,
    ui_trade_fuel_price: u8,
    ui_trade_fuel_net: u8,
    ui_trade_fuel_eff_price: u8,
    ui_trade_slider_fuel: u8,
    ui_trade_ice_qty: u8,
    ui_trade_ice_holds: u8,
    ui_trade_ice_price: u8,
    ui_trade_ice_net: u8,
    ui_trade_ice_eff_price: u8,
    ui_trade_slider_ice: u8,
    ui_trade_mach_qty: u8,
    ui_trade_mach_holds: u8,
    ui_trade_mach_price: u8,
    ui_trade_mach_net: u8,
    ui_trade_mach_eff_price: u8,
    ui_trade_slider_mach: u8,
    ui_trade_meds_qty: u8,
    ui_trade_meds_holds: u8,
    ui_trade_meds_price: u8,
    ui_trade_meds_net: u8,
    ui_trade_meds_eff_price: u8,
    ui_trade_slider_meds: u8,
    ui_trade_holds_tot: u8,
    ui_trade_cash_tot: u8,
    ui_trade_commit_button: u8,
    //
    pub curr_sector: u8,
    //
    pub zoom_level: u8,
    pub zoom_cam_x: f32,
    pub zoom_cam_y: f32,
    //
    panning: bool,
    prev_pan_x: i32,
    prev_pan_y: i32,
    is_menu_open: bool,
    galaxy_hover_x: f32,
    galaxy_hover_y: f32,
    screen_hover_x: f32,
    screen_hover_y: f32,
    //
    galaxy_map: u16,
    pub galaxy_territory: u16,
    pub galaxy_geom: u16,
    pub galaxy_path: u16,
    pub galaxy_ship_entities: u16,
    galaxy_tileset: u16,
    pub galaxy_hover_ent: u16,
    galaxy_ui_year: u8,
    //
    menu_grid: UIgrid,
    dim_ent: u16,
    planet_menu_tilemap: u16,
    planet_menu_tileset_lush: u16,
    planet_menu_tileset_desert: u16,
    planet_menu_tileset_ocean: u16,
    planet_menu_tileset_volcano: u16,
    //
    planet_menu_research_title: u8,
    planet_menu_research_button: u8,
    planet_menu_research_selection: u8,
    planet_menu_research_description_title: u8,
    planet_menu_research_description_rows: u8,
    planet_menu_research_percent: u8,
    //
    station_menu_tileset: u16,
    station_nav_ui_all_idx: u8,
    station_nav_ui_local_idx: u8,
    station_nav_ui_path_0_idx: u8,
    station_nav_ui_path_1_idx: u8,
    all_nav_known: bool,
    ui_station_hangar_slider_idx: u8,
    ui_station_hangar_total_cost: u8,
    ui_station_hangar_cash_after_hire: u8,
    ui_station_hangar_qty_avail: u8,
    ui_station_hangar_hire_button: u8,
    ui_station_terralink_table: u8,
    ui_station_terralink_table_offset: u8,
    ui_station_terralink_pgup: u8,
    ui_station_terralink_pgdn: u8,
    ui_station_marketlink_table_offset: u8,
    ui_station_marketlink_table: u8,
    ui_station_marketlink_pgup: u8,
    ui_station_marketlink_pgdn: u8,
    ui_station_house_base: u8,
    ui_station_house_table_offset: u8,
    ui_station_houses_pgup: u8,
    ui_station_houses_pgdn: u8,
    ui_station_houses_reconstruct_avatars: bool,
    //
    battle_menu_tileset: u16,
    battle_menu_tilemap: u16,
    battle_map_tileset: u16,
    battle_tiles_0: u16,
    battle_tiles_1: u16,
    battle_tiles_0_red: u16,
    battle_tiles_1_red: u16,
    battle_tiles_0_blue: u16,
    battle_tiles_1_blue: u16,
    //
    battle_player_entity_0: u16,
    battle_player_fighter_count_ent_0: u16,
    battle_enemy_entity_0: u16,
    battle_enemy_fighter_count_ent_0: u16,
    battle_selector_ent: u16,
    battle_selector_unit_ent: u16,
    battle_click_delay: u16,
    battle_explosion_tileset_ent: u16,
    battle_explosion_ent: u16,
    battle_animation_lock: bool,
    battle_turn_order_ent: u16,
    battle_turn_order_count_ent: u16,
    battle_turn_order_selector_ent: u16,
    battle_hold: bool,
    //
    battle_ui_player_fighter_count: u8,
    battle_ui_enemy_fighter_count: u8,
    battle_ui_avatar_player: u8,
    battle_ui_avatar_enemy: u16,
    battle_ui_reconstruct_avatars: bool,
    battle_ui_auto_toggle: u8,
    //
    curr_menu: u8,
}

pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    println!("Initializing UI");
    // set default cache data
    cache.uidata.zoom_level = 1;
    cache.uidata.zoom_cam_x = 960.0 / 2.0;
    cache.uidata.zoom_cam_y = 400.0;//540.0 / 2.0;
    cache.uidata.panning = false;
    cache.uidata.curr_sector = HOVER_SECTOR_INVALID;
    cache.uidata.is_menu_open = false;
    cache.uidata.galaxy_hover_x = 0.0;
    cache.uidata.galaxy_hover_y = 0.0;
    cache.uidata.screen_hover_x = 0.0;
    cache.uidata.screen_hover_y = 0.0;
    cache.uidata.curr_menu = MENU_NONE;
    cache.uidata.battle_ui_reconstruct_avatars = false;
    cache.uidata.ui_station_houses_reconstruct_avatars = false;

    // create entities for background elements;

    cache.uidata.galaxy_tileset = world.new_entity() as u16;
    world.entity_set_tileset(cache.uidata.galaxy_tileset as usize, String::from("assets/sm-galaxy-fade.png"), 960, 540, 960, 540);

    cache.uidata.planet_menu_tileset_lush = world.new_entity() as u16;
    cache.uidata.planet_menu_tileset_desert = world.new_entity() as u16;
    cache.uidata.planet_menu_tileset_ocean = world.new_entity() as u16;
    cache.uidata.planet_menu_tileset_volcano = world.new_entity() as u16;
    
    cache.uidata.battle_menu_tileset = world.new_entity() as u16;

    
    cache.uidata.battle_menu_tilemap = world.new_entity() as u16;
    world.entity_set_position_xy(cache.uidata.battle_menu_tilemap as usize, -30.0, -30.0);
    world.entity_set_scale_xy(cache.uidata.battle_menu_tilemap as usize, 960.0, 540.0);
    let f = 0.5;
    world.entity_set_color_rgba(cache.uidata.battle_menu_tilemap as usize, 0.8 * f, 0.8 * f, 1.0 * f, 1.0);
    
    cache.uidata.battle_map_tileset = world.new_entity() as u16;
    world.entity_set_tileset(cache.uidata.battle_map_tileset as usize, String::from("assets/sprites.png"), 128, 128, 16, 16);

    cache.uidata.station_menu_tileset = world.new_entity() as u16;
    //world.entity_set_tileset(cache.uidata.station_menu_tileset, String::from("assets/station.png.png"), 960, 540, 960, 540);

    cache.uidata.planet_menu_tilemap = world.new_entity() as u16;
    world.entity_set_position_xy(cache.uidata.planet_menu_tilemap as usize, -30.0, -30.0);
    world.entity_set_scale_xy(cache.uidata.planet_menu_tilemap as usize, 960.0, 540.0);

    cache.uidata.battle_tiles_0_blue = world.new_entity() as u16;
    cache.uidata.battle_tiles_1_blue = world.new_entity() as u16;
    cache.uidata.battle_tiles_0_red = world.new_entity() as u16;
    cache.uidata.battle_tiles_1_red = world.new_entity() as u16;
    cache.uidata.battle_tiles_0 = world.new_entity() as u16;
    cache.uidata.battle_tiles_1 = world.new_entity() as u16;

    let e = world.new_entity();
    cache.uidata.battle_selector_unit_ent = e as u16;
    world.entity_set_billboard(e, String::from("assets/sel_fighter_player.png"));
    world.entity_set_scale_xy(e, 48.0, 32.0);

    // player battle entities
    for i in 0..17 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_player_entity_0 = e as u16; }
    }

    // enemy battle entities
    for i in 0..17 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_enemy_entity_0 = e as u16; }
    }

    // player ship counter entities
    for i in 0..16 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_player_fighter_count_ent_0 = e as u16; }
    }

    // enemy ship counter entities
    for i in 0..16 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_enemy_fighter_count_ent_0 = e as u16; }
    }

    let e = world.new_entity();
    cache.uidata.battle_selector_ent = e as u16;
    world.entity_set_billboard(e, String::from("assets/selector.png"));
    world.entity_set_scale_xy(e, 48.0, 32.0);

    let e = world.new_entity();
    cache.uidata.battle_explosion_tileset_ent = e as u16;
    world.entity_set_tileset(e, String::from("assets/explosion.png"), 288, 32, 32, 32);

    let e = world.new_entity();
    cache.uidata.battle_explosion_ent = e as u16;
    world.entity_set_tilemap(e, cache.uidata.battle_explosion_tileset_ent as usize, 1, &vec![0]);
    world.entity_set_scale_xy(e, 32.0, 32.0);

    for i in 0..32 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_turn_order_ent = e as u16; }
        world.entity_set_scale_xy(e, 24.0, 16.0);
    }

    for i in 0..32 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.battle_turn_order_count_ent = e as u16; }
    }

    let e = world.new_entity();
    cache.uidata.battle_turn_order_selector_ent = e as u16;
    world.entity_set_billboard(e, String::from("assets/turn_order_selector.png"));
    world.entity_set_scale_xy(e, 32.0, 8.0);

    battle::initialize(cache, heap, world);

    // galaxy map
    cache.uidata.galaxy_map = world.new_entity() as u16;
    world.entity_set_tilemap(cache.uidata.galaxy_map as usize, cache.uidata.galaxy_tileset as usize, 1, &vec![1]);
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, true);

    cache.uidata.galaxy_territory = world.new_entity() as u16;
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, true);

    
    for i in 0..8 {
        let e = world.new_entity();
        if 0 == i { cache.uidata.galaxy_ship_entities = e as u16; }
        world.entity_set_visibility(e, false);
        world.entity_set_billboard(e, String::from("assets/player_freighter.png"));
        world.entity_set_scale_xy(e, 80.0, 48.0);
    }

    cache.uidata.galaxy_geom = world.new_entity() as u16;
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, true);

    
    // galaxy hover entity
    cache.uidata.galaxy_hover_ent = world.new_entity() as u16;
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, true);
    let mut pdata: Vec<mgfw::ecs::Position> = Vec::new();
    let mut cdata: Vec<mgfw::ecs::Color> = Vec::new();
    let c2 = mgfw::ecs::Color::new(1.0, 1.0, 1.0, 1.0);
    let r = 10.0;
        for i in 0..24 {
        let an0 = i as f64 / 24.0 * 2.0 * mgfw::PI;
        let an1 = (i + 1) as f64 / 24.0 * 2.0 * mgfw::PI;
        let xx0 = r * an0.cos();
        let yy0 = r * an0.sin() + 0.5;
        let xx1 = r * an1.cos();
        let yy1 = r * an1.sin() + 0.5;
        pdata.push(mgfw::ecs::Position::new(xx0 as f32, yy0 as f32));
        pdata.push(mgfw::ecs::Position::new(xx1 as f32, yy1 as f32));
        cdata.push(c2);
        cdata.push(c2);
    }
    world.entity_set_line_buffer(cache.uidata.galaxy_hover_ent as usize, &pdata, &cdata);


    // screen dimmer
    cache.uidata.dim_ent = world.new_entity() as u16;

    let mut pdata: Vec<mgfw::ecs::Position> = Vec::new();
    let mut cdata: Vec<mgfw::ecs::Color> = Vec::new();
    pdata.push(mgfw::ecs::Position::new(0.0, 0.0));
    pdata.push(mgfw::ecs::Position::new(0.0, SCREEN_YRES as f32));
    pdata.push(mgfw::ecs::Position::new(SCREEN_XRES as f32, SCREEN_YRES as f32));
    pdata.push(mgfw::ecs::Position::new(0.0, 0.0));
    pdata.push(mgfw::ecs::Position::new(SCREEN_XRES as f32, SCREEN_YRES as f32));
    pdata.push(mgfw::ecs::Position::new(SCREEN_XRES as f32, 0.0));
    for _ in 0..6 {
        cdata.push(mgfw::ecs::Color::new(0.1, 0.05, 0.2, 0.535));
    }
    world.entity_set_triangle_buffer(cache.uidata.dim_ent as usize, &pdata, &cdata);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);

    // path that gets highlighted when navigating
    cache.uidata.galaxy_path = world.new_entity() as u16;

    // galaxy hud window
    build_galaxy_ui_year(cache, world);
    build_galaxy_ui_quit(cache, world);

//    build_battle_ui_base(cache, heap, world);

}


pub fn add_marker(pdata: &mut Vec::<mgfw::ecs::Position>, cdata: &mut Vec::<mgfw::ecs::Color>, x0: f32, y0: f32, pc: u8, sc: u8) {

    let z = 0.3;
    let c_green = mgfw::ecs::Color::new(z, 1.0, z, 1.0);
    let c_blue = mgfw::ecs::Color::new(z, 1.0, 1.0, 1.0);
    let c_yellow = mgfw::ecs::Color::new(1.0, 1.0, z, 1.0);
    let c_red = mgfw::ecs::Color::new(1.0, z, z, 1.0);
    let c_black = mgfw::ecs::Color::new(0.0, 0.0, 0.0, 1.0);
    let c_white = mgfw::ecs::Color::new(1.0, 1.0, 1.0, 1.0);

    let c0 = match pc {
        PLANET_CLASS_LUSH => c_green.clone(),
        PLANET_CLASS_WATER => c_blue.clone(),
        PLANET_CLASS_DESERT => c_yellow.clone(),
        PLANET_CLASS_VOLCANO => c_red.clone(),
        PLANET_CLASS_INVALID => c_black.clone(),
        _ => c_black.clone(),
    };

    let r = 3.5;
    let rc = r * 0.50;
    let rs = r * 0.90;

    pdata.push(mgfw::ecs::Position::new(x0 as f32 + r, y0 as f32 + 0.0));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 - rs));
    
    pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 - rs));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 - rs));

    pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 - rs));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 - r, y0 as f32 + 0.0));

    pdata.push(mgfw::ecs::Position::new(x0 as f32 - r, y0 as f32 + 0.0));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 + rs));

    pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 + rs));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 + rs));

    pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 + rs));
    pdata.push(mgfw::ecs::Position::new(x0 as f32 + r, y0 as f32 + 0.0));

    for _ in 0..12 {
        cdata.push(c0);
    }

    if PLANET_CLASS_INVALID != pc {
        pdata.push(mgfw::ecs::Position::new(x0 as f32 + r, y0 as f32 + 0.0));
        pdata.push(mgfw::ecs::Position::new(x0 as f32 - r, y0 as f32 + 0.0));

        pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 + rs));
        pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 - rs));

        pdata.push(mgfw::ecs::Position::new(x0 as f32 + rc, y0 as f32 - rs));
        pdata.push(mgfw::ecs::Position::new(x0 as f32 - rc, y0 as f32 + rs));

        for _ in 0..6 {
            cdata.push(c0);
        }
    }

    /*let r = 3.5;
    for i in 0..6 {
        let an0 = i as f64 / 6.0 * 2.0 * mgfw::PI;
        let an1 = (i + 1) as f64 / 6.0 * 2.0 * mgfw::PI;
        let xx0 = x0 as f64 + r * an0.cos();
        let yy0 = y0 as f64 + r * an0.sin();
        let xx1 = x0 as f64 + r * an1.cos();
        let yy1 = y0 as f64 + r * an1.sin();
        pdata.push(mgfw::ecs::Position::new(xx0 as f32, yy0 as f32));
        pdata.push(mgfw::ecs::Position::new(xx1 as f32, yy1 as f32));
        cdata.push(c0);
        cdata.push(c0);
    }*/

    if STATION_CLASS_INVALID != sc {

        let c0 = match pc {
            PLANET_CLASS_LUSH => c_green.clone(),
            PLANET_CLASS_WATER => c_blue.clone(),
            PLANET_CLASS_DESERT => c_yellow.clone(),
            PLANET_CLASS_VOLCANO => c_red.clone(),
            PLANET_CLASS_INVALID => c_white.clone(),
            _ => c_white.clone(),
        };

        let r1 = r as f32 * 1.2;
        let r2 = r1 as f32 * 1.6;
        let r3 = r1 as f32 * 2.1;
        
        pdata.push(mgfw::ecs::Position::new(x0 - r2, y0 - r1));
        pdata.push(mgfw::ecs::Position::new(x0 + r2, y0 - r1));
        //
        pdata.push(mgfw::ecs::Position::new(x0 - r2, y0 - r1));
        pdata.push(mgfw::ecs::Position::new(x0, y0 + r3));
        //
        pdata.push(mgfw::ecs::Position::new(x0 + r2, y0 - r1));
        pdata.push(mgfw::ecs::Position::new(x0, y0 + r3));
        //
        for _ in 0..6 {
            cdata.push(c0.clone());
        }

        if STATION_CLASS_FEDERAL == sc {
            let r1 = r1 * 1.5;
            let r2 = r1 * 1.6;
            let r3 = r1 * 2.1;
            
            pdata.push(mgfw::ecs::Position::new(x0 - r2, y0 - r1));
            pdata.push(mgfw::ecs::Position::new(x0 + r2, y0 - r1));
            //
            pdata.push(mgfw::ecs::Position::new(x0 - r2, y0 - r1));
            pdata.push(mgfw::ecs::Position::new(x0, y0 + r3));
            //
            pdata.push(mgfw::ecs::Position::new(x0 + r2, y0 - r1));
            pdata.push(mgfw::ecs::Position::new(x0, y0 + r3));
            //
            for _ in 0..6 {
                cdata.push(c0.clone());
            }
        }
    }
}


pub fn add_connection(pdata: &mut Vec::<mgfw::ecs::Position>, cdata: &mut Vec::<mgfw::ecs::Color>, x0: f32, y0: f32, x1: f32, y1: f32, c0: & mgfw::ecs::Color) {
    pdata.push(mgfw::ecs::Position::new(x0, y0));
    pdata.push(mgfw::ecs::Position::new(x1, y1));
    //
    cdata.push(c0.clone());
    cdata.push(c0.clone());
}

pub fn add_poly_connection(pdata: &mut Vec::<mgfw::ecs::Position>, cdata: &mut Vec::<mgfw::ecs::Color>, x0: f32, y0: f32, x1: f32, y1: f32, c0: & mgfw::ecs::Color, w: f32, r: f32) {

    let dx = x1 - x0;
    let dy = y1 - y0;
    let norm = (dx * dx + dy * dy).sqrt();
    let px = dy / norm;
    let py = -dx / norm;

    let mut c = 0;

    for i in 0..16 {
        let an0 = i as f32 / 16.0 * 2.0 * mgfw::PI as f32;    
        let an1 = (i + 1) as f32 / 16.0 * 2.0 * mgfw::PI as f32;

        pdata.push(mgfw::ecs::Position::new(x0, y0));
        pdata.push(mgfw::ecs::Position::new(x0 + r * an0.cos(), y0 + r * an0.sin()));
        pdata.push(mgfw::ecs::Position::new(x0 + r * an1.cos(), y0 + r * an1.sin()));
        c += 1;

        pdata.push(mgfw::ecs::Position::new(x1, y1));
        pdata.push(mgfw::ecs::Position::new(x1 + r * an0.cos(), y1 + r * an0.sin()));
        pdata.push(mgfw::ecs::Position::new(x1 + r * an1.cos(), y1 + r * an1.sin()));
        c += 1;
    }

    pdata.push(mgfw::ecs::Position::new(x0 + w * px, y0 + w * py));
    pdata.push(mgfw::ecs::Position::new(x0 - w * px, y0 - w * py));
    pdata.push(mgfw::ecs::Position::new(x1 - w * px, y1 - w * py));
    c += 1;

    pdata.push(mgfw::ecs::Position::new(x1 - w * px, y1 - w * py));
    pdata.push(mgfw::ecs::Position::new(x1 + w * px, y1 + w * py));
    pdata.push(mgfw::ecs::Position::new(x0 + w * px, y0 + w * py));
    c += 1;

    //
    for _ in 0..c*3 {
        cdata.push(c0.clone());
    }
    
}


fn get_screen_to_galaxy_xy(cache: &GameData, world: & mgfw::ecs::World, sx: i32, sy: i32) -> (f32, f32) {
    let z = get_cam_zoom_z(cache);
    let p = world.entity_get_position(cache.uidata.galaxy_map as usize);

    let dx = (sx as f32 - p.x) / z;
    let dy = (sy as f32 - p.y) / z;
    (dx, dy)
}

fn get_galaxy_to_screen_xy(cache: &GameData, world: &mgfw::ecs::World, gx: f32, gy: f32) -> (f32, f32) {
    
    let z = get_cam_zoom_z(cache);
    let p = world.entity_get_position(cache.uidata.galaxy_map as usize);

    let sx = gx * z + p.x;
    let sy = gy * z + p.y;
    (sx, sy)
}

fn get_cam_zoom_z(cache: &GameData) -> f32 {
    let w = 150.0;
    let sz = 960.0 + w * (cache.uidata.zoom_level - 1) as f32;
    sz / 960.0
}

fn update_galaxy_hover_selector(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    let (mx, my) = get_screen_to_galaxy_xy(cache, world, world.mouse_x, world.mouse_y);
    // find nearest loc
    let mut minidx = heap.state_data.sectors.len();
    let mut mindist = 1.0e+9;
    for i in 0..heap.state_data.sectors.len() {
        if !heap.state_data.sectors[i].known { continue; } // skip
        let dx = (heap.state_data.sectors[i].x - mx).abs();
        let dy = (heap.state_data.sectors[i].y - my).abs();
        if dy < 30.0 && dx < 30.0 {
            let d = dx * dx + dy * dy;
            if d < mindist {
                mindist = d;
                minidx = i;
            }
        }
    }
    
    if minidx != heap.state_data.sectors.len() {
        cache.uidata.galaxy_hover_x = heap.state_data.sectors[minidx].x;
        cache.uidata.galaxy_hover_y = heap.state_data.sectors[minidx].y;
        update_galaxy_zoom(cache, world);
        cache.uidata.curr_sector = minidx as u8;
        world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, true);
    
    } else {
        cache.uidata.curr_sector = HOVER_SECTOR_INVALID;
        world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, false);
    }
}


pub fn update_galaxy_zoom(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    if 1 == cache.uidata.zoom_level {
        cache.uidata.zoom_cam_x = 960.0 / 2.0;
        cache.uidata.zoom_cam_y = 540.0 / 2.0;
    }

    let r = 540.0 / 960.0;
    let z = get_cam_zoom_z(cache);
    let sx = z * 960.0;
    let sy = sx * r;
    
    let cx = SCREEN_XRES_HALF as f32 + (960.0 / 2.0 - cache.uidata.zoom_cam_x) * z;
    let cy = SCREEN_YRES_HALF as f32 + (540.0 / 2.0 - cache.uidata.zoom_cam_y) * z;

    let mut x0 = cx - sx / 2.0;
    let mut y0 = cy - sy / 2.0;

    if x0 + sx < SCREEN_XRES as f32 {
        x0 = SCREEN_XRES as f32 - sx;
    }
    if x0 > 0.0 { x0 = 0.0; }
    if y0 + sy < SCREEN_YRES as f32 {
        y0 = SCREEN_YRES as f32 - sy;
    }
    if y0 > 0.0 { y0 = 0.0; }

    world.entity_set_position_xy(cache.uidata.galaxy_map as usize, x0, y0);
    world.entity_set_scale_xy(cache.uidata.galaxy_map as usize, sx, sy);
    
    world.entity_set_position_xy(cache.uidata.galaxy_geom as usize, x0, y0);
    world.entity_set_scale_xy(cache.uidata.galaxy_geom as usize, sx, sy);

    world.entity_set_position_xy(cache.uidata.galaxy_territory as usize, x0, y0);
    world.entity_set_scale_xy(cache.uidata.galaxy_territory as usize, sx, sy);

    world.entity_set_position_xy(cache.uidata.galaxy_path as usize, x0, y0);
    world.entity_set_scale_xy(cache.uidata.galaxy_path as usize, sx, sy);

    world.entity_set_position_xy(cache.uidata.galaxy_hover_ent as usize, x0 + cache.uidata.galaxy_hover_x * z, y0 + cache.uidata.galaxy_hover_y * z);
    world.entity_set_scale_xy(cache.uidata.galaxy_hover_ent as usize, z, z);

}


fn get_cash_string(qty: i32) -> String {
    
    let ret = format!("{}", qty);
    
    let n = if qty < 0 { ret.len() - 1 } else { ret.len() };
    
    let mut s = format!("");
    for i in 0..n {
        let idx = ret.len() - 1 - i;
        s = format!("{}{}", ret.as_bytes()[idx] as char, s);
        if 2 == i % 3 && i < n-1 {
            s = format!(",{s}");
        }
    }

    if qty < 0 {
        s = format!("-{s}");
    }
    
    // todo remove this print
    println!("get cash {qty}, {s}");
    format!("{}", s)
}

pub fn event(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {
    let mut blown = false;
    let mut clear = false;

    if mgfw::EVENT_INPUT_KEYBOARD_RELEASED_GRAVE == event_id {
        //cache.quit_requested = true;
    }
    
    // no op while ship is animating
    if heap.state_data.ships[0].animating { return false; }

    if MENU_GALAXY_LOCATION == cache.uidata.curr_menu || MENU_GALAXY_NAVIGATE == cache.uidata.curr_menu || MENU_NONE == cache.uidata.curr_menu || MENU_QUIT_CONFIRM == cache.uidata.curr_menu {
        if mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE == event_id {
            clear = true;

        } else if mgfw::EVENT_INPUT_MOUSE_WHEEL_UP == event_id {
            if cache.uidata.zoom_level < 20 {
                cache.uidata.zoom_level += 1;
                update_galaxy_zoom(cache, world);
                clear = true;

                for i in 0..8 {
                    let e = cache.uidata.galaxy_ship_entities as usize + i;
                    if cache.uidata.zoom_level > 5 {
                        world.entity_set_billboard(e, String::from("assets/player_freighter.png"));
                        world.entity_set_scale_xy(e, 80.0, 48.0);
                    } else {
                        world.entity_set_billboard(e, String::from("assets/freighter_sm.png"));
                        world.entity_set_scale_xy(e, 40.0, 24.0);
                    }
                }
            }
        
        } else if mgfw::EVENT_INPUT_MOUSE_WHEEL_DOWN == event_id {
            if cache.uidata.zoom_level > 1 {
                cache.uidata.zoom_level -= 1;
                update_galaxy_zoom(cache, world);
                clear = true;

                for i in 0..8 {
                    let e = cache.uidata.galaxy_ship_entities as usize + i;
                    if cache.uidata.zoom_level > 5 {
                        world.entity_set_billboard(e, String::from("assets/player_freighter.png"));
                        world.entity_set_scale_xy(e, 80.0, 48.0);
                    } else {
                        world.entity_set_billboard(e, String::from("assets/freighter_sm.png"));
                        world.entity_set_scale_xy(e, 40.0, 24.0);
                    }
                }
            }
        } else if mgfw::EVENT_INPUT_KEYBOARD_PRESSED_SPACE == event_id {
            cache.uidata.panning = true;
            cache.uidata.prev_pan_x = world.mouse_x;
            cache.uidata.prev_pan_y = world.mouse_y;
            clear = true;
        
        } else if mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE == event_id {
            cache.uidata.panning = false;
            clear = true;

        } else if mgfw::EVENT_INPUT_MOUSE_BUTTON_UP == event_id {
            if cache.uidata.is_menu_open && !cache.uidata.menu_grid.is_inside(world.mouse_x, world.mouse_y) {
                clear = true;

            } else if !cache.uidata.is_menu_open {
                update_galaxy_hover_selector(cache, heap, world);
                if cache.uidata.curr_sector != HOVER_SECTOR_INVALID {
                    let (sx, sy) = get_galaxy_to_screen_xy(cache, world, cache.uidata.galaxy_hover_x, cache.uidata.galaxy_hover_y);
                    cache.uidata.screen_hover_x = sx;
                    cache.uidata.screen_hover_y = sy;

                    build_galaxy_ui_popup(cache, heap, world);
                    blown = true;
                } else {
                    clear = true;
                }
            }
        }

        if clear {
            if MENU_NONE != cache.uidata.curr_menu {
                world.ugui.clear();
                build_galaxy_ui_year(cache, world);
                build_galaxy_ui_quit(cache, world);
                cache.uidata.is_menu_open = false;
                world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
                world.entity_set_visibility(cache.uidata.galaxy_path as usize, false);
                cache.uidata.curr_menu = MENU_NONE;
                cache.uidata.curr_sector = cache.state_cache.location;
            }
        }

    } else if MENU_BATTLE_BASE <= cache.uidata.curr_menu && MENU_BATTLE_ENEMY_OFFERS_TRIBUTE >= cache.uidata.curr_menu {
        event_battle(cache, heap, world, event_id);
        return false;
    }

    if (mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE == event_id) || (!clear && cache.uidata.is_menu_open && mgfw::EVENT_INPUT_MOUSE_BUTTON_UP == event_id && !cache.uidata.menu_grid.is_inside(world.mouse_x, world.mouse_y)) {
        match cache.uidata.curr_menu {
            MENU_PLANET_MAIN | MENU_STATION_MAIN | MENU_QUIT_CONFIRM | MENU_GALAXY_FREIGHTER_STATUS => {
                cache.uidata.curr_sector = cache.state_cache.location;
                build_galaxy_ui_popup(cache, heap, world);
            }
            MENU_PLANET_CARGO | MENU_PLANET_HANGAR | MENU_PLANET_MANUFACTURING | MENU_PLANET_MILITARY | MENU_PLANET_POPULATION | MENU_PLANET_RESEARCH => build_planet_ui_main(cache, heap, world),
            MENU_STATION_CARGO | MENU_STATION_NAV | MENU_STATION_TRADE | MENU_STATION_HANGAR | MENU_STATION_TERRALINK | MENU_STATION_MARKETLINK | MENU_STATION_HOUSES => build_station_ui_main(cache, heap, world),
            _ => (),
        }
    }

    blown
}

// called at 1200 hz
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {

    let mut expect_blown = false;

    if cache.uidata.panning {
        let z = get_cam_zoom_z(cache);
        let dx = (world.mouse_x - cache.uidata.prev_pan_x) as f32 / z;
        let dy = (world.mouse_y - cache.uidata.prev_pan_y) as f32 / z;
        cache.uidata.prev_pan_x = world.mouse_x;
        cache.uidata.prev_pan_y = world.mouse_y;
        cache.uidata.zoom_cam_x -= dx;
        cache.uidata.zoom_cam_y -= dy;
        update_galaxy_zoom(cache, world);
    
    } else {
        if 1 == cache.frame % 64 && MENU_NONE == cache.uidata.curr_menu {
            // !cache.uidata.is_menu_open {
            update_galaxy_hover_selector(cache, heap, world);
        }
    }

    if MENU_GALAXY_LOCATION == cache.uidata.curr_menu || MENU_GALAXY_NAVIGATE == cache.uidata.curr_menu || MENU_NONE == cache.uidata.curr_menu {
        galaxy::update(cache, heap, world);
    
    } else if MENU_BATTLE_BASE <= cache.uidata.curr_menu && MENU_BATTLE_ENEMY_OFFERS_TRIBUTE >= cache.uidata.curr_menu {
        expect_blown |= battle::update(cache, heap, world);
    
    } else if MENU_STATION_HOUSES == cache.uidata.curr_menu {
        if cache.uidata.ui_station_houses_reconstruct_avatars {
            cache.uidata.ui_station_houses_reconstruct_avatars = false;
            for i in 0..10 {
                let gidx = cache.uidata.ui_station_house_base as usize + i * 8;
                world.ugui.force_reconstruct(gidx+2);
            }
        }
    }
    

    if let Some(c) = world.ugui.pop_callback() {

        println!("received callback {}", c.id);

        let mut success = false;
        success |= check_callback_galaxy(cache, heap, world, &c);
        success |= check_callback_planet(cache, heap, world, &c);
        success |= check_callback_station(cache, heap, world, &c);
        success |= check_callback_battle(cache, heap, world, &c);

        if CALLBACK_QUIT_GAME == c.id {
            success = true;
            cache.quit_requested = true;
        }
        
        if !success {
            println!("ERROR - invalid callback received {}", c.id);
        }
    }

    expect_blown
}