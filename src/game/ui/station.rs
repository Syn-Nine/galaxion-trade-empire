use crate::{mgfw::ecs::ugui::Callback, game::state::HOUSE_PLAYER};
use super::*;


pub fn build_station_ui_main(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    // update territorial boundaries on the galaxy map when visiting stations
    state::update_galaxy_geometry(cache, heap, world);

    cache.uidata.curr_menu = MENU_STATION_MAIN;
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, false);
    world.entity_set_visibility(cache.uidata.planet_menu_tilemap as usize, true);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    cache.uidata.is_menu_open = true;

    for i in 0..heap.state_data.ships.len() {
        let e = cache.uidata.galaxy_ship_entities as usize + i;
        world.entity_set_visibility(e, false);
    }

    world.entity_set_tileset(cache.uidata.station_menu_tileset as usize, String::from("assets/station.png"), 960, 540, 960, 540);
    world.entity_set_tilemap(cache.uidata.planet_menu_tilemap as usize, cache.uidata.station_menu_tileset as usize, 1, &vec![1]);

    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.4, 0.6]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.20, 0.80]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {}", cache.state_cache.location as usize)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();

    // options
    let subgrid = vsplit[1].tab_pad();
    let vsplit = subgrid.split_even(SPLIT_V, 11);
    
    // add some vertical padding to the buttons
    let mut vsplit_pad: Vec::<UIgrid> = Vec::new();
    for i in 0..vsplit.len() {
        let mut g = vsplit[i].clone();
        g.y0 += 1.0;
        g.y1 -= 1.0;
        vsplit_pad.push(g);
    }

    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[0], &format!("Station")).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[1], &format!("Trade")).callback(CALLBACK_STATION_MAIN_TRADE));
    //world.ugui.add(menu_tab, ugui::button(&vsplit_pad[2], &format!("Bank")).callback(CALLBACK_STATION_MAIN_BANK).enabled(false));
    //world.ugui.add(menu_tab, ugui::button(&vsplit_pad[3], &format!("Bounties")).callback(CALLBACK_STATION_MAIN_BOUNTIES).enabled(false));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[2], &format!("Terra Link")).callback(CALLBACK_STATION_MAIN_TERRALINK));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[3], &format!("Market Link")).callback(CALLBACK_STATION_MAIN_MARKETLINK));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[4], &format!("Nav Center")).callback(CALLBACK_STATION_MAIN_NAV).enabled(!cache.uidata.all_nav_known));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[5], &format!("Houses")).callback(CALLBACK_STATION_MAIN_HOUSES));
    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[6], &format!("Freighter")).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[7], &format!("Cargo")).callback(CALLBACK_STATION_MAIN_CARGO));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[8], &format!("Hangar Bay")).callback(CALLBACK_STATION_MAIN_HANGAR));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[10], &format!("Lift Off")).callback(CALLBACK_STATION_MAIN_LIFTOFF));


}


fn build_station_ui_cargo(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_CARGO;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.25, 0.75]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {}", cache.state_cache.location)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 11);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Status of Ship Cargo Holds")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cargo:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Qty:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Holds:")).halign(HALIGN_RIGHT).highlight(true));

    let hold_colonists = heap.state_data.ships[0].hold_colonists;
    let hold_bios = heap.state_data.ships[0].hold_bios;
    let hold_fuel = heap.state_data.ships[0].hold_fuel;
    let hold_ice = heap.state_data.ships[0].hold_ice;
    let hold_mach = heap.state_data.ships[0].hold_mach;
    let hold_meds = heap.state_data.ships[0].hold_meds;
    //
    let ratio_colonists = 0.005;
    let ratio_bios = 0.003;
    let ratio_ice = 0.004;
    let ratio_fuel = 0.002;
    let ratio_mach = 0.003;
    let ratio_meds = 0.001;

    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_colonists)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_colonists as f32 * ratio_colonists)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[3].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_bios)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_bios as f32 * ratio_bios)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[4].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_fuel)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_fuel as f32 * ratio_fuel)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[5].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_ice)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_ice as f32 * ratio_ice)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[6].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_mach)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_mach as f32 * ratio_mach)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[7].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_meds)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{:.2}", hold_meds as f32 * ratio_meds)).halign(HALIGN_RIGHT)) as u8;
    //
    world.ugui.add(menu_tab, ugui::label(&subsplit[8], &format!("Total: {:.2}/{}", hold_tot, heap.state_data.ships[0].holds)).halign(HALIGN_RIGHT)) as u8;


    subsplit[10].y0 = subsplit[10].y1 - 22.0;
    world.ugui.add(menu_tab, ugui::button(&subsplit[10], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));

}


fn build_station_ui_terralink(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_TERRALINK;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.25, 0.75]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.17, 0.83]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {} - Status of House Planets", cache.state_cache.location)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 13);

    // top half
    let hsplit = subsplit[0].split_vec(SPLIT_H, vec![0.10, 0.25, 0.37, 0.52, 0.84]);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Sct:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Name:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Class:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Pop:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("Researching:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("Hangar:")).halign(HALIGN_RIGHT).highlight(true));

    cache.uidata.ui_station_terralink_table_offset = 0; // start at 0
    let pplanets = state::get_player_planet_vec(heap);
    let nrows = pplanets.len();
    
    // make base table
    for i in 0..10 {
        subsplit[i+1].y0 -= 3.0;
        subsplit[i+1].y1 -= 4.0;
        let mut hsplit = subsplit[i+1].split_vec(SPLIT_H, vec![0.10, 0.30, 0.37, 0.52, 0.84]);
        hsplit[0].x1 -= 6.0;
        let gidx = world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("")).halign(HALIGN_LEFT).visible(false)) as u8;
        if 0 == i { cache.uidata.ui_station_terralink_table = gidx; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("")).halign(HALIGN_LEFT));
        world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("")).halign(HALIGN_RIGHT));
        world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("")).halign(HALIGN_RIGHT));
        world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("")).halign(HALIGN_RIGHT));
        world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("")).halign(HALIGN_RIGHT));
    }

    // populate
    let start = cache.uidata.ui_station_terralink_table_offset as usize;
    for i in 0..10 {

        if start + i >= nrows { break; }
            
        let pidx = pplanets[start + i];

        let name = heap.state_data.planet_names[pidx].clone();
        let pop = heap.state_data.sectors[pidx].planet_data.cur_pop;
        let research = planet::get_research_name(heap.state_data.sectors[pidx].planet_data.researching);
        let fighters = heap.state_data.sectors[pidx].planet_data.fighters;        
        let class = format!("{}", match heap.state_data.sectors[pidx].planet_class {
            PLANET_CLASS_DESERT => "D",
            PLANET_CLASS_LUSH => "L",
            PLANET_CLASS_VOLCANO => "V",
            PLANET_CLASS_WATER => "W",
            _ => ""
        });

        let gidx = cache.uidata.ui_station_terralink_table as usize + i * 6;
        world.ugui.set_text(gidx+0, &format!("{}", pidx));
        world.ugui.set_text(gidx+1, &format!("{}", name));
        world.ugui.set_text(gidx+2, &format!("{}", class));
        world.ugui.set_text(gidx+3, &format!("{}", pop));
        world.ugui.set_text(gidx+4, &format!("{}", research));
        world.ugui.set_text(gidx+5, &format!("{}", fighters));

        world.ugui.set_visible(gidx as usize, true);
        world.ugui.set_callback(gidx as usize, ugui::Callback { enabled: true, id: CALLBACK_STATION_NAV_TO, idata0: pidx as i32, ..Default::default() });
    }

    subsplit[11].y0 -= 3.0;
    subsplit[11].y1 -= 1.0;
    subsplit[12].y1 += 1.0;
    let mut hsplit = subsplit[11].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    cache.uidata.ui_station_terralink_pgup = world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Page Up")).callback(CALLBACK_STATION_TERRALINK_PAGEUP).enabled(false)) as u8;
    let enabled = if 9 < nrows { true } else { false };
    cache.uidata.ui_station_terralink_pgdn = world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Page Down")).callback(CALLBACK_STATION_TERRALINK_PAGEDOWN).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::button(&subsplit[12], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));

}


fn build_station_ui_marketlink(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_TERRALINK;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    let main = grid.split_vec(SPLIT_H, vec![0.11, 0.42, 0.89]);

    //////////////////////////////////////////////
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.25, 0.75]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Status of Ship Cargo Holds")).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 10);

    // top half
    let hsplit = subsplit[0].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cargo:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Qty:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("EPH:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Holds:")).halign(HALIGN_RIGHT).highlight(true));
    
    let hold_colonists = heap.state_data.ships[0].hold_colonists;
    let hold_bios = heap.state_data.ships[0].hold_bios;
    let hold_fuel = heap.state_data.ships[0].hold_fuel;
    let hold_ice = heap.state_data.ships[0].hold_ice;
    let hold_mach = heap.state_data.ships[0].hold_mach;
    let hold_meds = heap.state_data.ships[0].hold_meds;
    //
    let ratio_colonists = 0.005;
    let ratio_bios = 0.003;
    let ratio_ice = 0.004;
    let ratio_fuel = 0.002;
    let ratio_mach = 0.003;
    let ratio_meds = 0.001;
    //
    let eff_price_bios = heap.state_data.ships[0].effective_price_bios as i32;
    let eff_price_fuel = heap.state_data.ships[0].effective_price_fuel as i32;
    let eff_price_ice = heap.state_data.ships[0].effective_price_ice as i32;
    let eff_price_mach = heap.state_data.ships[0].effective_price_mach as i32;
    let eff_price_meds = heap.state_data.ships[0].effective_price_meds as i32;

    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

    //
    let hsplit = subsplit[1].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_colonists)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("N/A")).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_colonists as f32 * ratio_colonists)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_bios)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", eff_price_bios)).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_bios as f32 * ratio_bios)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[3].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_fuel)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", eff_price_fuel)).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_fuel as f32 * ratio_fuel)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[4].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_ice)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", eff_price_ice)).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_ice as f32 * ratio_ice)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[5].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_mach)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", eff_price_mach)).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_mach as f32 * ratio_mach)).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[6].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_meds)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", eff_price_meds)).halign(HALIGN_RIGHT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_meds as f32 * ratio_meds)).halign(HALIGN_RIGHT)) as u8;
    //
    world.ugui.add(menu_tab, ugui::label(&subsplit[7], &format!("Total: {:.2}/{}", hold_tot, heap.state_data.ships[0].holds)).halign(HALIGN_RIGHT)) as u8;


    subsplit[9].y0 = subsplit[9].y1 - 22.0;
    world.ugui.add(menu_tab, ugui::button(&subsplit[9], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));


    //////////////////////////////////////////////

    let vsplit = main[2].split_vec(SPLIT_V, vec![0.18, 0.82]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Status of Station Market Prices")).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 12);

    // top half
    let hsplit = subsplit[0].split_vec(SPLIT_H, vec![0.1, 0.28, 0.46, 0.64, 0.82]);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Sector:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Bios:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Fuel:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Ice:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("Mach:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("Meds:")).halign(HALIGN_RIGHT).highlight(true));

    cache.uidata.ui_station_marketlink_table_offset = 0; // start at 0
    
    // make base table
    for i in 0..10 {
        subsplit[i+1].y0 -= 3.0;
        subsplit[i+1].y1 -= 4.0;
        let mut hsplit = subsplit[i+1].split_vec(SPLIT_H, vec![0.12, 0.28, 0.46, 0.64, 0.82]);
        let gidx = world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("")).halign(HALIGN_LEFT).visible(false)) as u8;
        if 0 == i { cache.uidata.ui_station_marketlink_table = gidx; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("")).halign(HALIGN_RIGHT).highlight(false).enabled(true));
        world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("")).halign(HALIGN_RIGHT).highlight(false).enabled(true));
        world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("")).halign(HALIGN_RIGHT).highlight(false).enabled(true));
        world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("")).halign(HALIGN_RIGHT).highlight(false).enabled(true));
        world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("")).halign(HALIGN_RIGHT).highlight(false).enabled(true));
    }

    subsplit[11].y0 -= 2.0;
    let mut hsplit = subsplit[11].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    cache.uidata.ui_station_marketlink_pgup = world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Page Up")).callback(CALLBACK_STATION_MARKETLINK_PAGEUP).enabled(false)) as u8;
    cache.uidata.ui_station_marketlink_pgdn = world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Page Down")).callback(CALLBACK_STATION_MARKETLINK_PAGEDOWN).enabled(false)) as u8;

    // populate
    populate_marketlink(cache, heap, world);


}

fn populate_marketlink(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    let svec = state::get_known_station_vec(heap);
    let nrows = svec.len();

    let mut bmax_bios = -1;
    let mut smin_bios = -1;
    let mut bmax_fuel = -1;
    let mut smin_fuel = -1;
    let mut bmax_ice = -1;
    let mut smin_ice = -1;
    let mut bmax_mach = -1;
    let mut smin_mach = -1;
    let mut bmax_meds = -1;
    let mut smin_meds = -1;
    
    for i in 0..nrows {
        let pidx = svec[i];

        let bios = heap.state_data.sectors[pidx].station_data.trade_price_bios;
        let fuel = heap.state_data.sectors[pidx].station_data.trade_price_fuel;
        let ice = heap.state_data.sectors[pidx].station_data.trade_price_ice;
        let mach = heap.state_data.sectors[pidx].station_data.trade_price_mach;
        let meds = heap.state_data.sectors[pidx].station_data.trade_price_meds;

        if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_bios {
            if bmax_bios == -1 || bios > bmax_bios { bmax_bios = bios; }
        } else {
            if smin_bios == -1 || bios < smin_bios { smin_bios = bios; }
        }

        if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_fuel {
            if bmax_fuel == -1 || fuel > bmax_fuel { bmax_fuel = fuel; }
        } else {
            if smin_fuel == -1 || fuel < smin_fuel { smin_fuel = fuel; }
        }

        if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_ice {
            if bmax_ice == -1 || ice > bmax_ice { bmax_ice = ice; }
        } else {
            if smin_ice == -1 || ice < smin_ice { smin_ice = ice; }
        }

        if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_mach {
            if bmax_mach == -1 || mach > bmax_mach { bmax_mach = mach; }
        } else {
            if smin_mach == -1 || mach < smin_mach { smin_mach = mach; }
        }

        if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_meds {
            if bmax_meds == -1 || meds > bmax_meds { bmax_meds = meds; }
        } else {
            if smin_meds == -1 || meds < smin_meds { smin_meds = meds; }
        }

    }

    let start = cache.uidata.ui_station_marketlink_table_offset as usize;
    for i in 0..10 {

        let gidx = cache.uidata.ui_station_marketlink_table as usize + i * 6;

        // turn off all callbacks
        world.ugui.set_visible(gidx as usize, false);
        world.ugui.set_callback(gidx as usize, ugui::Callback { enabled: false, id: CALLBACK_INVALID, ..Default::default() });
        
        if start + i >= nrows {
            for j in 0..6 {
                world.ugui.set_text(gidx+j, &format!(""));
                world.ugui.set_highlight(gidx+j, false);
                world.ugui.set_enabled(gidx+j, true);
            }
            continue;
        }
            
        let pidx = svec[start + i];

        let bios = heap.state_data.sectors[pidx].station_data.trade_price_bios;
        let fuel = heap.state_data.sectors[pidx].station_data.trade_price_fuel;
        let ice = heap.state_data.sectors[pidx].station_data.trade_price_ice;
        let mach = heap.state_data.sectors[pidx].station_data.trade_price_mach;
        let meds = heap.state_data.sectors[pidx].station_data.trade_price_meds;

        let sbios = format!("{}", if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_bios { "B" } else { "S" });
        let sfuel = format!("{}", if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_fuel { "B" } else { "S" });
        let sice = format!("{}", if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_ice { "B" } else { "S" });
        let smach = format!("{}", if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_mach { "B" } else { "S" });
        let smeds = format!("{}", if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_meds { "B" } else { "S" });
        
        if pidx == cache.state_cache.location as usize {
            world.ugui.set_text(gidx+0, &format!("{}", pidx));
            world.ugui.set_enabled(gidx+0, false);
        } else {
            world.ugui.set_text(gidx+0, &format!("{}", pidx));
            world.ugui.set_enabled(gidx+0, true);
        }
        
        world.ugui.set_text(gidx+1, &format!("{} {}", bios, sbios));
        let highlight = if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_bios && bios == bmax_bios { true } else { false };
        world.ugui.set_highlight(gidx+1, highlight);
        let enabled = if state::STATION_SELLING == heap.state_data.sectors[pidx].station_data.buysell_bios && bios == smin_bios { false } else { true };
        world.ugui.set_enabled(gidx+1, enabled);
        
        world.ugui.set_text(gidx+2, &format!("{} {}", fuel, sfuel));
        let highlight = if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_fuel && fuel == bmax_fuel { true } else { false };
        world.ugui.set_highlight(gidx+2, highlight);
        let enabled = if state::STATION_SELLING == heap.state_data.sectors[pidx].station_data.buysell_fuel && fuel == smin_fuel { false } else { true };
        world.ugui.set_enabled(gidx+2, enabled);
        
        world.ugui.set_text(gidx+3, &format!("{} {}", ice, sice));
        let highlight = if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_ice && ice == bmax_ice { true } else { false };
        world.ugui.set_highlight(gidx+3, highlight);
        let enabled = if state::STATION_SELLING == heap.state_data.sectors[pidx].station_data.buysell_ice && ice == smin_ice { false } else { true };
        world.ugui.set_enabled(gidx+3, enabled);
        
        world.ugui.set_text(gidx+4, &format!("{} {}", mach, smach));
        let highlight = if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_mach && mach == bmax_mach { true } else { false };
        world.ugui.set_highlight(gidx+4, highlight);
        let enabled = if state::STATION_SELLING == heap.state_data.sectors[pidx].station_data.buysell_mach && mach == smin_mach { false } else { true };
        world.ugui.set_enabled(gidx+4, enabled);
        
        world.ugui.set_text(gidx+5, &format!("{} {}", meds, smeds));
        let highlight = if state::STATION_BUYING == heap.state_data.sectors[pidx].station_data.buysell_meds && meds == bmax_meds { true } else { false };
        world.ugui.set_highlight(gidx+5, highlight);
        let enabled = if state::STATION_SELLING == heap.state_data.sectors[pidx].station_data.buysell_meds && meds == smin_meds { false } else { true };
        world.ugui.set_enabled(gidx+5, enabled);

        world.ugui.set_visible(gidx as usize, true);
        world.ugui.set_callback(gidx as usize, ugui::Callback { enabled: true, id: CALLBACK_STATION_NAV_TO, idata0: pidx as i32, ..Default::default() });
        
    }

    let enabled = if 0 == start { false } else { true };
    world.ugui.set_enabled(cache.uidata.ui_station_marketlink_pgup as usize, enabled);

    let enabled = if start + 9 < nrows { true } else { false };
    world.ugui.set_enabled(cache.uidata.ui_station_marketlink_pgdn as usize, enabled);

}


fn build_station_ui_houses(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_HOUSES;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.27, 0.73]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.06, 0.94]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {} - Known Royal Houses", cache.state_cache.location)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_station_tab = menu_tab as u8;

    let mut vsplit = vsplit[1].tab_pad().split_vec(SPLIT_V, vec![0.9, 0.95]);
    vsplit[0].y1 -= 3.0;

    let subsplit = vsplit[0].split_even(SPLIT_V, 5);

    for i in 0..10 {
        let col = i % 2;
        let row = (i - col) / 2;

        let mut hsplit = subsplit[row].split_even(SPLIT_H, 2);
        hsplit[0].x1 -= 1.0;
        hsplit[1].x0 += 1.0;

        let mut temp = hsplit[col].clone();
        temp.y1 -= 1.0;
        let e = world.ugui.add(menu_tab, ugui::panel(&temp)) as u8;
        if 0 == i { cache.uidata.ui_station_house_base = e; }
        
        temp.y0 += 2.0;
        temp.x1 = temp.x0 + 72.0;
        temp.y0 = temp.y1 - 72.0;
        let enemy_avatar = heap.state_data.house_avatars[0].clone();
        let tile_set = world.ugui.add(ugui::PARENT_WINDOW, ugui::tileset(format!("assets/avatars_sm/{}", enemy_avatar), 0, 0));
        let t = world.ugui.add(ugui::PARENT_WINDOW, ugui::tilemap(&temp.pad(), tile_set, 1, &vec![1])) as u8;

        temp.x0 = temp.x1 + 2.0;
        temp.x1 = hsplit[col].x1 - 6.0;
        temp.y0 += 6.0;
        temp.y1 = temp.y0 + 16.0;
        world.ugui.add(menu_tab, ugui::label(&temp, &format!("House {}", heap.state_data.house_names[0].clone())).halign(HALIGN_LEFT).highlight(true));

        temp.y0 = temp.y1 + 0.0;
        temp.y1 = temp.y0 + 16.0;
        let htemp = temp.split(SPLIT_H, 0.6);
        world.ugui.add(menu_tab, ugui::label(&htemp[0], &format!("Territories:")).halign(HALIGN_LEFT));
        world.ugui.add(menu_tab, ugui::label(&htemp[1], &format!("")).halign(HALIGN_RIGHT));

        temp.y0 = temp.y1 + 0.0;
        temp.y1 = temp.y0 + 16.0;
        let htemp = temp.split(SPLIT_H, 0.6);
        world.ugui.add(menu_tab, ugui::label(&htemp[0], &format!("Planets:")).halign(HALIGN_LEFT));
        world.ugui.add(menu_tab, ugui::label(&htemp[1], &format!("")).halign(HALIGN_RIGHT));

    }

    cache.uidata.ui_station_house_table_offset = 0;

    vsplit[1].y0 -= 3.0;
    vsplit[1].y1 -= 1.0;
    vsplit[2].y1 += 1.0;
    let mut hsplit = vsplit[1].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    cache.uidata.ui_station_houses_pgup = world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Page Up")).callback(CALLBACK_STATION_HOUSES_PGUP).enabled(false).callback(CALLBACK_STATION_HOUSES_PGUP)) as u8;
    cache.uidata.ui_station_houses_pgdn = world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Page Down")).callback(CALLBACK_STATION_HOUSES_PGDN).enabled(true).callback(CALLBACK_STATION_HOUSES_PGDN)) as u8;
    world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));

    populate_house_table(cache, heap, world);

    cache.uidata.ui_station_houses_reconstruct_avatars = true;

}

fn populate_house_table(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    
    // make house list
    let mut houses: Vec<usize> = Vec::new();
    houses.push(HOUSE_PLAYER as usize);
    for i in 1..heap.state_data.last_house {
        for j in 0..heap.state_data.sectors.len() {
            if heap.state_data.sectors[j].known && heap.state_data.sectors[j].house == i+1 {
                houses.push(i as usize + 1);
                break;
            }
        }
    }

    for i in 0..heap.state_data.dead_houses.len() {
        houses.push(heap.state_data.dead_houses[i] as usize);
    }
    
    for i in 0..10 {
        let gidx = cache.uidata.ui_station_house_base as usize + i * 8;

        for k in 0..8 {
            world.ugui.set_visible(gidx+k, false);
        }
        
        let hidx = cache.uidata.ui_station_house_table_offset as usize + i;
        if hidx >= houses.len() { continue; }

        for k in 0..8 {
            world.ugui.set_visible(gidx+k, true);
        }

        let nplanets = state::get_num_planets_house(heap, houses[hidx] as u8);
        let enabled = if 0 < nplanets { true } else { false };

        world.ugui.set_highlight(gidx+3, enabled);
        world.ugui.set_enabled(gidx+3, enabled);
        world.ugui.set_enabled(gidx+4, enabled);
        world.ugui.set_enabled(gidx+5, enabled);
        world.ugui.set_enabled(gidx+6, enabled);
        world.ugui.set_enabled(gidx+7, enabled);

        let enemy_avatar = heap.state_data.house_avatars[houses[hidx]].clone();
        world.ugui.set_tileset(gidx+1, format!("assets/avatars_sm/{}", enemy_avatar), 0, 0);
        world.ugui.set_text(gidx+3, &format!("House {}", heap.state_data.house_names[houses[hidx]].clone()));
        world.ugui.set_text(gidx+5, &format!("{}", state::get_num_territories_house(heap, houses[hidx] as u8)));
        world.ugui.set_text(gidx+7, &format!("{}", nplanets));

    }

    let enabled = if 0 == cache.uidata.ui_station_house_table_offset { false } else { true };
    world.ugui.set_enabled(cache.uidata.ui_station_houses_pgup as usize, enabled);

    let enabled = if cache.uidata.ui_station_house_table_offset as usize + 10 < houses.len() { true } else { false };
    world.ugui.set_enabled(cache.uidata.ui_station_houses_pgdn as usize, enabled);

    println!("total houses: {}", houses.len());

}

fn build_station_ui_hangar(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_HANGAR;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.3, 0.7]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.3, 0.7]);

    let sidx = cache.state_cache.location as usize;
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {}", cache.state_cache.location)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_station_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 8);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Status of Ship Hangar")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Unit:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("In Hanger:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("On Station:")).halign(HALIGN_RIGHT).highlight(true));
    
    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.ships[0].fighters)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_station_hangar_qty_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].station_data.merc_available)).halign(HALIGN_RIGHT)) as u8;

    
    let cash_avail = cache.state_cache.cash;
    let merc_avail = heap.state_data.sectors[sidx].station_data.merc_available as i32;
    let merc_price = heap.state_data.sectors[sidx].station_data.merc_price;

    let scroller_enabled = if cash_avail >= merc_price && 0 < merc_avail { true } else { false };
    cache.uidata.ui_station_hangar_slider_idx = world.ugui.add(menu_tab, ugui::slider(&subsplit[3], 0.0).callback(CALLBACK_STATION_HANGAR_SLIDER).highlight(true).enabled(scroller_enabled)) as u8;
    //

    cache.state_cache.merc_to_buy = 0;

    let hsplit = subsplit[4].split_even(SPLIT_H, 2);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Mercenary Cost: {merc_price}/ea")).halign(HALIGN_LEFT));
    cache.uidata.ui_station_hangar_total_cost = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Total: 0")).halign(HALIGN_RIGHT)) as u8;

    cache.uidata.ui_station_hangar_cash_after_hire = world.ugui.add(menu_tab, ugui::label(&subsplit[5], &format!("Net: {}", get_cash_string(cash_avail))).halign(HALIGN_RIGHT)) as u8;

    subsplit[7].y0 = subsplit[7].y1 - 22.0;
    let mut hsplit = subsplit[7].split_even(SPLIT_H, 3);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    hsplit[1].x1 -= 1.0;
    hsplit[2].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));
    cache.uidata.ui_station_hangar_hire_button = world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Hire")).callback(CALLBACK_STATION_HIRE_FIGHTERS).enabled(false)) as u8;
    world.ugui.add(menu_tab, ugui::button(&hsplit[2], &format!("Take Max")).callback(CALLBACK_STATION_TAKE_MAX_FIGHTERS).enabled(scroller_enabled)) as u8;

}


fn build_station_ui_nav(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_STATION_NAV;
    
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.4, 0.6]);

    let mut vsplit = main[1].clone();
    let mut nops = 1;

    let mut nearby: Vec<usize> = Vec::new();
    let sidx = cache.state_cache.location as usize;

    if 0 == heap.state_data.sectors[sidx].station_data.nav_init {
        nearby = state::find_local_stations(heap, sidx);
        if 1 <= nearby.len() {
            heap.state_data.sectors[sidx].station_data.nav_path_0_idx = nearby[0];
            heap.state_data.sectors[sidx].station_data.nav_init = 1;
            if 2 <= nearby.len() {
                heap.state_data.sectors[sidx].station_data.nav_path_1_idx = nearby[1];
                heap.state_data.sectors[sidx].station_data.nav_init = 2;
            }
        }
    } else {
        if 1 <= heap.state_data.sectors[sidx].station_data.nav_init {
            nearby.push(heap.state_data.sectors[sidx].station_data.nav_path_0_idx);
            if 2 <= heap.state_data.sectors[sidx].station_data.nav_init {
                nearby.push(heap.state_data.sectors[sidx].station_data.nav_path_1_idx);
            }
        }
    }
    nops += nearby.len() as i32;
    nops += 2;

    // do we already know about 80 percent of the sectors?
    let mut nknown = 0;
    for i in 0..heap.state_data.sectors.len() {
        if heap.state_data.sectors[i].known { nknown += 1; }
    }
    let download_all = if nknown as f32 > heap.state_data.sectors.len() as f32 * 0.8 { true } else { false };
    if download_all { nops += 1; }

    let hh = 24.0 + nops as f32 * 30.0;
    vsplit.y0 = SCREEN_YRES_HALF as f32 - hh * 0.5;
    vsplit.y1 = vsplit.y0 + hh;

    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {}", sidx)).highlight(true));
    cache.uidata.menu_grid = vsplit.clone();

    // options
    let subgrid = vsplit.tab_pad();
    let vsplit = subgrid.split_even(SPLIT_V, nops + 1);
    
    // add some vertical padding to the buttons
    let mut subsplit: Vec::<UIgrid> = Vec::new();
    for i in 0..vsplit.len() {
        let mut g = vsplit[i].clone();
        g.y0 += 1.0;
        g.y1 -= 1.0;
        subsplit.push(g);
    }

    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Download Nav Maps")).highlight(true));
    cache.uidata.station_nav_ui_local_idx = world.ugui.add(menu_tab, ugui::button(&subsplit[1], &format!("Local Sectors")).callback_i0(CALLBACK_STATION_NAV_DOWNLOAD_LOCAL, sidx as i32).enabled(!heap.state_data.sectors[sidx].station_data.nav_local)) as u8;
    for i in 0..nearby.len() {
        let mut enabled = true;
        if 0 == i && (heap.state_data.sectors[sidx].station_data.nav_path_0 || heap.state_data.sectors[nearby[i]].known) { enabled = false; }
        if 1 == i && (heap.state_data.sectors[sidx].station_data.nav_path_1 || heap.state_data.sectors[nearby[i]].known) { enabled = false; }
        let cb = if 0 == i { CALLBACK_STATION_NAV_DOWNLOAD_PATH_0 } else { CALLBACK_STATION_NAV_DOWNLOAD_PATH_1 };
        let id = world.ugui.add(menu_tab, ugui::button(&subsplit[2 + i], &format!("Path To Station {}", nearby[i])).callback_i0(cb, nearby[i] as i32).enabled(enabled)) as u8;
        if 0 == i { cache.uidata.station_nav_ui_path_0_idx = id; }
        if 1 == i { cache.uidata.station_nav_ui_path_1_idx = id; }
    }

    if download_all {
        cache.uidata.station_nav_ui_all_idx = world.ugui.add(menu_tab, ugui::button(&subsplit[nops as usize - 2], &format!("Download Galaxy Map")).callback(CALLBACK_STATION_NAV_DOWNLOAD_ALL)) as u8;
    }

    world.ugui.add(menu_tab, ugui::button(&subsplit[nops as usize], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));

}


pub fn build_station_ui_trade(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    let sidx = cache.state_cache.location as usize;

    cache.uidata.curr_menu = MENU_STATION_TRADE;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.15, 0.85]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.2, 0.8]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Station - Sector {}", sidx)).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 15);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Station Market")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 8);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Resource:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Station:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Qty:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Have:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("Holds:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("EPH:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("Price:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("Net:")).halign(HALIGN_RIGHT).highlight(true));
    

    let hold_colonists = heap.state_data.ships[0].hold_colonists;
    let hold_bios = heap.state_data.ships[0].hold_bios;
    let hold_fuel = heap.state_data.ships[0].hold_fuel;
    let hold_ice = heap.state_data.ships[0].hold_ice;
    let hold_mach = heap.state_data.ships[0].hold_mach;
    let hold_meds = heap.state_data.ships[0].hold_meds;
    let hold_resources = hold_bios + hold_fuel + hold_ice + hold_mach + hold_meds;
    //
    cache.state_cache.trade_qty_bios = 0;
    cache.state_cache.trade_qty_fuel = 0;
    cache.state_cache.trade_qty_ice = 0;
    cache.state_cache.trade_qty_mach = 0;
    cache.state_cache.trade_qty_meds = 0;
    //
    let price_bios = heap.state_data.sectors[sidx].station_data.trade_price_bios;
    let price_fuel = heap.state_data.sectors[sidx].station_data.trade_price_fuel;
    let price_ice = heap.state_data.sectors[sidx].station_data.trade_price_ice;
    let price_mach = heap.state_data.sectors[sidx].station_data.trade_price_mach;
    let price_meds = heap.state_data.sectors[sidx].station_data.trade_price_meds;
    //
    let eff_price_bios = heap.state_data.ships[0].effective_price_bios as i32;
    let eff_price_fuel = heap.state_data.ships[0].effective_price_fuel as i32;
    let eff_price_ice = heap.state_data.ships[0].effective_price_ice as i32;
    let eff_price_mach = heap.state_data.ships[0].effective_price_mach as i32;
    let eff_price_meds = heap.state_data.ships[0].effective_price_meds as i32;
    //
    let ratio_colonists = 0.005;
    let ratio_bios = 0.003;
    let ratio_ice = 0.004;
    let ratio_fuel = 0.002;
    let ratio_mach = 0.003;
    let ratio_meds = 0.001;

    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 8);
    let mut enabled = true;
    if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_bios {
        if 0 == hold_bios { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Buying")).halign(HALIGN_RIGHT).enabled(enabled));
    } else {
        if price_bios > cache.state_cache.cash { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Selling")).halign(HALIGN_RIGHT).enabled(enabled));
    }
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT).enabled(enabled));
    cache.uidata.ui_trade_bios_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}/", cache.state_cache.trade_qty_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{}", hold_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_bios_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}", cache.state_cache.trade_qty_bios as f32 * ratio_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_bios_eff_price = world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("{}", eff_price_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_bios_price = world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("{}", price_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_bios_net = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", price_bios * cache.state_cache.trade_qty_bios)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    //
    subsplit[3].y1 -= 4.0;
    cache.uidata.ui_trade_slider_bios = world.ugui.add(menu_tab, ugui::slider(&subsplit[3], 0.0).callback(CALLBACK_TRADE_SLIDER_BIOLOGICS).highlight(true)) as u8;
    //
    let hsplit = subsplit[4].split_even(SPLIT_H, 8);
    let mut enabled = true;
    if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_fuel {
        if 0 == hold_fuel { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Buying")).halign(HALIGN_RIGHT).enabled(enabled));
    } else {
        if price_fuel > cache.state_cache.cash { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Selling")).halign(HALIGN_RIGHT).enabled(enabled));
    }
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT).enabled(enabled));
    cache.uidata.ui_trade_fuel_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}/", cache.state_cache.trade_qty_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{}", hold_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_fuel_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}", cache.state_cache.trade_qty_fuel as f32 * ratio_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_fuel_eff_price = world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("{}", eff_price_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_fuel_price = world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("{}", price_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_fuel_net = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", price_fuel * cache.state_cache.trade_qty_fuel)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    //
    subsplit[5].y1 -= 4.0;
    cache.uidata.ui_trade_slider_fuel = world.ugui.add(menu_tab, ugui::slider(&subsplit[5], 0.0).callback(CALLBACK_TRADE_SLIDER_FUEL).highlight(true)) as u8;
    //
    let hsplit = subsplit[6].split_even(SPLIT_H, 8);
    let mut enabled = true;
    if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_ice {
        if 0 == hold_ice { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Buying")).halign(HALIGN_RIGHT).enabled(enabled));
    } else {
        if price_ice > cache.state_cache.cash { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Selling")).halign(HALIGN_RIGHT).enabled(enabled));
    }
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT).enabled(enabled));
    cache.uidata.ui_trade_ice_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}/", cache.state_cache.trade_qty_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{}", hold_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_ice_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}", cache.state_cache.trade_qty_ice as f32 * ratio_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_ice_eff_price = world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("{}", eff_price_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_ice_price = world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("{}", price_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_ice_net = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", price_ice * cache.state_cache.trade_qty_ice)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    
    //
    subsplit[7].y1 -= 4.0;
    cache.uidata.ui_trade_slider_ice = world.ugui.add(menu_tab, ugui::slider(&subsplit[7], 0.0).callback(CALLBACK_TRADE_SLIDER_ICE).highlight(true)) as u8;
    //
    let hsplit = subsplit[8].split_even(SPLIT_H, 8);
    let mut enabled = true;
    if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_mach {
        if 0 == hold_mach { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Buying")).halign(HALIGN_RIGHT).enabled(enabled));
    } else {
        if price_mach > cache.state_cache.cash { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Selling")).halign(HALIGN_RIGHT).enabled(enabled));
    }
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT).enabled(enabled));
    cache.uidata.ui_trade_mach_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}/", cache.state_cache.trade_qty_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{}", hold_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_mach_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}", cache.state_cache.trade_qty_mach as f32 * ratio_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_mach_eff_price = world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("{}", eff_price_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_mach_price = world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("{}", price_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_mach_net = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", price_mach * cache.state_cache.trade_qty_mach)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    
    //
    subsplit[9].y1 -= 4.0;
    cache.uidata.ui_trade_slider_mach = world.ugui.add(menu_tab, ugui::slider(&subsplit[9], 0.0).callback(CALLBACK_TRADE_SLIDER_MACHINERY).highlight(true)) as u8;
    //
    let hsplit = subsplit[10].split_even(SPLIT_H, 8);
    let mut enabled = true;
    if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_meds {
        if 0 == hold_meds { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Buying")).halign(HALIGN_RIGHT).enabled(enabled));
    } else {
        if price_meds > cache.state_cache.cash { enabled = false; }
        world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Selling")).halign(HALIGN_RIGHT).enabled(enabled));
    }
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT).enabled(enabled));
    cache.uidata.ui_trade_meds_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}/", cache.state_cache.trade_qty_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{}", hold_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_meds_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}", cache.state_cache.trade_qty_meds as f32 * ratio_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_meds_eff_price = world.ugui.add(menu_tab, ugui::label(&hsplit[5], &format!("{}", eff_price_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_meds_price = world.ugui.add(menu_tab, ugui::label(&hsplit[6], &format!("{}", price_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    cache.uidata.ui_trade_meds_net = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", price_meds * cache.state_cache.trade_qty_meds)).halign(HALIGN_RIGHT).enabled(enabled)) as u8;
    
    //
    subsplit[11].y1 -= 4.0;
    cache.uidata.ui_trade_slider_meds = world.ugui.add(menu_tab, ugui::slider(&subsplit[11], 0.0).callback(CALLBACK_TRADE_SLIDER_MEDICINE).highlight(true)) as u8;
    //
    let hsplit = subsplit[12].split_even(SPLIT_H, 8);
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Total:")).halign(HALIGN_RIGHT));
    cache.uidata.ui_trade_holds_tot = world.ugui.add(menu_tab, ugui::label(&hsplit[4], &format!("{:.2}/{}", hold_tot, heap.state_data.ships[0].holds)).halign(HALIGN_RIGHT)) as u8;
    let cash = cache.state_cache.cash;
    cache.uidata.ui_trade_cash_tot = world.ugui.add(menu_tab, ugui::label(&hsplit[7], &format!("{}", get_cash_string(cache.state_cache.cash))).halign(HALIGN_RIGHT)) as u8;

    subsplit[14].y0 = subsplit[14].y1 - 26.0;
    let mut hsplit = subsplit[14].split_even(SPLIT_H, 5);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    hsplit[1].x1 -= 1.0;
    hsplit[2].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Back to Main")).callback(CALLBACK_STATION_BACK_TO_MAIN));
    world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Reset")).callback(CALLBACK_TRADE_RESET));
    let enabled = if cash >= price_bios || cash >= price_fuel || cash >= price_ice || cash >= price_mach || cash >= price_meds { true } else { false };
    world.ugui.add(menu_tab, ugui::button(&hsplit[2], &format!("Buy Max")).callback(CALLBACK_TRADE_BUY_MAX).enabled(enabled));
    let enabled = if 0 == hold_resources { false } else { true };
    world.ugui.add(menu_tab, ugui::button(&hsplit[3], &format!("Sell Max")).callback(CALLBACK_TRADE_SELL_MAX).enabled(enabled));
    cache.uidata.ui_trade_commit_button = world.ugui.add(menu_tab, ugui::button(&hsplit[4], &format!("Commit")).callback(CALLBACK_TRADE_COMMIT).enabled(false)) as u8;

}


pub fn check_callback_station(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, c: &Callback) -> bool {
    
    let mut update_trade_sliders = false;
    let mut update_terralink_table = false;
    let mut update_marketlink_table = false;
    let mut update_house_table = false;
    
    let sidx = cache.state_cache.location as usize;

    match c.id {
        CALLBACK_STATION_BACK_TO_MAIN => build_station_ui_main(cache, heap, world),
        //
        CALLBACK_STATION_MAIN_CARGO => build_station_ui_cargo(cache, heap, world),
        CALLBACK_STATION_MAIN_NAV => build_station_ui_nav(cache, heap, world),
        CALLBACK_STATION_MAIN_TRADE => build_station_ui_trade(cache, heap, world),
        CALLBACK_STATION_MAIN_HANGAR => build_station_ui_hangar(cache, heap, world),
        CALLBACK_STATION_MAIN_TERRALINK => build_station_ui_terralink(cache, heap, world),
        CALLBACK_STATION_MAIN_MARKETLINK => build_station_ui_marketlink(cache, heap, world),
        CALLBACK_STATION_MAIN_HOUSES => build_station_ui_houses(cache, heap, world),
        CALLBACK_STATION_MAIN_LIFTOFF => build_galaxy_ui_popup(cache, heap, world),
        //
        CALLBACK_STATION_NAV_DOWNLOAD_ALL => {
            for i in 0..heap.state_data.sectors.len() {
                heap.state_data.sectors[i].known = true;
                cache.uidata.all_nav_known = true;
            }
            
            state::update_galaxy_geometry(cache, heap, world);
            if 1 <= heap.state_data.sectors[sidx].station_data.nav_init {
                world.ugui.set_enabled(cache.uidata.station_nav_ui_path_0_idx as usize, false);
                if 2 <= heap.state_data.sectors[sidx].station_data.nav_init {
                    world.ugui.set_enabled(cache.uidata.station_nav_ui_path_1_idx as usize, false);
                }
            }
            world.ugui.set_enabled(cache.uidata.station_nav_ui_all_idx as usize, false);
            world.ugui.set_enabled(cache.uidata.station_nav_ui_local_idx as usize, false);
            world.ugui.force_reconstruct(cache.uidata.station_nav_ui_local_idx as usize);
        }
        CALLBACK_STATION_NAV_DOWNLOAD_LOCAL => {
            heap.state_data.sectors[sidx].station_data.nav_local = true;
            state::find_local_sectors(heap, c.idata0 as usize);
            state::update_galaxy_geometry(cache, heap, world);
            if 1 <= heap.state_data.sectors[sidx].station_data.nav_init {
                if heap.state_data.sectors[heap.state_data.sectors[sidx].station_data.nav_path_0_idx].known {
                    world.ugui.set_enabled(cache.uidata.station_nav_ui_path_0_idx as usize, false);        
                }
                if 2 <= heap.state_data.sectors[sidx].station_data.nav_init {
                    if heap.state_data.sectors[heap.state_data.sectors[sidx].station_data.nav_path_1_idx].known {
                        world.ugui.set_enabled(cache.uidata.station_nav_ui_path_1_idx as usize, false);        
                    }
                }
            }
            world.ugui.set_enabled(cache.uidata.station_nav_ui_local_idx as usize, false);
            world.ugui.force_reconstruct(cache.uidata.station_nav_ui_local_idx as usize);
        }
        CALLBACK_STATION_NAV_DOWNLOAD_PATH_0 => {
            heap.state_data.sectors[sidx].station_data.nav_path_0 = true;
            state::learn_path(heap, sidx, c.idata0 as usize, true);
            state::update_galaxy_geometry(cache, heap, world);
            world.ugui.set_enabled(cache.uidata.station_nav_ui_path_0_idx as usize, false);
            world.ugui.force_reconstruct(cache.uidata.station_nav_ui_path_0_idx as usize);
        }
        CALLBACK_STATION_NAV_DOWNLOAD_PATH_1 => {
            heap.state_data.sectors[sidx].station_data.nav_path_1 = true;
            state::learn_path(heap, sidx, c.idata0 as usize, true);
            state::update_galaxy_geometry(cache, heap, world);
            world.ugui.set_enabled(cache.uidata.station_nav_ui_path_1_idx as usize, false);
            world.ugui.force_reconstruct(cache.uidata.station_nav_ui_path_1_idx as usize);
        }
        CALLBACK_TRADE_RESET => {
            cache.state_cache.trade_qty_bios = 0;
            cache.state_cache.trade_qty_fuel = 0;
            cache.state_cache.trade_qty_ice = 0;
            cache.state_cache.trade_qty_mach = 0;
            cache.state_cache.trade_qty_meds = 0;

            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_bios as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_fuel as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_ice as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_mach as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_meds as usize, 0.0);

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_COMMIT => {
            
            let mut cash_avail = cache.state_cache.cash;

            let pre_bios = heap.state_data.ships[0].hold_bios;
            let pre_fuel = heap.state_data.ships[0].hold_fuel;
            let pre_ice = heap.state_data.ships[0].hold_ice;
            let pre_mach = heap.state_data.ships[0].hold_mach;
            let pre_meds = heap.state_data.ships[0].hold_meds;

            let qty_bios = cache.state_cache.trade_qty_bios;
            let qty_fuel = cache.state_cache.trade_qty_fuel;
            let qty_ice = cache.state_cache.trade_qty_ice;
            let qty_mach = cache.state_cache.trade_qty_mach;
            let qty_meds = cache.state_cache.trade_qty_meds;
            //
            let price_bios = heap.state_data.sectors[sidx].station_data.trade_price_bios;
            let price_fuel = heap.state_data.sectors[sidx].station_data.trade_price_fuel;
            let price_ice = heap.state_data.sectors[sidx].station_data.trade_price_ice;
            let price_mach = heap.state_data.sectors[sidx].station_data.trade_price_mach;
            let price_meds = heap.state_data.sectors[sidx].station_data.trade_price_meds;
            //
            if 0 < qty_bios {
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_bios {
                    cash_avail -= qty_bios * price_bios;
                    heap.state_data.ships[0].hold_bios += qty_bios;
                    heap.state_data.sectors[sidx].station_data.trade_price_bios = (heap.state_data.sectors[sidx].station_data.trade_price_bios as f32 * 1.02).ceil() as i32;
                    // re-calculate effective price
                    heap.state_data.ships[0].effective_price_bios = ((heap.state_data.ships[0].effective_price_bios * pre_bios as f32) + (qty_bios * price_bios) as f32) / heap.state_data.ships[0].hold_bios as f32;
                } else {
                    cash_avail += qty_bios * price_bios;
                    heap.state_data.ships[0].hold_bios -= qty_bios;
                    heap.state_data.sectors[sidx].station_data.trade_price_bios = (heap.state_data.sectors[sidx].station_data.trade_price_bios as f32 / 1.02).floor() as i32;
                    if 0 == heap.state_data.ships[0].hold_bios {
                        heap.state_data.ships[0].effective_price_bios = 0.0;
                    }
                }
            }
            //
            if 0 < qty_fuel {
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_fuel {
                    cash_avail -= qty_fuel * price_fuel;
                    heap.state_data.ships[0].hold_fuel += qty_fuel;
                    heap.state_data.sectors[sidx].station_data.trade_price_fuel = (heap.state_data.sectors[sidx].station_data.trade_price_fuel as f32 * 1.02).ceil() as i32;
                    // re-calculate effective price
                    heap.state_data.ships[0].effective_price_fuel = ((heap.state_data.ships[0].effective_price_fuel * pre_fuel as f32) + (qty_fuel * price_fuel) as f32) / heap.state_data.ships[0].hold_fuel as f32;
                } else {
                    cash_avail += qty_fuel * price_fuel;
                    heap.state_data.ships[0].hold_fuel -= qty_fuel;
                    heap.state_data.sectors[sidx].station_data.trade_price_fuel = (heap.state_data.sectors[sidx].station_data.trade_price_fuel as f32 / 1.02).floor() as i32;
                    if 0 == heap.state_data.ships[0].hold_fuel {
                        heap.state_data.ships[0].effective_price_fuel = 0.0;
                    }
                }
            }
            //
            if 0 < qty_ice {
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_ice {
                    cash_avail -= qty_ice * price_ice;
                    heap.state_data.ships[0].hold_ice += qty_ice;
                    heap.state_data.sectors[sidx].station_data.trade_price_ice = (heap.state_data.sectors[sidx].station_data.trade_price_ice as f32 * 1.02).ceil() as i32;
                    // re-calculate effective price
                    heap.state_data.ships[0].effective_price_ice = ((heap.state_data.ships[0].effective_price_ice * pre_ice as f32) + (qty_ice * price_ice) as f32) / heap.state_data.ships[0].hold_ice as f32;
                } else {
                    cash_avail += qty_ice * price_ice;
                    heap.state_data.ships[0].hold_ice -= qty_ice;
                    heap.state_data.sectors[sidx].station_data.trade_price_ice = (heap.state_data.sectors[sidx].station_data.trade_price_ice as f32 / 1.02).floor() as i32;
                    if 0 == heap.state_data.ships[0].hold_ice {
                        heap.state_data.ships[0].effective_price_ice = 0.0;
                    }
                }
            }
            //
            if 0 < qty_mach {
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_mach {
                    cash_avail -= qty_mach * price_mach;
                    heap.state_data.ships[0].hold_mach += qty_mach;
                    heap.state_data.sectors[sidx].station_data.trade_price_mach = (heap.state_data.sectors[sidx].station_data.trade_price_mach as f32 * 1.02).ceil() as i32;
                    // re-calculate effective price
                    heap.state_data.ships[0].effective_price_mach = ((heap.state_data.ships[0].effective_price_mach * pre_mach as f32) + (qty_mach * price_mach) as f32) / heap.state_data.ships[0].hold_mach as f32;
                } else {
                    cash_avail += qty_mach * price_mach;
                    heap.state_data.ships[0].hold_mach -= qty_mach;
                    heap.state_data.sectors[sidx].station_data.trade_price_mach = (heap.state_data.sectors[sidx].station_data.trade_price_mach as f32 / 1.02).floor() as i32;
                    if 0 == heap.state_data.ships[0].hold_mach {
                        heap.state_data.ships[0].effective_price_mach = 0.0;
                    }
                }
            }
            //
            if 0 < qty_meds {
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_meds {
                    cash_avail -= qty_meds * price_meds;
                    heap.state_data.ships[0].hold_meds += qty_meds;
                    heap.state_data.sectors[sidx].station_data.trade_price_meds = (heap.state_data.sectors[sidx].station_data.trade_price_meds as f32 * 1.02).ceil() as i32;
                    // re-calculate effective price
                    heap.state_data.ships[0].effective_price_meds = ((heap.state_data.ships[0].effective_price_meds * pre_meds as f32) + (qty_meds * price_meds) as f32) / heap.state_data.ships[0].hold_meds as f32;
                } else {
                    cash_avail += qty_meds * price_meds;
                    heap.state_data.ships[0].hold_meds -= qty_meds;
                    heap.state_data.sectors[sidx].station_data.trade_price_meds = (heap.state_data.sectors[sidx].station_data.trade_price_meds as f32 / 1.02).floor() as i32;
                    if 0 == heap.state_data.ships[0].hold_meds {
                        heap.state_data.ships[0].effective_price_meds = 0.0;
                    }
                }
            }

            if 0 > cash_avail { cash_avail = 0; }
            cache.state_cache.cash = cash_avail;


            build_station_ui_main(cache, heap, world);
        }
        CALLBACK_TRADE_SLIDER_BIOLOGICS => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_trade_slider_bios as usize);

            let cash_avail = cache.state_cache.cash;
            let bios_avail = heap.state_data.ships[0].hold_bios;

            // if selling, calculate number of units
            if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_bios {
                cache.state_cache.trade_qty_bios = (tgt_val * cash_avail as f32 / heap.state_data.sectors[sidx].station_data.trade_price_bios as f32).floor() as i32;
            } else {
                if 0 == bios_avail {
                    world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_bios as usize, 0.0);
                } else {
                    cache.state_cache.trade_qty_bios = (tgt_val * bios_avail as f32).floor() as i32;
                    if cache.state_cache.trade_qty_bios > heap.state_data.ships[0].hold_bios {
                        cache.state_cache.trade_qty_bios = heap.state_data.ships[0].hold_bios;
                    }
                }
            }

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_SLIDER_FUEL => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_trade_slider_fuel as usize);

            let cash_avail = cache.state_cache.cash;
            let fuel_avail = heap.state_data.ships[0].hold_fuel;

            // if selling, calculate number of units
            if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_fuel {
                cache.state_cache.trade_qty_fuel = (tgt_val * cash_avail as f32 / heap.state_data.sectors[sidx].station_data.trade_price_fuel as f32).floor() as i32;
            } else {
                if 0 == fuel_avail {
                    world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_fuel as usize, 0.0);
                } else {
                    cache.state_cache.trade_qty_fuel = (tgt_val * fuel_avail as f32).floor() as i32;
                    if cache.state_cache.trade_qty_fuel > heap.state_data.ships[0].hold_fuel {
                        cache.state_cache.trade_qty_fuel = heap.state_data.ships[0].hold_fuel;
                    }
                }
            }

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_SLIDER_ICE => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_trade_slider_ice as usize);

            let cash_avail = cache.state_cache.cash;
            let ice_avail = heap.state_data.ships[0].hold_ice;

            // if selling, calculate number of units
            if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_ice {
                cache.state_cache.trade_qty_ice = (tgt_val * cash_avail as f32 / heap.state_data.sectors[sidx].station_data.trade_price_ice as f32).floor() as i32;
            } else {
                if 0 == ice_avail {
                    world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_ice as usize, 0.0);
                } else {
                    cache.state_cache.trade_qty_ice = (tgt_val * ice_avail as f32).floor() as i32;
                    if cache.state_cache.trade_qty_ice > heap.state_data.ships[0].hold_ice {
                        cache.state_cache.trade_qty_ice = heap.state_data.ships[0].hold_ice;
                    }
                }
            }

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_SLIDER_MACHINERY => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_trade_slider_mach as usize);

            let cash_avail = cache.state_cache.cash;
            let mach_avail = heap.state_data.ships[0].hold_mach;

            // if selling, calculate number of units
            if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_mach {
                cache.state_cache.trade_qty_mach = (tgt_val * cash_avail as f32 / heap.state_data.sectors[sidx].station_data.trade_price_mach as f32).floor() as i32;
            } else {
                if 0 == mach_avail {
                    world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_mach as usize, 0.0);
                } else {
                    cache.state_cache.trade_qty_mach = (tgt_val * mach_avail as f32).floor() as i32;
                    if cache.state_cache.trade_qty_mach > heap.state_data.ships[0].hold_mach {
                        cache.state_cache.trade_qty_mach = heap.state_data.ships[0].hold_mach;
                    }
                }
            }

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_SLIDER_MEDICINE => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_trade_slider_meds as usize);

            let cash_avail = cache.state_cache.cash;
            let meds_avail = heap.state_data.ships[0].hold_meds;

            // if selling, calculate number of units
            if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_meds {
                cache.state_cache.trade_qty_meds = (tgt_val * cash_avail as f32 / heap.state_data.sectors[sidx].station_data.trade_price_meds as f32).floor() as i32;
            } else {
                if 0 == meds_avail {
                    world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_meds as usize, 0.0);
                } else {
                    cache.state_cache.trade_qty_meds = (tgt_val * meds_avail as f32).floor() as i32;
                    if cache.state_cache.trade_qty_meds > heap.state_data.ships[0].hold_meds {
                        cache.state_cache.trade_qty_meds = heap.state_data.ships[0].hold_meds;
                    }
                }
            }
            
            update_trade_sliders = true;
        }
        CALLBACK_TRADE_SELL_MAX => {
            
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_bios as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_fuel as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_ice as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_mach as usize, 0.0);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_meds as usize, 0.0);

            if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_bios && 0 < heap.state_data.ships[0].hold_bios {
                cache.state_cache.trade_qty_bios = heap.state_data.ships[0].hold_bios;
                world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_bios as usize, 1.0);
            } else {
                cache.state_cache.trade_qty_bios = 0;
            }
            if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_fuel && 0 < heap.state_data.ships[0].hold_fuel {
                cache.state_cache.trade_qty_fuel = heap.state_data.ships[0].hold_fuel;
                world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_fuel as usize, 1.0);
            } else {
                cache.state_cache.trade_qty_fuel = 0;
            }
            if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_ice && 0 < heap.state_data.ships[0].hold_ice {
                cache.state_cache.trade_qty_ice = heap.state_data.ships[0].hold_ice;
                world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_ice as usize, 1.0);
            } else {
                cache.state_cache.trade_qty_ice = 0;
            }
            if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_mach && 0 < heap.state_data.ships[0].hold_mach {
                cache.state_cache.trade_qty_mach = heap.state_data.ships[0].hold_mach;
                world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_mach as usize, 1.0);
            } else {
                cache.state_cache.trade_qty_mach = 0;
            }
            if state::STATION_BUYING == heap.state_data.sectors[sidx].station_data.buysell_meds && 0 < heap.state_data.ships[0].hold_meds {
                cache.state_cache.trade_qty_meds = heap.state_data.ships[0].hold_meds;
                world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_meds as usize, 1.0);
            } else {
                cache.state_cache.trade_qty_meds = 0;
            }

            update_trade_sliders = true;
        }
        CALLBACK_TRADE_BUY_MAX => {

            let mut qty_bios = 0;
            let mut qty_fuel = 0;
            let mut qty_ice = 0;
            let mut qty_mach = 0;
            let mut qty_meds = 0;

            let mut cash = cache.state_cache.cash;

            let price_bios = heap.state_data.sectors[sidx].station_data.trade_price_bios;
            let price_fuel = heap.state_data.sectors[sidx].station_data.trade_price_fuel;
            let price_ice = heap.state_data.sectors[sidx].station_data.trade_price_ice;
            let price_mach = heap.state_data.sectors[sidx].station_data.trade_price_mach;
            let price_meds = heap.state_data.sectors[sidx].station_data.trade_price_meds;

            let max_bios = cash as f32 / price_bios as f32;
            let max_fuel = cash as f32 / price_fuel as f32;
            let max_ice = cash as f32 / price_ice as f32;
            let max_mach = cash as f32 / price_mach as f32;
            let max_meds = cash as f32 / price_meds as f32;

            let mut step_sz = 10;

            loop {
                let mut keep_going = false;

                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_bios {
                    let delta = price_bios * step_sz;
                    if 0 <= cash - delta {
                        cash -= delta;
                        qty_bios += step_sz;
                        keep_going = true;
                    }
                }
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_fuel {
                    let delta = price_fuel * step_sz;
                    if 0 <= cash - delta {
                        cash -= delta;
                        qty_fuel += step_sz;
                        keep_going = true;
                    }
                }
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_ice {
                    let delta = price_ice * step_sz;
                    if 0 <= cash - delta {
                        cash -= delta;
                        qty_ice += step_sz;
                        keep_going = true;
                    }
                }
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_mach {
                    let delta = price_mach * step_sz;
                    if 0 <= cash - delta {
                        cash -= delta;
                        qty_mach += step_sz;
                        keep_going = true;
                    }
                }
                if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_meds {
                    let delta = price_meds * step_sz;
                    if 0 <= cash - delta {
                        cash -= delta;
                        qty_meds += step_sz;
                        keep_going = true;
                    }
                }

                if !keep_going {
                    step_sz -= 1;
                    if 0 == step_sz { break; }
                }
            }

            cache.state_cache.trade_qty_bios = qty_bios;
            cache.state_cache.trade_qty_fuel = qty_fuel;
            cache.state_cache.trade_qty_ice = qty_ice;
            cache.state_cache.trade_qty_mach = qty_mach;
            cache.state_cache.trade_qty_meds = qty_meds;

            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_bios as usize, qty_bios as f32 / max_bios);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_fuel as usize, qty_fuel as f32 / max_fuel);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_ice as usize, qty_ice as f32 / max_ice);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_mach as usize, qty_mach as f32 / max_mach);
            world.ugui.set_slider_offset(cache.uidata.ui_trade_slider_meds as usize, qty_meds as f32 / max_meds);

            update_trade_sliders = true;

        }
        CALLBACK_STATION_HANGAR_SLIDER => {
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_station_hangar_slider_idx as usize);

            let cash_avail = cache.state_cache.cash;
            let mut merc_avail = heap.state_data.sectors[sidx].station_data.merc_available as i32;
            let merc_price = heap.state_data.sectors[sidx].station_data.merc_price;

            if merc_avail > 3000 - heap.state_data.ships[0].fighters as i32 {
                merc_avail = 3000 - heap.state_data.ships[0].fighters as i32;
            }
            let merc_total_cost = merc_avail * merc_price;

            let mut qty;

            // quantity limited
            if merc_total_cost <= cash_avail {
                qty = (tgt_val * merc_avail as f32).round() as i32;
            
            // price limited
            } else {
                let rem = cash_avail % merc_price;
                qty = (cash_avail - rem) / merc_price;
                qty = (tgt_val * qty as f32).round() as i32;
            }

            cache.state_cache.merc_to_buy = qty;

            world.ugui.set_text(cache.uidata.ui_station_hangar_qty_avail as usize, &format!("{}", heap.state_data.sectors[sidx].station_data.merc_available as i32 - qty));
            world.ugui.set_text(cache.uidata.ui_station_hangar_total_cost as usize, &format!("Total: {}", qty * merc_price));
            world.ugui.set_text(cache.uidata.ui_station_hangar_cash_after_hire as usize, &format!("Net: {}", get_cash_string(cache.state_cache.cash - qty * merc_price)));

            world.ugui.set_enabled(cache.uidata.ui_station_hangar_hire_button as usize, false);
            if 0 < qty {
                world.ugui.set_enabled(cache.uidata.ui_station_hangar_hire_button as usize, true);
            }
            
            world.ugui.force_reconstruct(cache.uidata.ui_station_tab as usize);
        }
        CALLBACK_STATION_HIRE_FIGHTERS => {

            let merc_price = heap.state_data.sectors[sidx].station_data.merc_price;
            let qty = cache.state_cache.merc_to_buy;

            cache.state_cache.cash -= merc_price * qty;
            heap.state_data.sectors[sidx].station_data.merc_available -= qty as u8;
            heap.state_data.ships[0].fighters += qty;

            build_station_ui_main(cache, heap, world);
        }
        CALLBACK_STATION_TAKE_MAX_FIGHTERS  => {

            let cash_avail = cache.state_cache.cash;
            let mut merc_avail = heap.state_data.sectors[sidx].station_data.merc_available as i32;
            let merc_price = heap.state_data.sectors[sidx].station_data.merc_price;

            if merc_avail > 3000 - heap.state_data.ships[0].fighters as i32 {
                merc_avail = 3000 - heap.state_data.ships[0].fighters as i32;
            }
            let merc_total_cost = merc_avail * merc_price;

            let qty;

            // quantity limited
            if merc_total_cost <= cash_avail {
                qty = merc_avail;
            
            // price limited
            } else {
                let rem = cash_avail % merc_price;
                qty = (cash_avail - rem) / merc_price;
            }

            cache.state_cache.cash -= merc_price * qty;
            heap.state_data.sectors[sidx].station_data.merc_available -= qty as u8;
            heap.state_data.ships[0].fighters += qty;

            build_station_ui_main(cache, heap, world);
        }
        CALLBACK_STATION_TERRALINK_PAGEUP => {
            assert!(cache.uidata.ui_station_terralink_table_offset >= 10);
            cache.uidata.ui_station_terralink_table_offset -= 10;
            update_terralink_table = true;
        }
        CALLBACK_STATION_TERRALINK_PAGEDOWN => {
            cache.uidata.ui_station_terralink_table_offset += 10;
            update_terralink_table = true;
        }
        CALLBACK_STATION_MARKETLINK_PAGEUP => {
            assert!(cache.uidata.ui_station_marketlink_table_offset >= 10);
            cache.uidata.ui_station_marketlink_table_offset -= 10;
            update_marketlink_table = true;
        }
        CALLBACK_STATION_MARKETLINK_PAGEDOWN => {
            cache.uidata.ui_station_marketlink_table_offset += 10;
            update_marketlink_table = true;
        }
        CALLBACK_STATION_HOUSES_PGUP => {
            assert!(cache.uidata.ui_station_house_table_offset >= 10);
            cache.uidata.ui_station_house_table_offset -= 10;
            update_house_table = true;
        }
        CALLBACK_STATION_HOUSES_PGDN => {
            cache.uidata.ui_station_house_table_offset += 10;
            update_house_table = true;
        }
        CALLBACK_STATION_NAV_TO => {
            cache.uidata.curr_sector = c.idata0 as u8;
            cache.uidata.galaxy_hover_x = heap.state_data.sectors[c.idata0 as usize].x;
            cache.uidata.galaxy_hover_y = heap.state_data.sectors[c.idata0 as usize].y;
            update_galaxy_zoom(cache, world);
            world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, true);
            let (sx, sy) = get_galaxy_to_screen_xy(cache, world, cache.uidata.galaxy_hover_x, cache.uidata.galaxy_hover_y);
            cache.uidata.screen_hover_x = sx;
            cache.uidata.screen_hover_y = sy;
            build_galaxy_ui_popup(cache, heap, world);
            state::update_galaxy_geometry(cache, heap, world);
        }
        _ => return false,
    }

    if update_terralink_table {
        
        let pplanets = state::get_player_planet_vec(heap);
        let nrows = pplanets.len();
        let cpage = cache.uidata.ui_station_terralink_table_offset as usize / 10;
        let start = cache.uidata.ui_station_terralink_table_offset as usize;

        for i in 0..10 {

            let gidx = cache.uidata.ui_station_terralink_table as usize + i * 6;

            // turn off all callbacks
            world.ugui.set_callback(gidx as usize, ugui::Callback { enabled: false, id: CALLBACK_INVALID, ..Default::default() });
            
            if start + i >= nrows {
                world.ugui.set_text(gidx+0, &format!(""));
                world.ugui.set_text(gidx+1, &format!(""));
                world.ugui.set_text(gidx+2, &format!(""));
                world.ugui.set_text(gidx+3, &format!(""));
                world.ugui.set_text(gidx+4, &format!(""));
                world.ugui.set_text(gidx+5, &format!(""));
                
                world.ugui.set_visible(gidx+0, false);
                continue;
            }
                
            let pidx = pplanets[start + i];

            let name = heap.state_data.planet_names[pidx].clone();
            let pop = heap.state_data.sectors[pidx].planet_data.cur_pop;
            let research = planet::get_research_name(heap.state_data.sectors[pidx].planet_data.researching);
            let fighters = heap.state_data.sectors[pidx].planet_data.fighters;        
            let class = format!("{}", match heap.state_data.sectors[pidx].planet_class {
                PLANET_CLASS_DESERT => "D",
                PLANET_CLASS_LUSH => "L",
                PLANET_CLASS_VOLCANO => "V",
                PLANET_CLASS_WATER => "W",
                _ => ""
            });

            world.ugui.set_text(gidx+0, &format!("{}", pidx));
            world.ugui.set_text(gidx+1, &format!("{}", name));
            world.ugui.set_text(gidx+2, &format!("{}", class));
            world.ugui.set_text(gidx+3, &format!("{}", pop));
            world.ugui.set_text(gidx+4, &format!("{}", research));
            world.ugui.set_text(gidx+5, &format!("{}", fighters));

            world.ugui.set_visible(gidx+0, true);
            world.ugui.set_callback(gidx as usize, ugui::Callback { enabled: true, id: CALLBACK_STATION_NAV_TO, idata0: pidx as i32, ..Default::default() });
        }

        let enabled = if 0 < cpage { true } else { false };
        world.ugui.set_enabled(cache.uidata.ui_station_terralink_pgup as usize, enabled);
        let enabled = if start + 9 < nrows { true } else { false };
        world.ugui.set_enabled(cache.uidata.ui_station_terralink_pgdn as usize, enabled);

        world.ugui.force_reconstruct(cache.uidata.ui_station_terralink_table as usize);
    }

    if update_marketlink_table {
        populate_marketlink(cache, heap, world);
        world.ugui.force_reconstruct(cache.uidata.ui_station_marketlink_table as usize);
    }

    if update_house_table {
        populate_house_table(cache, heap, world);
        world.ugui.force_reconstruct(cache.uidata.ui_station_tab as usize);
        cache.uidata.ui_station_houses_reconstruct_avatars = true;
        for i in 0..10 {
            let gidx = cache.uidata.ui_station_house_base as usize + i * 8;
            world.ugui.force_reconstruct(gidx+1);
        }
    }

    if update_trade_sliders {
        //
        let hold_colonists = heap.state_data.ships[0].hold_colonists;
        let hold_bios = heap.state_data.ships[0].hold_bios;
        let hold_fuel = heap.state_data.ships[0].hold_fuel;
        let hold_ice = heap.state_data.ships[0].hold_ice;
        let hold_mach = heap.state_data.ships[0].hold_mach;
        let hold_meds = heap.state_data.ships[0].hold_meds;
        //
        let mut price_bios = heap.state_data.sectors[sidx].station_data.trade_price_bios;
        let mut price_fuel = heap.state_data.sectors[sidx].station_data.trade_price_fuel;
        let mut price_ice = heap.state_data.sectors[sidx].station_data.trade_price_ice;
        let mut price_mach = heap.state_data.sectors[sidx].station_data.trade_price_mach;
        let mut price_meds = heap.state_data.sectors[sidx].station_data.trade_price_meds;
        //
        let mut trade_qty_bios = cache.state_cache.trade_qty_bios;
        let mut trade_qty_fuel = cache.state_cache.trade_qty_fuel;
        let mut trade_qty_ice = cache.state_cache.trade_qty_ice;
        let mut trade_qty_mach = cache.state_cache.trade_qty_mach;
        let mut trade_qty_meds = cache.state_cache.trade_qty_meds;
        //
        //
        let mut eff_price_bios = heap.state_data.ships[0].effective_price_bios as i32;
        let mut eff_price_fuel = heap.state_data.ships[0].effective_price_fuel as i32;
        let mut eff_price_ice = heap.state_data.ships[0].effective_price_ice as i32;
        let mut eff_price_mach = heap.state_data.ships[0].effective_price_mach as i32;
        let mut eff_price_meds = heap.state_data.ships[0].effective_price_meds as i32;
        //
        if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_bios {
            price_bios = -1 * price_bios;
            let tot_bios = hold_bios + trade_qty_bios;
            if 0 < tot_bios {
                eff_price_bios = (((heap.state_data.ships[0].effective_price_bios * hold_bios as f32) + (-price_bios as f32 * trade_qty_bios as f32)) / tot_bios as f32) as i32;
            }
        } else {
            trade_qty_bios = -1 * trade_qty_bios;
        }
        if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_fuel {
            price_fuel = -1 * price_fuel;
            let tot_fuel = hold_fuel + trade_qty_fuel;
            if 0 < tot_fuel {
                eff_price_fuel = (((heap.state_data.ships[0].effective_price_fuel * hold_fuel as f32) + (-price_fuel as f32 * trade_qty_fuel as f32)) / tot_fuel as f32) as i32;
            }
        } else {
            trade_qty_fuel = -1 * trade_qty_fuel;
        }
        if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_ice {
            price_ice = -1 * price_ice;
            let tot_ice = hold_ice + trade_qty_ice;
            if 0 < tot_ice {
                eff_price_ice = (((heap.state_data.ships[0].effective_price_ice * hold_ice as f32) + (-price_ice as f32 * trade_qty_ice as f32)) / tot_ice as f32) as i32;
            }
        } else {
            trade_qty_ice = -1 * trade_qty_ice;
        }
        if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_mach {
            price_mach = -1 * price_mach;
            let tot_mach = hold_mach + trade_qty_mach;
            if 0 < tot_mach {
                eff_price_mach = (((heap.state_data.ships[0].effective_price_mach * hold_mach as f32) + (-price_mach as f32 * trade_qty_mach as f32)) / tot_mach as f32) as i32;
            }
        } else {
            trade_qty_mach = -1 * trade_qty_mach;
        }
        if state::STATION_SELLING == heap.state_data.sectors[sidx].station_data.buysell_meds {
            price_meds = -1 * price_meds;
            let tot_meds = hold_meds + trade_qty_meds;
            if 0 < tot_meds {
                eff_price_meds = (((heap.state_data.ships[0].effective_price_meds * hold_meds as f32) + (-price_meds as f32 * trade_qty_meds as f32)) / tot_meds as f32) as i32;
            }
        } else {
            trade_qty_meds = -1 * trade_qty_meds;
        }
        //
        let ratio_colonists = 0.005;
        let ratio_bios = 0.003;
        let ratio_ice = 0.004;
        let ratio_fuel = 0.002;
        let ratio_mach = 0.003;
        let ratio_meds = 0.001;

        let hold_tot = hold_colonists as f32 * ratio_colonists + (hold_bios + trade_qty_bios) as f32 * ratio_bios + (hold_fuel + trade_qty_fuel) as f32 * ratio_fuel + (hold_ice + trade_qty_ice) as f32 * ratio_ice + (hold_mach + trade_qty_mach) as f32 * ratio_mach + (hold_meds + trade_qty_meds) as f32 * ratio_meds;

        //
        let net_bios = price_bios * cache.state_cache.trade_qty_bios;
        let net_fuel = price_fuel * cache.state_cache.trade_qty_fuel;
        let net_ice = price_ice * cache.state_cache.trade_qty_ice;
        let net_mach = price_mach * cache.state_cache.trade_qty_mach;
        let net_meds = price_meds * cache.state_cache.trade_qty_meds;
        
        let cash_tot = cache.state_cache.cash + net_bios + net_fuel + net_ice + net_mach + net_meds;

        //
        world.ugui.set_text(cache.uidata.ui_trade_bios_qty as usize, &format!("{}/", cache.state_cache.trade_qty_bios));
        world.ugui.set_text(cache.uidata.ui_trade_bios_holds as usize, &format!("{:.2}", cache.state_cache.trade_qty_bios as f32 * ratio_bios));
        world.ugui.set_text(cache.uidata.ui_trade_bios_net as usize, &format!("{}", net_bios));
        world.ugui.set_text(cache.uidata.ui_trade_bios_eff_price as usize, &format!("{}", eff_price_bios));
        //
        world.ugui.set_text(cache.uidata.ui_trade_fuel_qty as usize, &format!("{}/", cache.state_cache.trade_qty_fuel));
        world.ugui.set_text(cache.uidata.ui_trade_fuel_holds as usize, &format!("{:.2}", cache.state_cache.trade_qty_fuel as f32 * ratio_fuel));
        world.ugui.set_text(cache.uidata.ui_trade_fuel_net as usize, &format!("{}", net_fuel));
        world.ugui.set_text(cache.uidata.ui_trade_fuel_eff_price as usize, &format!("{}", eff_price_fuel));
        //
        world.ugui.set_text(cache.uidata.ui_trade_ice_qty as usize, &format!("{}/", cache.state_cache.trade_qty_ice));
        world.ugui.set_text(cache.uidata.ui_trade_ice_holds as usize, &format!("{:.2}", cache.state_cache.trade_qty_ice as f32 * ratio_ice));
        world.ugui.set_text(cache.uidata.ui_trade_ice_net as usize, &format!("{}", net_ice));
        world.ugui.set_text(cache.uidata.ui_trade_ice_eff_price as usize, &format!("{}", eff_price_ice));
        //
        world.ugui.set_text(cache.uidata.ui_trade_mach_qty as usize, &format!("{}/", cache.state_cache.trade_qty_mach));
        world.ugui.set_text(cache.uidata.ui_trade_mach_holds as usize, &format!("{:.2}", cache.state_cache.trade_qty_mach as f32 * ratio_mach));
        world.ugui.set_text(cache.uidata.ui_trade_mach_net as usize, &format!("{}", net_mach));
        world.ugui.set_text(cache.uidata.ui_trade_mach_eff_price as usize, &format!("{}", eff_price_mach));
        //
        world.ugui.set_text(cache.uidata.ui_trade_meds_qty as usize, &format!("{}/", cache.state_cache.trade_qty_meds));
        world.ugui.set_text(cache.uidata.ui_trade_meds_holds as usize, &format!("{:.2}", cache.state_cache.trade_qty_meds as f32 * ratio_meds));
        world.ugui.set_text(cache.uidata.ui_trade_meds_net as usize, &format!("{}", net_meds));
        world.ugui.set_text(cache.uidata.ui_trade_meds_eff_price as usize, &format!("{}", eff_price_meds));
        //
        world.ugui.set_text(cache.uidata.ui_trade_holds_tot as usize, &format!("{:.2}/{}", hold_tot, heap.state_data.ships[0].holds));
        world.ugui.set_text(cache.uidata.ui_trade_cash_tot as usize, &format!("{}", get_cash_string(cash_tot)));

        //
        world.ugui.set_enabled(cache.uidata.ui_trade_commit_button as usize, false);
        if 0 <= cash_tot && (0 < cache.state_cache.trade_qty_bios || 0 < cache.state_cache.trade_qty_fuel || 0 < cache.state_cache.trade_qty_ice || 0 < cache.state_cache.trade_qty_mach || 0 < cache.state_cache.trade_qty_meds) {
            world.ugui.set_enabled(cache.uidata.ui_trade_commit_button as usize, true);
        }

        world.ugui.force_reconstruct(cache.uidata.ui_trade_bios_qty as usize);
    }

    true
}

