use crate::{mgfw::ecs::ugui::Callback};
use std::collections::vec_deque::VecDeque;

use super::*;


pub fn build_galaxy_ui_popup(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    if HOVER_SECTOR_INVALID == cache.uidata.curr_sector { return; }
    
    cache.uidata.is_menu_open = true;

    for i in 0..heap.state_data.ships.len() {
        if cache.uidata.curr_sector == heap.state_data.ships[i].location {
            build_galaxy_ui_location(cache, heap, world);
            return;
        }
    }

    build_galaxy_ui_navigate(cache, heap, world);
}

pub fn build_galaxy_ui_location(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_GALAXY_LOCATION;
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, true);
    world.entity_set_visibility(cache.uidata.planet_menu_tilemap as usize, false);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    let sidx = cache.uidata.curr_sector as usize;

    let mut ops = 3;
    if heap.state_data.sectors[sidx].station_class != STATION_CLASS_INVALID { ops += 2; }
    if heap.state_data.sectors[sidx].planet_class != PLANET_CLASS_INVALID { ops += 2; }

    
    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.42, 0.58]);
    let mut main = main[1].clone();
    let mw = main.x1 - main.x0;
    main.x0 = cache.uidata.screen_hover_x + 5.0;
    main.x1 = main.x0 + mw;
    let mh = 16.0 + ops as f32 * 26.0;
    main.y0 = cache.uidata.screen_hover_y;
    main.y1 = main.y0 + mh;
    if main.x1 > SCREEN_XRES as f32 - 1.0 {
        main.x1 = SCREEN_XRES as f32 - 1.0;
        main.x0 = main.x1 - mw;
    }
    if main.x0 < 1.0 {
        main.x0 = 1.0;
        main.x1 = main.x0 + mw;
    }
    if main.y1 > SCREEN_YRES as f32 - 1.0 {
        main.y1 = SCREEN_YRES as f32 - 1.0;
        main.y0 = main.y1 - mh;
    }
    if main.y0 < 1.0 {
        main.y0 = 1.0;
        main.y1 = main.y0 + mh;
    }
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&main));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Sector {}", sidx)).highlight(true));
    cache.uidata.menu_grid = main.clone();

    // options
    let subgrid = main.tab_pad();
    let vsplit = subgrid.split_even(SPLIT_V, ops);
    
    // add some vertical padding to the buttons
    let mut vsplit_pad: Vec::<UIgrid> = Vec::new();
    for i in 0..vsplit.len() {
        let mut g = vsplit[i].clone();
        g.y0 += 2.0;
        vsplit_pad.push(g);
    }

    vsplit_pad[0].y0 -= 4.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[0], &format!("House {}", heap.state_data.house_names[heap.state_data.sectors[cache.state_cache.location as usize].house as usize])).enabled(false));

    let mut idx = 1;
    if heap.state_data.sectors[sidx].station_class != STATION_CLASS_INVALID {
        vsplit_pad[idx].y0 -= 5.0;
        vsplit_pad[idx].y1 -= 5.0;
        world.ugui.add(menu_tab, ugui::label(&vsplit_pad[idx], &format!("Station")).highlight(true));
        vsplit_pad[idx+1].y0 -= 9.0;
        vsplit_pad[idx+1].y1 -= 2.0;
        world.ugui.add(menu_tab, ugui::button(&vsplit_pad[idx+1], &format!("Dock")).callback(CALLBACK_GALAXY_DOCK));
        idx += 2;
    }
    if heap.state_data.sectors[sidx].planet_class != PLANET_CLASS_INVALID {
        vsplit_pad[idx].y0 -= 5.0;
        vsplit_pad[idx].y1 -= 5.0;
        world.ugui.add(menu_tab, ugui::label(&vsplit_pad[idx], &format!("Planet")).highlight(true));
        vsplit_pad[idx+1].y0 -= 9.0;
        vsplit_pad[idx+1].y1 -= 2.0;
        if state::HOUSE_PLAYER == heap.state_data.sectors[sidx].house {
            world.ugui.add(menu_tab, ugui::button(&vsplit_pad[idx+1], &format!("Land")).callback(CALLBACK_GALAXY_LAND));
        } else {
            world.ugui.add(menu_tab, ugui::button(&vsplit_pad[idx+1], &format!("Attack")).callback(CALLBACK_LAUNCH_BATTLE));//CALLBACK_GALAXY_CLAIM));
        }
        idx += 2;
    }
    vsplit_pad[idx].y0 -= 5.0;
    vsplit_pad[idx].y1 -= 5.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[idx], &format!("Freighter")).highlight(true));
    vsplit_pad[idx+1].y0 -= 9.0;
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[idx+1], &format!("Status")).callback(CALLBACK_GALAXY_STATS));
    
    //
    build_galaxy_ui_year(cache, world);
    build_galaxy_ui_quit(cache, world);

}


pub fn build_galaxy_ui_navigate(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_GALAXY_NAVIGATE;
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, true);
    world.entity_set_visibility(cache.uidata.planet_menu_tilemap as usize, false);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.42, 0.58]);
    let mut main = main[1].clone();
    let mw = main.x1 - main.x0;
    main.x0 = cache.uidata.screen_hover_x + 5.0;
    main.x1 = main.x0 + mw;
    let mh = 80.0;
    main.y0 = cache.uidata.screen_hover_y;
    main.y1 = main.y0 + mh;

    if main.x1 > SCREEN_XRES as f32 - 1.0 {
        main.x1 = SCREEN_XRES as f32 - 1.0;
        main.x0 = main.x1 - mw;
    }
    if main.x0 < 0.0 {
        main.x0 = 0.0;
        main.x1 = main.x0 + mw;
    }
    if main.y1 > SCREEN_YRES as f32 - 1.0 {
        main.y1 = SCREEN_YRES as f32 - 1.0;
        main.y0 = main.y1 - mh;
    }
    if main.y0 < 0.0 {
        main.y0 = 0.0;
        main.y1 = main.y0 + mh;
    }
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&main));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Sector {}", cache.uidata.curr_sector)).highlight(true));
    cache.uidata.menu_grid = main.clone();

    // build nav path
    let path = state::find_path(heap, cache.state_cache.location as usize, cache.uidata.curr_sector as usize, false);
    heap.state_data.ships[0].nav_deque = VecDeque::new();
    if 1 < path.len() {
        for i in 0..path.len()-1 {
            heap.state_data.ships[0].nav_deque.push_front(path[i]);
        }
    }

    // options
    let subgrid = main.tab_pad().split(SPLIT_V, 0.4);

    world.ugui.add(menu_tab, ugui::label(&subgrid[0], &format!("House {}", heap.state_data.house_names[heap.state_data.sectors[cache.uidata.curr_sector as usize].house as usize])).enabled(false));

    world.ugui.add(menu_tab, ugui::button(&subgrid[1], &format!("Navigate")).callback_i1(CALLBACK_GALAXY_NAVIGATE, cache.uidata.curr_sector as i32, path.len() as i32 - 1));

    // path geometry
    let c1 = mgfw::ecs::Color::new(1.0, 0.0, 1.0, 1.0);
    
    let mut pdata: Vec<mgfw::ecs::Position> = Vec::new();
    let mut cdata: Vec<mgfw::ecs::Color> = Vec::new();

    for i in 0..(path.len()-1) {
        let idx0 = path[i];
        let idx1 = path[i + 1];
        let x0 = heap.state_data.sectors[idx0].x;
        let y0 = heap.state_data.sectors[idx0].y - 0.5;
        let x1 = heap.state_data.sectors[idx1].x;
        let y1 = heap.state_data.sectors[idx1].y - 0.5;
        add_connection(&mut pdata, &mut cdata, x0, y0, x1, y1, &c1);
    }

    for i in 0..pdata.len() {
        pdata[i].x = pdata[i].x / 960.0;
        pdata[i].y = pdata[i].y / 540.0;
    }

    world.entity_set_line_buffer(cache.uidata.galaxy_path as usize, &pdata, &cdata);
    world.entity_set_visibility(cache.uidata.galaxy_path as usize, true);
    update_galaxy_zoom(cache, world);

    //
    build_galaxy_ui_year(cache, world);
    build_galaxy_ui_quit(cache, world);

}


pub fn build_galaxy_ui_year(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, 100.0, 25.0);
    let panel = world.ugui.add(ugui::PARENT_WINDOW, ugui::panel(&grid));
    let grid = grid.pad();
    let hsplit = grid.split(SPLIT_H, 0.4);
    world.ugui.add(panel, ugui::label(&hsplit[0], &format!("Year:")).halign(HALIGN_LEFT));
    cache.uidata.galaxy_ui_year = world.ugui.add(panel, ugui::label(&hsplit[1], &format!("{} ", cache.state_cache.year)).halign(HALIGN_RIGHT)) as u8;

}

pub fn build_galaxy_ui_quit(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    let grid = mgfw::ecs::uigrid::new(SCREEN_XRES as f32 - 100.0, 0.0, 100.0, 25.0);
    world.ugui.add(ugui::PARENT_WINDOW, ugui::button(&grid, &format!("Quit Game")).callback(CALLBACK_QUIT_GAME_POPUP));
}

pub fn build_galaxy_ui_quit_confirm(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_QUIT_CONFIRM;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.4, 0.6]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Quit Game")).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();

    let subgrid = vsplit[1].tab_pad().pad();

    let mut vsplit = subgrid.split_even(SPLIT_V, 3);
    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("Are you sure you want to quit?")).highlight(true));
    
    vsplit[2].y0 -= 12.0;
    let mut hsplit = vsplit[2].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Cancel")).callback(CALLBACK_QUIT_GAME_POPUP_CANCEL));
    world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Quit Game")).callback(CALLBACK_QUIT_GAME));

    cache.uidata.is_menu_open = true;

}


pub fn build_galaxy_ui_freighter_status(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_GALAXY_FREIGHTER_STATUS;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.22, 0.78]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Freighter Status")).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 13);

    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Status of Ship Cargo Holds")).highlight(true));

    // top half
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

    let hsplit = subsplit[9].split_even(SPLIT_H, 2);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cash on Hand:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", get_cash_string(cache.state_cache.cash))).halign(HALIGN_RIGHT).highlight(true));

    let hsplit = subsplit[10].split_even(SPLIT_H, 2);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters in Hangar:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.ships[0].fighters)).halign(HALIGN_RIGHT).highlight(true));
    
    subsplit[12].y0 = subsplit[12].y1 - 22.0;
    world.ugui.add(menu_tab, ugui::button(&subsplit[12], &format!("Close")).callback(CALLBACK_GALAXY_CLOSE_STATS));

    cache.uidata.is_menu_open = true;

}


pub fn check_callback_galaxy(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, c: &Callback) -> bool {
    match c.id {
        CALLBACK_GALAXY_LAND => build_planet_ui_main(cache, heap, world),
        CALLBACK_GALAXY_DOCK => build_station_ui_main(cache, heap, world),
        CALLBACK_GALAXY_STATS => build_galaxy_ui_freighter_status(cache, heap, world),
        CALLBACK_GALAXY_CLOSE_STATS => {
            world.ugui.clear();
            build_galaxy_ui_year(cache, world);
            build_galaxy_ui_quit(cache, world);
            cache.uidata.is_menu_open = false;
            world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
            cache.uidata.curr_menu = MENU_NONE;
        }
        CALLBACK_GALAXY_NAVIGATE => {
            heap.state_data.ships[0].animating = true;
            heap.state_data.ships[0].animating_ratio = 0.0;
            world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
            world.ugui.clear();
            build_galaxy_ui_year(cache, world);
            build_galaxy_ui_quit(cache, world);
            /*/
            state::game_step(cache, heap, world, c.idata1);
            cache.state_cache.location = c.idata0 as u8;
            heap.state_data.ships[0].location = cache.state_cache.location;
            world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
            world.entity_set_visibility(cache.uidata.galaxy_path as usize, false);
            build_galaxy_ui_popup(cache, heap, world);*/
        }
        CALLBACK_LAUNCH_BATTLE => {
            build_battle_ui_base(cache, heap, world);
            battle_launch(cache, heap, world);
        }
        CALLBACK_QUIT_GAME_POPUP => {
            build_galaxy_ui_quit_confirm(cache, world);
        }
        CALLBACK_QUIT_GAME_POPUP_CANCEL => {
            world.ugui.clear();
            build_galaxy_ui_year(cache, world);
            build_galaxy_ui_quit(cache, world);
            cache.uidata.is_menu_open = false;
            world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
            cache.uidata.curr_menu = MENU_NONE;
        }
        _ => return false,
    }
    true
}

pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    if !heap.state_data.ships[0].animating {
        for i in 0..heap.state_data.ships.len() {
            let e = cache.uidata.galaxy_ship_entities as usize + i;
            world.entity_set_visibility(e, true);
            let idx = heap.state_data.ships[i].location as usize;
            let gx = heap.state_data.sectors[idx].x;
            let gy = heap.state_data.sectors[idx].y;
            let (sx, sy) = get_galaxy_to_screen_xy(cache, world, gx, gy);
            world.entity_set_position_xy(e, sx, sy);
        }
    
    } else {

        // update animations
        let mut delta = (1.0 - heap.state_data.ships[0].animating_ratio) * 0.008;
        if delta < 0.001 { delta = 0.001; }

        let base_r = heap.state_data.ships[0].animating_ratio;
        heap.state_data.ships[0].animating_ratio = heap.state_data.ships[0].animating_ratio + delta;
        let r = heap.state_data.ships[0].animating_ratio;

        let idx = heap.state_data.ships[0].location as usize;
        let gx = heap.state_data.sectors[idx].x;
        let gy = heap.state_data.sectors[idx].y;
        let (sx0, sy0) = get_galaxy_to_screen_xy(cache, world, gx, gy);

        let idx = heap.state_data.ships[0].nav_deque[0];
        let gx = heap.state_data.sectors[idx].x;
        let gy = heap.state_data.sectors[idx].y;
        let (sx1, sy1) = get_galaxy_to_screen_xy(cache, world, gx, gy);

        let sx = sx0 + (sx1 - sx0) * r;
        let sy = sy0 + (sy1 - sy0) * r;
        let e = cache.uidata.galaxy_ship_entities as usize + 0;
        world.entity_set_position_xy(e, sx, sy);

        /*let psx = (sx0 + (sx1 - sx0) * base_r) - SCREEN_XRES_HALF as f32;
        let psy = (sy0 + (sy1 - sy0) * base_r) - SCREEN_YRES_HALF as f32;
        let qsx = (sx0 + (sx1 - sx0) * r) - SCREEN_XRES_HALF as f32;
        let qsy = (sy0 + (sy1 - sy0) * r) - SCREEN_YRES_HALF as f32;
        let pmag = psx * psx + psy * psy;
        let qmag = qsx * qsx + qsy * qsy;

        if qmag > pmag {
            let z = get_cam_zoom_z(cache);
            let dx = (sx1 - sx0) * (r - base_r) / z;
            let dy = (sy1 - sy0) * (r - base_r) / z;
            cache.uidata.zoom_cam_x += dx * 0.5;
            cache.uidata.zoom_cam_y += dy * 0.5;
            update_galaxy_zoom(cache, world);
        }*/


        /*let mut alpha = r;
        if 1.0 < alpha { alpha = 1.0; }
        world.entity_set_alpha(e, alpha);*/
        
        if (1.0 - r) < 0.01 {
            state::game_step(cache, heap, world, 1);
            world.ugui.set_text(cache.uidata.galaxy_ui_year as usize, &format!("{}", cache.state_cache.year));
            world.ugui.force_reconstruct(cache.uidata.galaxy_ui_year as usize);
            
            cache.state_cache.location = idx as u8;
            heap.state_data.ships[0].nav_deque.pop_front();
            heap.state_data.ships[0].location = cache.state_cache.location;
            heap.state_data.ships[0].animating_ratio = 0.0;
            //world.entity_set_alpha(e, 1.0);

            if heap.state_data.ships[0].nav_deque.is_empty() {
                heap.state_data.ships[0].animating = false;
                world.entity_set_visibility(cache.uidata.galaxy_path as usize, false);
                build_galaxy_ui_popup(cache, heap, world);
            }
        }
    }

}