use crate::mgfw::ecs::ugui::Callback;
use super::*;

pub fn build_planet_ui_main(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_PLANET_MAIN;
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, false);
    world.entity_set_visibility(cache.uidata.planet_menu_tilemap as usize, true);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    cache.uidata.is_menu_open = true;

    let sidx = cache.state_cache.location as usize;

    for i in 0..heap.state_data.ships.len() {
        let e = cache.uidata.galaxy_ship_entities as usize + i;
        world.entity_set_visibility(e, false);
    }

    match heap.state_data.sectors[sidx].planet_class {
        PLANET_CLASS_DESERT => {
            world.entity_set_tileset(cache.uidata.planet_menu_tileset_desert as usize, String::from("assets/sm-desert-planet.png"), 960, 540, 960, 540);
            world.entity_set_tilemap(cache.uidata.planet_menu_tilemap as usize, cache.uidata.planet_menu_tileset_desert as usize, 1, &vec![1]);
        }
        PLANET_CLASS_LUSH => {
            world.entity_set_tileset(cache.uidata.planet_menu_tileset_lush as usize, String::from("assets/sm-lush-planet.png"), 960, 540, 960, 540);
            world.entity_set_tilemap(cache.uidata.planet_menu_tilemap as usize, cache.uidata.planet_menu_tileset_lush as usize, 1, &vec![1]);
        }
        PLANET_CLASS_WATER => {
            world.entity_set_tileset(cache.uidata.planet_menu_tileset_ocean as usize, String::from("assets/sm-ocean-planet.png"), 960, 540, 960, 540);
            world.entity_set_tilemap(cache.uidata.planet_menu_tilemap as usize, cache.uidata.planet_menu_tileset_ocean as usize, 1, &vec![1]);
        }
        PLANET_CLASS_VOLCANO => {
            world.entity_set_tileset(cache.uidata.planet_menu_tileset_volcano as usize, String::from("assets/sm-volcano-planet.png"), 960, 540, 960, 540);
            world.entity_set_tilemap(cache.uidata.planet_menu_tilemap as usize, cache.uidata.planet_menu_tileset_volcano as usize, 1, &vec![1]);
        }
        _ => assert!(false),
    }

    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.4, 0.6]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.25, 0.75]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();

    // options
    let subgrid = vsplit[1].tab_pad();
    let vsplit = subgrid.split_even(SPLIT_V, 9);
    
    // add some vertical padding to the buttons
    let mut vsplit_pad: Vec::<UIgrid> = Vec::new();
    for i in 0..vsplit.len() {
        let mut g = vsplit[i].clone();
        g.y0 += 1.0;
        g.y1 -= 1.0;
        vsplit_pad.push(g);
    }

    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[0], &format!("Planet")).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[1], &format!("Population")).callback(CALLBACK_PLANET_MAIN_POPULATION));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[2], &format!("Manufacturing")).callback(CALLBACK_PLANET_MAIN_MANUFACTURING));
    let enabled = if ALL_RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.researching { true } else { false };
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[3], &format!("Research")).callback(CALLBACK_PLANET_MAIN_RESEARCH).enabled(enabled));
    //world.ugui.add(menu_tab, ugui::button(&vsplit_pad[4], &format!("Defense")).callback(CALLBACK_PLANET_MAIN_MILITARY).enabled(false));
    world.ugui.add(menu_tab, ugui::label(&vsplit_pad[4], &format!("Freighter")).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[5], &format!("Cargo")).callback(CALLBACK_PLANET_MAIN_CARGO));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[6], &format!("Hangar Bay")).callback(CALLBACK_PLANET_MAIN_HANGAR));
    world.ugui.add(menu_tab, ugui::button(&vsplit_pad[8], &format!("Lift Off")).callback(CALLBACK_PLANET_MAIN_LIFTOFF));
    

}


pub fn build_planet_ui_population(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_PLANET_POPULATION;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    let sidx = cache.state_cache.location as usize;

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.2, 0.8]);
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_pop_tab = menu_tab as u8;

    // options
    let vsplit = vsplit[1].tab_pad().pad().split_vec(SPLIT_V, vec![0.5]);

    // top half
    let subsplit = vsplit[0].split_vec(SPLIT_V, vec![0.15, 0.25]);
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Population Outlook - 100 years")).highlight(true));

    let popdata = state::run_pop_sim(cache, heap);
    let mut mn = heap.state_data.sectors[sidx].planet_data.cur_pop;
    let mut mx = heap.state_data.sectors[sidx].planet_data.cur_pop;
    for i in 0..popdata.len() {
        if mn > popdata[i] { mn = popdata[i]; }
        if mx < popdata[i] { mx = popdata[i]; }
    }

    let mut simdata: Vec<(f32, f32)> = Vec::new();
    for i in 0..popdata.len() {
        let x0 = i as f32 / popdata.len() as f32;
        let y0 = 1.0 - (popdata[i] - mn) as f32 / (mx - mn) as f32;

        if i > 0 {
            let dx = x0 - simdata.last().unwrap().0;
            let dy = y0 - simdata.last().unwrap().1;
            for j in 1..10 {
                let r = j as f32 / 10.0;
                simdata.push((x0 - dx * r, y0 - dy * r));
            }
        }
        simdata.push((x0, y0));
    }

    let end_pop = popdata[popdata.len() - 1];

    let hsplit = subsplit[1].split_even(SPLIT_H, 2);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Start: {}", heap.state_data.sectors[sidx].planet_data.cur_pop)).halign(HALIGN_LEFT));
    cache.uidata.ui_pop_end = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("End: {end_pop}")).halign(HALIGN_RIGHT)) as u8;

    // graph
    world.ugui.add(menu_tab, ugui::sunken(&subsplit[2]));
    let subsplit = subsplit[2].pad().pad();
    world.ugui.add(menu_tab, ugui::plotgrid(&subsplit, 10, 5));

    cache.uidata.ui_pop_graph = world.ugui.add(menu_tab, ugui::plotdata(&subsplit, &simdata, PLOT_DATA_MARKER_BLUE)) as u8;


    let mut subsplit = vsplit[1].split_even(SPLIT_V, 9);
    let hsplit = subsplit[0].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Job:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Fraction:")).halign(HALIGN_CENTER).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Current:")).halign(HALIGN_RIGHT).highlight(true));
    //
    let hsplit = subsplit[1].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Farmers")).halign(HALIGN_LEFT));
    cache.uidata.ui_pop_farm = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_farm * 10.0)).halign(HALIGN_CENTER)) as u8;
    cache.uidata.ui_pop_farm_curr = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_farm)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[2].y1 -= 4.0;
    cache.uidata.ui_pop_slider_farm = world.ugui.add(menu_tab, ugui::slider(&subsplit[2], heap.state_data.sectors[sidx].planet_data.frac_farm).callback(CALLBACK_POPULATION_SLIDER_FARMER)) as u8;
    //
    let hsplit = subsplit[3].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Laborers")).halign(HALIGN_LEFT));
    cache.uidata.ui_pop_labor = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_labor * 10.0)).halign(HALIGN_CENTER)) as u8;
    cache.uidata.ui_pop_labor_curr = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_labor)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[4].y1 -= 4.0;
    cache.uidata.ui_pop_slider_labor = world.ugui.add(menu_tab, ugui::slider(&subsplit[4], heap.state_data.sectors[sidx].planet_data.frac_labor).callback(CALLBACK_POPULATION_SLIDER_LABORER)) as u8;
    //
    let hsplit = subsplit[5].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Scientists")).halign(HALIGN_LEFT));
    cache.uidata.ui_pop_sci = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_sci * 10.0)).halign(HALIGN_CENTER)) as u8;
    cache.uidata.ui_pop_sci_curr = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_sci)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[6].y1 -= 4.0;
    cache.uidata.ui_pop_slider_sci = world.ugui.add(menu_tab, ugui::slider(&subsplit[6], heap.state_data.sectors[sidx].planet_data.frac_sci).callback(CALLBACK_POPULATION_SLIDER_SCIENTIST)) as u8;

    
    subsplit[8].y0 = subsplit[8].y1 - 22.0;
    world.ugui.add(menu_tab, ugui::button(&subsplit[8], &format!("Back to Main")).callback(CALLBACK_PLANET_BACK_TO_MAIN));

}


pub fn build_planet_ui_manufacturing(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_PLANET_MANUFACTURING;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.15, 0.9]);

    let sidx = cache.state_cache.location as usize;
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 21);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Manufacturing Outlook - 100 years")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Product:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Start:")).halign(HALIGN_CENTER).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("End:")).halign(HALIGN_RIGHT).highlight(true));

    let (bios, fuel, ice, mach, meds) = state::run_mfg_sim(cache, heap);

    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_bios)).halign(HALIGN_CENTER));
    cache.uidata.ui_mfg_end_bios = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{bios}")).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[3].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_fuel)).halign(HALIGN_CENTER));
    cache.uidata.ui_mfg_end_fuel = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{fuel}")).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[4].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_ice)).halign(HALIGN_CENTER));
    cache.uidata.ui_mfg_end_ice = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{ice}")).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[5].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_mach)).halign(HALIGN_CENTER));
    cache.uidata.ui_mfg_end_mach = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{mach}")).halign(HALIGN_RIGHT)) as u8;
    //
    let hsplit = subsplit[6].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_meds)).halign(HALIGN_CENTER));
    cache.uidata.ui_mfg_end_meds = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{meds}")).halign(HALIGN_RIGHT)) as u8;
    //
    
    let hsplit = subsplit[8].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Product:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Fraction:")).halign(HALIGN_CENTER).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Current:")).halign(HALIGN_RIGHT).highlight(true));
    //
    let hsplit = subsplit[9].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT));
    cache.uidata.ui_mfg_bios = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_bios * 10.0)).halign(HALIGN_CENTER)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_bios)).halign(HALIGN_RIGHT));
    //
    subsplit[10].y1 -= 4.0;
    cache.uidata.ui_mfg_slider_bios = world.ugui.add(menu_tab, ugui::slider(&subsplit[10], heap.state_data.sectors[sidx].planet_data.frac_bios).callback(CALLBACK_MANUFACTURING_SLIDER_BIOLOGICS)) as u8;
    //
    let hsplit = subsplit[11].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT));
    cache.uidata.ui_mfg_fuel = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_fuel * 10.0)).halign(HALIGN_CENTER)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_fuel)).halign(HALIGN_RIGHT));
    //
    subsplit[12].y1 -= 4.0;
    cache.uidata.ui_mfg_slider_fuel = world.ugui.add(menu_tab, ugui::slider(&subsplit[12], heap.state_data.sectors[sidx].planet_data.frac_fuel).callback(CALLBACK_MANUFACTURING_SLIDER_FUEL)) as u8;
    //
    let hsplit = subsplit[13].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT));
    cache.uidata.ui_mfg_ice = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_ice * 10.0)).halign(HALIGN_CENTER)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_ice)).halign(HALIGN_RIGHT));
    //
    subsplit[14].y1 -= 4.0;
    cache.uidata.ui_mfg_slider_ice = world.ugui.add(menu_tab, ugui::slider(&subsplit[14], heap.state_data.sectors[sidx].planet_data.frac_ice).callback(CALLBACK_MANUFACTURING_SLIDER_ICE)) as u8;
    //
    let hsplit = subsplit[15].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT));
    cache.uidata.ui_mfg_mach = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_mach * 10.0)).halign(HALIGN_CENTER)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_mach)).halign(HALIGN_RIGHT));
    //
    subsplit[16].y1 -= 4.0;
    cache.uidata.ui_mfg_slider_mach = world.ugui.add(menu_tab, ugui::slider(&subsplit[16], heap.state_data.sectors[sidx].planet_data.frac_mach).callback(CALLBACK_MANUFACTURING_SLIDER_MACHINERY)) as u8;
    //
    let hsplit = subsplit[17].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT));
    cache.uidata.ui_mfg_meds = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_meds * 10.0)).halign(HALIGN_CENTER)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_meds)).halign(HALIGN_RIGHT));
    //
    subsplit[18].y1 -= 4.0;
    cache.uidata.ui_mfg_slider_meds = world.ugui.add(menu_tab, ugui::slider(&subsplit[18], heap.state_data.sectors[sidx].planet_data.frac_meds).callback(CALLBACK_MANUFACTURING_SLIDER_MEDICINE)) as u8;
    

    
    subsplit[20].y0 = subsplit[20].y1 - 22.0;
    world.ugui.add(menu_tab, ugui::button(&subsplit[20], &format!("Back to Main")).callback(CALLBACK_PLANET_BACK_TO_MAIN));

}



pub fn build_planet_ui_research(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    
    cache.uidata.curr_menu = MENU_PLANET_RESEARCH;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.25, 0.75]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.2, 0.8]);

    let sidx = cache.state_cache.location as usize;
    cache.uidata.planet_menu_research_selection = heap.state_data.sectors[sidx].planet_data.researching;
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_research_tab = menu_tab as u8;

    let vsplit = vsplit[1].tab_pad().split_vec(SPLIT_V, vec![0.08, 0.9]);
    let hsplit = vsplit[0].pad().split_even(SPLIT_H, 2);

    let rsch = heap.state_data.sectors[sidx].planet_data.researching;

    cache.uidata.planet_menu_research_title = world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Researching: {}", get_research_name(rsch))).halign(HALIGN_LEFT).highlight(true)) as u8;

    if RESEARCH_INVALID != heap.state_data.sectors[sidx].planet_data.researching &&
        ALL_RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.researching {
        cache.uidata.planet_menu_research_percent = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{:.1}% (T-{} Years)", heap.state_data.sectors[sidx].planet_data.sci_research_percent, state::run_sci_sim(cache, heap))).halign(HALIGN_RIGHT).highlight(true)) as u8;
    } else {
        cache.uidata.planet_menu_research_percent = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("")).halign(HALIGN_RIGHT).highlight(true)) as u8;
    }

    world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Back to Main")).callback(CALLBACK_PLANET_BACK_TO_MAIN));

    let hsplit = vsplit[1].split(SPLIT_H, 0.4);

    let vsplit = hsplit[1].pad().split_vec(SPLIT_V, vec![0.1, 0.8]);

    cache.uidata.planet_menu_research_description_title = world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("<- Select a research area")).halign(HALIGN_LEFT).valign(VALIGN_TOP).highlight(true)) as u8;

    let line_height = 16.0;
    let mut g = vsplit[1].clone();
    let w = g.x1 - g.x0;
    g.y1 = g.y0 + line_height;

    for i in 0..8 {
        let id = world.ugui.add(menu_tab, ugui::label(&g, &format!("")).halign(HALIGN_LEFT)) as u8;
        g.y0 = g.y1;
        g.y1 += line_height;

        if 0 == i {
            cache.uidata.planet_menu_research_description_rows = id;
        }
    }
    
    if RESEARCH_INVALID == heap.state_data.sectors[sidx].planet_data.researching {
        cache.uidata.planet_menu_research_button = world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Start Research")).enabled(false).callback(CALLBACK_PLANET_RESEARCH_BUTTON)) as u8;
    } else {
        cache.uidata.planet_menu_research_button = world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Waiting for research to complete...")).enabled(false).callback(CALLBACK_PLANET_RESEARCH_BUTTON)) as u8;

        // update title and descriptiong
        world.ugui.set_text(cache.uidata.planet_menu_research_description_title as usize, &format!("{}", get_research_name(cache.uidata.planet_menu_research_selection)));

        // update descriptiong
        // block text
        let s = get_research_description(cache.uidata.planet_menu_research_selection);
        let parts = s.split(' ');
        let mut ss = String::from("   ");    

        // reset text rows
        for i in 0..8 {
            world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + i, &format!(""));
        }

        let mut cc = 0;
        for part in parts {
            let test = format!("{ss} {part}");
            let ww = world.ugui.get_text_width(&test);

            if ww as f32 > 257.0 {
                world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + cc, &format!("{ss}"));
                ss = format!("{part}");
                cc += 1;
            } else {
                ss = test;
            }
        }
        world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + cc, &format!("{ss}"));
    }
    

    let tree = world.ugui.add(menu_tab, ugui::tree(&hsplit[0].pad()));

    for i in 1..RESEARCH_COUNT {
        if RESEARCH_COMPLETE != heap.state_data.sectors[sidx].planet_data.research_status[i] && RESEARCH_NOT_AVAILABLE != heap.state_data.sectors[sidx].planet_data.research_status[i] {
            world.ugui.add_tree_node_callback(tree, ugui::TREE_ROOT, &format!("{}", get_research_name(i as u8)), CALLBACK_RESEARCH_BASE + i as u8);
        }
    }
    
}

pub fn get_research_name(id: u8) -> String {
    match id {
        RESEARCH_AGRICULTURE_I => String::from("Agriculture I"),
        RESEARCH_AGRICULTURE_II => String::from("Agriculture II"),
        RESEARCH_BIOLOGY_I => String::from("Biology I"),
        RESEARCH_BIOLOGY_II => String::from("Biology II"),
        RESEARCH_ENERGY_I => String::from("Energy I"),
        RESEARCH_ENERGY_II => String::from("Energy II"),
        RESEARCH_KNOWLEDGE_I => String::from("Knowledge I"),
        RESEARCH_KNOWLEDGE_II => String::from("Knowledge II"),
        RESEARCH_MEDICINE_I => String::from("Medicine I"),
        RESEARCH_MEDICINE_II => String::from("Medicine II"),
        RESEARCH_MILITARY_I => String::from("Military I"),
        RESEARCH_MILITARY_II => String::from("Military II"),
        RESEARCH_TECHNOLOGY_I => String::from("Technology I"),
        RESEARCH_TECHNOLOGY_II => String::from("Technology II"),
        ALL_RESEARCH_COMPLETE => String::from("All Complete"),
        _ => String::from("Nothing")
    }
}

fn get_research_description(id: u8) -> String {
    
    match id {
        RESEARCH_AGRICULTURE_I => String::from("Basic agricultural research for testing methods, crops, and technologies to improve farming production. Research involves experimenting with soil types, crop varieties, irrigation, and pest control to boost yields, sustainability, and crop quality."),
        RESEARCH_AGRICULTURE_II => String::from("Advanced agricultural research employs cutting-edge technologies and interdisciplinary approaches to optimize farming practices. It encompasses genetic modification of crops, precision agriculture using drones and sensors, and the study of soil microbiomes."),
        RESEARCH_BIOLOGY_I => String::from("Basic biology research into living organisms, their structure, functions, and behavior. Research involves experiments and observations to understand biological processes, like photosynthesis or cell division, studying species, and testing hypotheses."),
        RESEARCH_BIOLOGY_II => String::from("Advanced biology research investigates intricate biological phenomena in areas like genetic engineering, proteomics, and neurobiology, striving to comprehend complex processes at the molecular, cellular, and organismal levels."),
        RESEARCH_ENERGY_I => String::from("Basic energy research into energy sources, conversion, and utilization. Research involves studies to better understand energy systems, efficiency improvements, performance optimization, sustainable solutions, and environmental impact."),
        RESEARCH_ENERGY_II => String::from("Advanced energy research explores innovations in nuclear fusion, advanced renewables, advanced materials for fuel cells, grid optimization, and more efficient energy storage solutions. Research contributes to more sustainable energy sources for the galaxy."),
        RESEARCH_KNOWLEDGE_I => String::from("Basic educational research into teaching and learning methods. Research involves studies to identify effective strategies, assess curriculum, and improve educational outcomes by exploring factors like classroom dynamics and the impact of instructional tools."),
        RESEARCH_KNOWLEDGE_II => String::from("Advanced educational research employs interdisciplinary approaches to investigate complex topics in pedagogy, psychology, and curriculum development. Researchers delve into areas such as personalized learning, cognitive neuroscience, and virtual education."),
        RESEARCH_MEDICINE_I => String::from("Basic medical research to understand disease mechanisms, test treatments, or identify risk factors. Encompasses clinical trials, surveys, and data analysis. Simple medical research contributes to improving patient care, public health, and medical knowledge."),
        RESEARCH_MEDICINE_II => String::from("Advanced medical research to unravel complex health-related challenges. This research explores areas like genomics, precision medicine, and immunotherapy, pushing the boundaries of medical knowledge to fuel the development of novel treatments and therapies."),
        RESEARCH_MILITARY_I => String::from("Basic military research into military technology, tactics, and strategy. Research involves studies to enhance weapons systems, train personnel, conduct military exercises, study historical conflicts, and analyze geopolitical factors."),
        RESEARCH_MILITARY_II => String::from("Advanced military research into cutting-edge technology, intelligence techniques, strategy, and international relations to tackle complex defense and security challenges. Research explores areas such as artificial intelligence, cyber warfare, and geopolitics."),
        RESEARCH_TECHNOLOGY_I => String::from("Basic research to understand the functioning and applications of technology. Research explores hardware, software, user interfaces, emerging technologies, as well as opportunities for enhancing technological efficiency."),
        RESEARCH_TECHNOLOGY_II => String::from("Advanced technology research explores new applications of emerging technologies. This research informs innovations in manufacturing and material science in diverse sectors to shape the future industrial technology landscape."),
        _ => String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed dignissim leo eu est tincidunt, vel pharetra justo scelerisque. Aenean in feugiat odio. Cras posuere metus eu libero faucibus, non ullamcorper dolor sollicitudin. Lorem ipsum dolor sit amet, consectetur adipiscing elit."),
    }

}


pub fn build_planet_ui_cargo(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_PLANET_CARGO;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.33, 0.67]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.15, 0.85]);

    let sidx = cache.state_cache.location as usize;
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_cargo_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 17);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Status of Ship Cargo Holds")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cargo:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("Qty:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("Avail:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("Holds:")).halign(HALIGN_RIGHT).highlight(true));

    let hold_colonists = heap.state_data.ships[0].hold_colonists;
    let hold_bios = heap.state_data.ships[0].hold_bios;
    let hold_fuel = heap.state_data.ships[0].hold_fuel;
    let hold_ice = heap.state_data.ships[0].hold_ice;
    let hold_mach = heap.state_data.ships[0].hold_mach;
    let hold_meds = heap.state_data.ships[0].hold_meds;
    //
    let planet_colonists = heap.state_data.sectors[sidx].planet_data.cur_pop;
    let planet_bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
    let planet_fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
    let planet_ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
    let planet_mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
    let planet_meds = heap.state_data.sectors[sidx].planet_data.cur_meds;
    //
    let ratio_colonists = 0.005;
    let ratio_bios = 0.003;
    let ratio_ice = 0.004;
    let ratio_fuel = 0.002;
    let ratio_mach = 0.003;
    let ratio_meds = 0.001;

    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

    let frac_colonists = hold_colonists as f32 / (hold_colonists as f32 + planet_colonists as f32);
    let frac_bios = if 0 < hold_bios + planet_bios { hold_bios as f32 / (hold_bios as f32 + planet_bios as f32) } else { 0.0 };
    let frac_fuel = if 0 < hold_fuel + planet_fuel { hold_fuel as f32 / (hold_fuel as f32 + planet_fuel as f32) } else { 0.0 };
    let frac_ice = if 0 < hold_ice + planet_ice { hold_ice as f32 / (hold_ice as f32 + planet_ice as f32) } else { 0.0 };
    let frac_mach = if 0 < hold_mach + planet_mach { hold_mach as f32 / (hold_mach as f32 + planet_mach as f32) } else { 0.0 };
    let frac_meds = if 0 < hold_meds + planet_meds { hold_meds as f32 / (hold_meds as f32 + planet_meds as f32) } else { 0.0 };

    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_colonists_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_colonists)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_colonists_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_colonists)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_colonists_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_colonists as f32 * ratio_colonists)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[3].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_colonists = world.ugui.add(menu_tab, ugui::slider(&subsplit[3], frac_colonists).callback(CALLBACK_CARGO_SLIDER_COLONISTS).highlight(true)) as u8;
    //
    let hsplit = subsplit[4].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_bios_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_bios)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_bios_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_bios)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_bios_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_bios as f32 * ratio_bios)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[5].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_bios = world.ugui.add(menu_tab, ugui::slider(&subsplit[5], frac_bios).callback(CALLBACK_CARGO_SLIDER_BIOLOGICS).highlight(true)) as u8;
    //
    let hsplit = subsplit[6].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_fuel_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_fuel)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_fuel_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_fuel)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_fuel_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_fuel as f32 * ratio_fuel)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[7].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_fuel = world.ugui.add(menu_tab, ugui::slider(&subsplit[7], frac_fuel).callback(CALLBACK_CARGO_SLIDER_FUEL).highlight(true)) as u8;
    //
    let hsplit = subsplit[8].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_ice_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_ice)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_ice_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_ice)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_ice_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_ice as f32 * ratio_ice)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[9].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_ice = world.ugui.add(menu_tab, ugui::slider(&subsplit[9], frac_ice).callback(CALLBACK_CARGO_SLIDER_ICE).highlight(true)) as u8;
    //
    let hsplit = subsplit[10].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_mach_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_mach)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_mach_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_mach)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_mach_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_mach as f32 * ratio_mach)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[11].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_mach = world.ugui.add(menu_tab, ugui::slider(&subsplit[11], frac_mach).callback(CALLBACK_CARGO_SLIDER_MACHINERY).highlight(true)) as u8;
    //
    let hsplit = subsplit[12].split_even(SPLIT_H, 4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine")).halign(HALIGN_LEFT));
    cache.uidata.ui_cargo_meds_qty = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", hold_meds)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_meds_avail = world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", planet_meds)).halign(HALIGN_RIGHT)) as u8;
    cache.uidata.ui_cargo_meds_holds = world.ugui.add(menu_tab, ugui::label(&hsplit[3], &format!("{:.2}", hold_meds as f32 * ratio_meds)).halign(HALIGN_RIGHT)) as u8;
    //
    subsplit[13].y1 -= 4.0;
    cache.uidata.ui_cargo_slider_meds = world.ugui.add(menu_tab, ugui::slider(&subsplit[13], frac_meds).callback(CALLBACK_CARGO_SLIDER_MEDICINE).highlight(true)) as u8;
    //
    cache.uidata.ui_cargo_holds_tot = world.ugui.add(menu_tab, ugui::label(&subsplit[14], &format!("Total: {:.2}/{}", hold_tot, heap.state_data.ships[0].holds)).halign(HALIGN_RIGHT)) as u8;


    subsplit[16].y0 = subsplit[16].y1 - 22.0;
    let mut hsplit = subsplit[16].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Back to Main")).callback(CALLBACK_PLANET_BACK_TO_MAIN));
    let enabled = if hold_tot < heap.state_data.ships[0].holds as f32 { true } else { false };
    world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Max Resources")).callback(CALLBACK_PLANET_MAX_RESOURCES).enabled(enabled));

}


pub fn build_planet_ui_hangar(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_PLANET_HANGAR;
    //world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.32, 0.68]);

    let vsplit = main[1].split_vec(SPLIT_V, vec![0.32, 0.68]);

    let sidx = cache.state_cache.location as usize;
    
    // planet menu
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Planet - {}", heap.state_data.planet_names[sidx])).highlight(true));
    cache.uidata.menu_grid = vsplit[1].clone();
    cache.uidata.ui_mfg_tab = menu_tab as u8;

    // options
    let mut subsplit = vsplit[1].tab_pad().pad().split_even(SPLIT_V, 6);

    // top half
    world.ugui.add(menu_tab, ugui::label(&subsplit[0], &format!("Status of Ship Hangar")).highlight(true));
    let hsplit = subsplit[1].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Unit:")).halign(HALIGN_LEFT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("In Hanger:")).halign(HALIGN_RIGHT).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("On Planet:")).halign(HALIGN_RIGHT).highlight(true));

    
    //
    let hsplit = subsplit[2].split_even(SPLIT_H, 3);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", heap.state_data.ships[0].fighters)).halign(HALIGN_RIGHT)) as u8;
    world.ugui.add(menu_tab, ugui::label(&hsplit[2], &format!("{}", heap.state_data.sectors[sidx].planet_data.fighters)).halign(HALIGN_RIGHT)) as u8;
    //

    let fighter_rate = heap.state_data.sectors[sidx].planet_data.fighter_rate as f32 * 0.10;
    let frac_mach = heap.state_data.sectors[sidx].planet_data.frac_mach;                
    let labor = heap.state_data.sectors[sidx].planet_data.cur_labor;
    let fighter_prod = frac_mach * labor as f32 * fighter_rate * 0.01;

    let hsplit = subsplit[3].split_even(SPLIT_H, 2);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Planet Production / 100yrs")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("+{}", (fighter_prod * 100.0).round())).halign(HALIGN_RIGHT));

    subsplit[5].y0 = subsplit[5].y1 - 22.0;
    let mut hsplit = subsplit[5].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Back to Main")).callback(CALLBACK_PLANET_BACK_TO_MAIN));
    let enabled = if heap.state_data.sectors[sidx].planet_data.fighters > 0 && heap.state_data.ships[0].fighters < 3000 { true } else { false };
    world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Take Max")).callback(CALLBACK_PLANET_TAKE_MAX_FIGHTERS).enabled(enabled));

}


pub fn check_callback_planet(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, c: &Callback) -> bool {
    
    let mut update_pop_sliders = false;
    let mut update_mfg_sliders = false;
    let mut update_cargo_sliders = false;
    let mut update_research = false;

    let sidx = cache.state_cache.location as usize;

    match c.id {
        CALLBACK_PLANET_MAIN_POPULATION => build_planet_ui_population(cache, heap, world),
        CALLBACK_PLANET_MAIN_MANUFACTURING => build_planet_ui_manufacturing(cache, heap, world),
        CALLBACK_PLANET_MAIN_RESEARCH => build_planet_ui_research(cache, heap, world),
        CALLBACK_PLANET_MAIN_CARGO => build_planet_ui_cargo(cache, heap, world),
        CALLBACK_PLANET_MAIN_HANGAR => build_planet_ui_hangar(cache, heap, world),
        CALLBACK_PLANET_MAIN_LIFTOFF => build_galaxy_ui_popup(cache, heap, world),
        CALLBACK_PLANET_BACK_TO_MAIN => build_planet_ui_main(cache, heap, world),
        CALLBACK_POPULATION_SLIDER_FARMER => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_pop_slider_farm as usize);
            let mut farm = heap.state_data.sectors[sidx].planet_data.frac_farm;
            let mut labor = heap.state_data.sectors[sidx].planet_data.frac_labor;
            let mut sci = heap.state_data.sectors[sidx].planet_data.frac_sci;
            if labor + sci < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = farm + labor + sci;
                farm /= tot;
                labor /= tot;
                sci /= tot;
                if f32::abs(farm - tgt_val) < 1.0e-6 { break; }
                farm = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_farm = farm;
            heap.state_data.sectors[sidx].planet_data.frac_labor = labor;
            heap.state_data.sectors[sidx].planet_data.frac_sci = sci;
            update_pop_sliders = true;
        }
        CALLBACK_POPULATION_SLIDER_LABORER => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_pop_slider_labor as usize);
            let mut farm = heap.state_data.sectors[sidx].planet_data.frac_farm;
            let mut labor = heap.state_data.sectors[sidx].planet_data.frac_labor;
            let mut sci = heap.state_data.sectors[sidx].planet_data.frac_sci;
            if farm + sci < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = farm + labor + sci;
                farm /= tot;
                labor /= tot;
                sci /= tot;
                if f32::abs(labor - tgt_val) < 1.0e-6 { break; }
                labor = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_farm = farm;
            heap.state_data.sectors[sidx].planet_data.frac_labor = labor;
            heap.state_data.sectors[sidx].planet_data.frac_sci = sci;
            update_pop_sliders = true;
        }
        CALLBACK_POPULATION_SLIDER_SCIENTIST => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_pop_slider_sci as usize);
            let mut farm = heap.state_data.sectors[sidx].planet_data.frac_farm;
            let mut labor = heap.state_data.sectors[sidx].planet_data.frac_labor;
            let mut sci = heap.state_data.sectors[sidx].planet_data.frac_sci;
            if labor + farm < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = farm + labor + sci;
                farm /= tot;
                labor /= tot;
                sci /= tot;
                if f32::abs(sci - tgt_val) < 1.0e-6 { break; }
                sci = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_farm = farm;
            heap.state_data.sectors[sidx].planet_data.frac_labor = labor;
            heap.state_data.sectors[sidx].planet_data.frac_sci = sci;
            update_pop_sliders = true;
        }
        CALLBACK_MANUFACTURING_SLIDER_BIOLOGICS => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_mfg_slider_bios as usize);
            let mut bios = heap.state_data.sectors[sidx].planet_data.frac_bios;
            let mut fuel = heap.state_data.sectors[sidx].planet_data.frac_fuel;
            let mut ice = heap.state_data.sectors[sidx].planet_data.frac_ice;
            let mut mach = heap.state_data.sectors[sidx].planet_data.frac_mach;
            let mut meds = heap.state_data.sectors[sidx].planet_data.frac_meds;
            if fuel + ice + mach + meds < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = bios + fuel + ice + mach + meds;
                bios /= tot;
                fuel /= tot;
                ice /= tot;
                mach /= tot;
                meds /= tot;
                if f32::abs(bios - tgt_val) < 1.0e-6 { break; }
                bios = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_bios = bios;
            heap.state_data.sectors[sidx].planet_data.frac_fuel = fuel;
            heap.state_data.sectors[sidx].planet_data.frac_ice = ice;
            heap.state_data.sectors[sidx].planet_data.frac_mach = mach;
            heap.state_data.sectors[sidx].planet_data.frac_meds = meds;
            update_mfg_sliders = true;
        }
        CALLBACK_MANUFACTURING_SLIDER_FUEL => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_mfg_slider_fuel as usize);
            let mut bios = heap.state_data.sectors[sidx].planet_data.frac_bios;
            let mut fuel = heap.state_data.sectors[sidx].planet_data.frac_fuel;
            let mut ice = heap.state_data.sectors[sidx].planet_data.frac_ice;
            let mut mach = heap.state_data.sectors[sidx].planet_data.frac_mach;
            let mut meds = heap.state_data.sectors[sidx].planet_data.frac_meds;
            if bios + ice + mach + meds < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = bios + fuel + ice + mach + meds;
                bios /= tot;
                fuel /= tot;
                ice /= tot;
                mach /= tot;
                meds /= tot;
                if f32::abs(fuel - tgt_val) < 1.0e-6 { break; }
                fuel = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_bios = bios;
            heap.state_data.sectors[sidx].planet_data.frac_fuel = fuel;
            heap.state_data.sectors[sidx].planet_data.frac_ice = ice;
            heap.state_data.sectors[sidx].planet_data.frac_mach = mach;
            heap.state_data.sectors[sidx].planet_data.frac_meds = meds;
            update_mfg_sliders = true;
        }
        CALLBACK_MANUFACTURING_SLIDER_ICE => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_mfg_slider_ice as usize);
            let mut bios = heap.state_data.sectors[sidx].planet_data.frac_bios;
            let mut fuel = heap.state_data.sectors[sidx].planet_data.frac_fuel;
            let mut ice = heap.state_data.sectors[sidx].planet_data.frac_ice;
            let mut mach = heap.state_data.sectors[sidx].planet_data.frac_mach;
            let mut meds = heap.state_data.sectors[sidx].planet_data.frac_meds;
            if bios + fuel + mach + meds < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = bios + fuel + ice + mach + meds;
                bios /= tot;
                fuel /= tot;
                ice /= tot;
                mach /= tot;
                meds /= tot;
                if f32::abs(ice - tgt_val) < 1.0e-6 { break; }
                ice = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_bios = bios;
            heap.state_data.sectors[sidx].planet_data.frac_fuel = fuel;
            heap.state_data.sectors[sidx].planet_data.frac_ice = ice;
            heap.state_data.sectors[sidx].planet_data.frac_mach = mach;
            heap.state_data.sectors[sidx].planet_data.frac_meds = meds;
            update_mfg_sliders = true;
        }
        CALLBACK_MANUFACTURING_SLIDER_MACHINERY => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_mfg_slider_mach as usize);
            let mut bios = heap.state_data.sectors[sidx].planet_data.frac_bios;
            let mut fuel = heap.state_data.sectors[sidx].planet_data.frac_fuel;
            let mut ice = heap.state_data.sectors[sidx].planet_data.frac_ice;
            let mut mach = heap.state_data.sectors[sidx].planet_data.frac_mach;
            let mut meds = heap.state_data.sectors[sidx].planet_data.frac_meds;
            if bios + fuel + ice + meds < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = bios + fuel + ice + mach + meds;
                bios /= tot;
                fuel /= tot;
                ice /= tot;
                mach /= tot;
                meds /= tot;
                if f32::abs(mach - tgt_val) < 1.0e-6 { break; }
                mach = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_bios = bios;
            heap.state_data.sectors[sidx].planet_data.frac_fuel = fuel;
            heap.state_data.sectors[sidx].planet_data.frac_ice = ice;
            heap.state_data.sectors[sidx].planet_data.frac_mach = mach;
            heap.state_data.sectors[sidx].planet_data.frac_meds = meds;
            update_mfg_sliders = true;
        }
        CALLBACK_MANUFACTURING_SLIDER_MEDICINE => {
            let mut tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_mfg_slider_meds as usize);
            let mut bios = heap.state_data.sectors[sidx].planet_data.frac_bios;
            let mut fuel = heap.state_data.sectors[sidx].planet_data.frac_fuel;
            let mut ice = heap.state_data.sectors[sidx].planet_data.frac_ice;
            let mut mach = heap.state_data.sectors[sidx].planet_data.frac_mach;
            let mut meds = heap.state_data.sectors[sidx].planet_data.frac_meds;
            if bios + fuel + ice + mach < 1.0e-6 { tgt_val = 1.0; }
            for _ in 0..100 {
                let tot = bios + fuel + ice + mach + meds;
                bios /= tot;
                fuel /= tot;
                ice /= tot;
                mach /= tot;
                meds /= tot;
                if f32::abs(meds - tgt_val) < 1.0e-6 { break; }
                meds = tgt_val;
            }
            heap.state_data.sectors[sidx].planet_data.frac_bios = bios;
            heap.state_data.sectors[sidx].planet_data.frac_fuel = fuel;
            heap.state_data.sectors[sidx].planet_data.frac_ice = ice;
            heap.state_data.sectors[sidx].planet_data.frac_mach = mach;
            heap.state_data.sectors[sidx].planet_data.frac_meds = meds;
            update_mfg_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_COLONISTS => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_colonists as usize);

            // calculate new hold requirement
            let hold_colonists = heap.state_data.ships[0].hold_colonists;
            let planet_colonists = heap.state_data.sectors[sidx].planet_data.cur_pop;
            let tot_colonists = hold_colonists + planet_colonists;

            // hold availablity, sub colonists
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_colonists = 0.005;
            let hold_avail_sub = hold_curr - hold_colonists as f32 * ratio_colonists;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_colonists).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_colonists as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }

            // update distributions
            heap.state_data.ships[0].hold_colonists = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_pop = tot_colonists - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_BIOLOGICS => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_bios as usize);

            // calculate new hold requirement
            let hold_bios = heap.state_data.ships[0].hold_bios;
            let planet_bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
            let tot_bios = hold_bios + planet_bios;

            // hold availablity, sub bios
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_bios = 0.003;
            let hold_avail_sub = hold_curr - hold_bios as f32 * ratio_bios;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_bios).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_bios as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }
            
            // update distributions
            heap.state_data.ships[0].effective_price_bios = (heap.state_data.ships[0].effective_price_bios * heap.state_data.ships[0].hold_bios as f32) / new_val as f32;
            heap.state_data.ships[0].hold_bios = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_bios = tot_bios - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_FUEL => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_fuel as usize);

            // calculate new hold requirement
            let hold_fuel = heap.state_data.ships[0].hold_fuel;
            let planet_fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
            let tot_fuel = hold_fuel + planet_fuel;

            // hold availablity, sub fuel
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_fuel = 0.002;
            let hold_avail_sub = hold_curr - hold_fuel as f32 * ratio_fuel;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_fuel).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_fuel as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }

            // update distributions
            heap.state_data.ships[0].effective_price_fuel = (heap.state_data.ships[0].effective_price_fuel * heap.state_data.ships[0].hold_fuel as f32) / new_val as f32;
            heap.state_data.ships[0].hold_fuel = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_fuel = tot_fuel - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_ICE => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_ice as usize);

            // calculate new hold requirement
            let hold_ice = heap.state_data.ships[0].hold_ice;
            let planet_ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
            let tot_ice = hold_ice + planet_ice;

            // hold availablity, sub ice
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_ice = 0.004;
            let hold_avail_sub = hold_curr - hold_ice as f32 * ratio_ice;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_ice).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_ice as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }

            // update distributions
            heap.state_data.ships[0].effective_price_ice = (heap.state_data.ships[0].effective_price_ice * heap.state_data.ships[0].hold_ice as f32) / new_val as f32;
            heap.state_data.ships[0].hold_ice = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_ice = tot_ice - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_MACHINERY => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_mach as usize);

            // calculate new hold requirement
            let hold_mach = heap.state_data.ships[0].hold_mach;
            let planet_mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
            let tot_mach = hold_mach + planet_mach;

            // hold availablity, sub mach
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_mach = 0.003;
            let hold_avail_sub = hold_curr - hold_mach as f32 * ratio_mach;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_mach).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_mach as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }

            // update distributions
            heap.state_data.ships[0].effective_price_mach = (heap.state_data.ships[0].effective_price_mach * heap.state_data.ships[0].hold_mach as f32) / new_val as f32;
            heap.state_data.ships[0].hold_mach = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_mach = tot_mach - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_CARGO_SLIDER_MEDICINE => {
            
            // get new slider value
            let tgt_val = world.ugui.get_slider_offset(cache.uidata.ui_cargo_slider_meds as usize);

            // calculate new hold requirement
            let hold_meds = heap.state_data.ships[0].hold_meds;
            let planet_meds = heap.state_data.sectors[sidx].planet_data.cur_meds;
            let tot_meds = hold_meds + planet_meds;

            // hold availablity, sub meds
            let hold_curr = state::get_hold_availibility(heap);

            let ratio_meds = 0.001;
            let hold_avail_sub = hold_curr - hold_meds as f32 * ratio_meds;
            let hold_avail = heap.state_data.ships[0].holds as f32 - hold_avail_sub;
            let max_can_take = (hold_avail / ratio_meds).floor() as i32;

            // apply hold constraint
            let mut new_val = (tgt_val * tot_meds as f32).floor() as i32;
            if new_val > max_can_take {
                new_val = max_can_take;
            }

            // update distributions
            heap.state_data.ships[0].effective_price_meds = (heap.state_data.ships[0].effective_price_meds * heap.state_data.ships[0].hold_meds as f32) / new_val as f32;
            heap.state_data.ships[0].hold_meds = new_val;
            heap.state_data.sectors[sidx].planet_data.cur_meds = tot_meds - new_val;

            update_cargo_sliders = true;
        }
        CALLBACK_PLANET_RESEARCH_BUTTON => {
            assert!(RESEARCH_INVALID == heap.state_data.sectors[sidx].planet_data.researching);

            heap.state_data.sectors[sidx].planet_data.researching = cache.uidata.planet_menu_research_selection;
            world.ugui.set_text(cache.uidata.planet_menu_research_title as usize, &format!("Researching: {}", get_research_name(heap.state_data.sectors[sidx].planet_data.researching)));
            world.ugui.set_text(cache.uidata.planet_menu_research_button as usize, &format!("Waiting for research to complete..."));
            world.ugui.set_enabled(cache.uidata.planet_menu_research_button as usize, false);
            world.ugui.force_reconstruct(cache.uidata.ui_research_tab as usize);
            let ridx = heap.state_data.sectors[sidx].planet_data.researching as usize;
            heap.state_data.sectors[sidx].planet_data.research_status[ridx] = RESEARCH_IN_PROGRESS;
        }
        CALLBACK_PLANET_TAKE_MAX_FIGHTERS => {
            let max = 3000 - heap.state_data.ships[0].fighters;
            let avail = heap.state_data.sectors[sidx].planet_data.fighters;
            let pickup = i32::min(max, avail);
            heap.state_data.ships[0].fighters += pickup;
            heap.state_data.sectors[sidx].planet_data.fighters -= pickup;
            build_planet_ui_main(cache, heap, world);
        }
        CALLBACK_PLANET_MAX_RESOURCES => {
            update_cargo_sliders = true;

            let mut step_size = 10;

            loop {
                let hold_colonists = heap.state_data.ships[0].hold_colonists;
                let hold_bios = heap.state_data.ships[0].hold_bios;
                let hold_fuel = heap.state_data.ships[0].hold_fuel;
                let hold_ice = heap.state_data.ships[0].hold_ice;
                let hold_mach = heap.state_data.ships[0].hold_mach;
                let hold_meds = heap.state_data.ships[0].hold_meds;
                //
                let planet_bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
                let planet_fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
                let planet_ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
                let planet_mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
                let planet_meds = heap.state_data.sectors[sidx].planet_data.cur_meds;
                //
                let ratio_colonists = 0.005;
                let ratio_bios = 0.003;
                let ratio_ice = 0.004;
                let ratio_fuel = 0.002;
                let ratio_mach = 0.003;
                let ratio_meds = 0.001;

                let holds = heap.state_data.ships[0].holds as f32;
                let mut keep_going = false;

                if step_size <= planet_bios {
                    let hold_tot = hold_colonists as f32 * ratio_colonists + (hold_bios + step_size) as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
                    if hold_tot < holds {
                        heap.state_data.ships[0].hold_bios += step_size;
                        heap.state_data.sectors[sidx].planet_data.cur_bios -= step_size;
                        keep_going = true;
                    }
                }
                if step_size <= planet_fuel {
                    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + (hold_fuel + step_size) as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
                    if hold_tot < holds {
                        heap.state_data.ships[0].hold_fuel += step_size;
                        heap.state_data.sectors[sidx].planet_data.cur_fuel -= step_size;
                        keep_going = true;
                    }
                }
                if step_size <= planet_ice {
                    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + (hold_ice + step_size) as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
                    if hold_tot < holds {
                        heap.state_data.ships[0].hold_ice += step_size;
                        heap.state_data.sectors[sidx].planet_data.cur_ice -= step_size;
                        keep_going = true;
                    }
                }
                if step_size <= planet_mach {
                    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + (hold_mach + step_size) as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
                    if hold_tot < holds {
                        heap.state_data.ships[0].hold_mach += step_size;
                        heap.state_data.sectors[sidx].planet_data.cur_mach -= step_size;
                        keep_going = true;
                    }
                }
                if step_size <= planet_meds {
                    let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + (hold_meds + step_size) as f32 * ratio_meds;
                    if hold_tot < holds {
                        heap.state_data.ships[0].hold_meds += step_size;
                        heap.state_data.sectors[sidx].planet_data.cur_meds -= step_size;
                        keep_going = true;
                    }
                }
                if !keep_going {
                    step_size -= 1;
                    if 0 == step_size { break; }
                }
            }            
        }
        _ => {
            if (c.id > CALLBACK_RESEARCH_BASE) && c.id < (CALLBACK_RESEARCH_BASE + RESEARCH_COUNT as u8) {
                cache.uidata.planet_menu_research_selection = c.id - CALLBACK_RESEARCH_BASE;
                update_research = true;
            } else {
                return false;
            }
        }
    }   

    if update_pop_sliders {

        world.ugui.set_slider_offset(cache.uidata.ui_pop_slider_farm as usize, heap.state_data.sectors[sidx].planet_data.frac_farm);
        world.ugui.set_slider_offset(cache.uidata.ui_pop_slider_labor as usize, heap.state_data.sectors[sidx].planet_data.frac_labor);
        world.ugui.set_slider_offset(cache.uidata.ui_pop_slider_sci as usize, heap.state_data.sectors[sidx].planet_data.frac_sci);
        //
        world.ugui.set_text(cache.uidata.ui_pop_farm as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_farm * 10.0));
        world.ugui.set_text(cache.uidata.ui_pop_labor as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_labor * 10.0));
        world.ugui.set_text(cache.uidata.ui_pop_sci as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_sci * 10.0));
        //
        let popdata = state::run_pop_sim(cache, heap);
        let cur_pop = heap.state_data.sectors[sidx].planet_data.cur_pop;
        let mut mn = cur_pop;
        let mut mx = cur_pop;
        for i in 0..popdata.len() {
            if mn > popdata[i] { mn = popdata[i]; }
            if mx < popdata[i] { mx = popdata[i]; }
        }

        let mut simdata: Vec<(f32, f32)> = Vec::new();
        for i in 0..popdata.len() {
            let x0 = i as f32 / popdata.len() as f32;
            let y0 = 1.0 - (popdata[i] - mn) as f32 / (mx - mn) as f32;

            if i > 0 {
                let dx = x0 - simdata.last().unwrap().0;
                let dy = y0 - simdata.last().unwrap().1;
                for j in 1..10 {
                    let r = j as f32 / 10.0;
                    simdata.push((x0 - dx * r, y0 - dy * r));
                }
            }
            simdata.push((x0, y0));
        }

        let est_farm = std::cmp::max(0, (cur_pop as f32 * heap.state_data.sectors[sidx].planet_data.frac_farm).floor() as i32);
        let mut est_labor = std::cmp::max(0, (cur_pop as f32 * heap.state_data.sectors[sidx].planet_data.frac_labor).floor() as i32);
        let mut est_sci = std::cmp::max(0, (cur_pop as f32 * heap.state_data.sectors[sidx].planet_data.frac_sci).floor() as i32);

        if est_farm + est_labor > cur_pop {
            est_labor = cur_pop - est_farm;
        }
        if est_farm + est_labor + est_sci > cur_pop {
            est_sci = cur_pop - est_farm - est_labor;
        }
        
        heap.state_data.sectors[sidx].planet_data.cur_farm = est_farm;
        heap.state_data.sectors[sidx].planet_data.cur_labor = est_labor;
        heap.state_data.sectors[sidx].planet_data.cur_sci = est_sci;

        world.ugui.set_text(cache.uidata.ui_pop_farm_curr as usize, &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_farm));
        world.ugui.set_text(cache.uidata.ui_pop_labor_curr as usize, &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_labor));
        world.ugui.set_text(cache.uidata.ui_pop_sci_curr as usize, &format!("{}", heap.state_data.sectors[sidx].planet_data.cur_sci));

        let end_pop = popdata[popdata.len() - 1];

        world.ugui.set_text(cache.uidata.ui_pop_end as usize, &format!("End: {end_pop}"));
        world.ugui.set_plotdata(cache.uidata.ui_pop_graph as usize, &simdata);
        //
        world.ugui.force_reconstruct(cache.uidata.ui_pop_tab as usize);
    }
    else if update_mfg_sliders {
        world.ugui.set_slider_offset(cache.uidata.ui_mfg_slider_bios as usize, heap.state_data.sectors[sidx].planet_data.frac_bios);
        world.ugui.set_slider_offset(cache.uidata.ui_mfg_slider_fuel as usize, heap.state_data.sectors[sidx].planet_data.frac_fuel);
        world.ugui.set_slider_offset(cache.uidata.ui_mfg_slider_ice as usize, heap.state_data.sectors[sidx].planet_data.frac_ice);
        world.ugui.set_slider_offset(cache.uidata.ui_mfg_slider_mach as usize, heap.state_data.sectors[sidx].planet_data.frac_mach);
        world.ugui.set_slider_offset(cache.uidata.ui_mfg_slider_meds as usize, heap.state_data.sectors[sidx].planet_data.frac_meds);
        //
        world.ugui.set_text(cache.uidata.ui_mfg_bios as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_bios * 10.0));
        world.ugui.set_text(cache.uidata.ui_mfg_fuel as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_fuel * 10.0));
        world.ugui.set_text(cache.uidata.ui_mfg_ice as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_ice * 10.0));
        world.ugui.set_text(cache.uidata.ui_mfg_mach as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_mach * 10.0));
        world.ugui.set_text(cache.uidata.ui_mfg_meds as usize, &format!("{:.1}/10", heap.state_data.sectors[sidx].planet_data.frac_meds * 10.0));
        //
        let (bios, fuel, ice, mach, meds) = state::run_mfg_sim(cache, heap);
        world.ugui.set_text(cache.uidata.ui_mfg_end_bios as usize, &format!("{bios}"));
        world.ugui.set_text(cache.uidata.ui_mfg_end_fuel as usize, &format!("{fuel}"));
        world.ugui.set_text(cache.uidata.ui_mfg_end_ice as usize, &format!("{ice}"));
        world.ugui.set_text(cache.uidata.ui_mfg_end_mach as usize, &format!("{mach}"));
        world.ugui.set_text(cache.uidata.ui_mfg_end_meds as usize, &format!("{meds}"));

        /*let popdata = run_pop_sim(cache);
        let mut mn = heap.state_data.sectors[sidx].planet_data.cur_pop;
        let mut mx = heap.state_data.sectors[sidx].planet_data.cur_pop;
        for i in 0..popdata.len() {
            if mn > popdata[i] { mn = popdata[i]; }
            if mx < popdata[i] { mx = popdata[i]; }
        }

        let mut simdata: Vec<(f32, f32)> = Vec::new();
        for i in 0..popdata.len() {
            let x0 = i as f32 / popdata.len() as f32;
            let y0 = 1.0 - (popdata[i] - mn) as f32 / (mx - mn) as f32;

            if i > 0 {
                let dx = x0 - simdata.last().unwrap().0;
                let dy = y0 - simdata.last().unwrap().1;
                for j in 1..10 {
                    let r = j as f32 / 10.0;
                    simdata.push((x0 - dx * r, y0 - dy * r));
                }
            }
            simdata.push((x0, y0));
        }

        let end_pop = popdata[popdata.len() - 1];

        world.ugui.set_text(cache.uidata.ui_pop_end as usize, &format!("End: {end_pop}"));
        world.ugui.set_plotdata(cache.uidata.ui_pop_graph as usize, &simdata);*/
        //
        world.ugui.force_reconstruct(cache.uidata.ui_mfg_tab as usize);
    
    } else if update_cargo_sliders {

        let hold_colonists = heap.state_data.ships[0].hold_colonists;
        let hold_bios = heap.state_data.ships[0].hold_bios;
        let hold_fuel = heap.state_data.ships[0].hold_fuel;
        let hold_ice = heap.state_data.ships[0].hold_ice;
        let hold_mach = heap.state_data.ships[0].hold_mach;
        let hold_meds = heap.state_data.ships[0].hold_meds;
        //
        let planet_colonists = heap.state_data.sectors[sidx].planet_data.cur_pop;
        let planet_bios = heap.state_data.sectors[sidx].planet_data.cur_bios;
        let planet_fuel = heap.state_data.sectors[sidx].planet_data.cur_fuel;
        let planet_ice = heap.state_data.sectors[sidx].planet_data.cur_ice;
        let planet_mach = heap.state_data.sectors[sidx].planet_data.cur_mach;
        let planet_meds = heap.state_data.sectors[sidx].planet_data.cur_meds;
        //
        let ratio_colonists = 0.005;
        let ratio_bios = 0.003;
        let ratio_ice = 0.004;
        let ratio_fuel = 0.002;
        let ratio_mach = 0.003;
        let ratio_meds = 0.001;

        let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;

        let frac_colonists = hold_colonists as f32 / (hold_colonists as f32 + planet_colonists as f32);
        let frac_bios = if 0 < hold_bios + planet_bios { hold_bios as f32 / (hold_bios as f32 + planet_bios as f32) } else { 0.0 };
        let frac_fuel = if 0 < hold_fuel + planet_fuel { hold_fuel as f32 / (hold_fuel as f32 + planet_fuel as f32) } else { 0.0 };
        let frac_ice = if 0 < hold_ice + planet_ice { hold_ice as f32 / (hold_ice as f32 + planet_ice as f32) } else { 0.0 };
        let frac_mach = if 0 < hold_mach + planet_mach { hold_mach as f32 / (hold_mach as f32 + planet_mach as f32) } else { 0.0 };
        let frac_meds = if 0 < hold_meds + planet_meds { hold_meds as f32 / (hold_meds as f32 + planet_meds as f32) } else { 0.0 };

        //
        world.ugui.set_text(cache.uidata.ui_cargo_colonists_qty as usize, &format!("{}", hold_colonists));
        world.ugui.set_text(cache.uidata.ui_cargo_colonists_avail as usize, &format!("{}", planet_colonists));
        world.ugui.set_text(cache.uidata.ui_cargo_colonists_holds as usize, &format!("{:.2}", hold_colonists as f32 * ratio_colonists));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_colonists as usize, frac_colonists);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_bios_qty as usize, &format!("{}", hold_bios));
        world.ugui.set_text(cache.uidata.ui_cargo_bios_avail as usize, &format!("{}", planet_bios));
        world.ugui.set_text(cache.uidata.ui_cargo_bios_holds as usize, &format!("{:.2}", hold_bios as f32 * ratio_bios));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_bios as usize, frac_bios);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_fuel_qty as usize, &format!("{}", hold_fuel));
        world.ugui.set_text(cache.uidata.ui_cargo_fuel_avail as usize, &format!("{}", planet_fuel));
        world.ugui.set_text(cache.uidata.ui_cargo_fuel_holds as usize, &format!("{:.2}", hold_fuel as f32 * ratio_fuel));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_fuel as usize, frac_fuel);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_ice_qty as usize, &format!("{}", hold_ice));
        world.ugui.set_text(cache.uidata.ui_cargo_ice_avail as usize, &format!("{}", planet_ice));
        world.ugui.set_text(cache.uidata.ui_cargo_ice_holds as usize, &format!("{:.2}", hold_ice as f32 * ratio_ice));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_ice as usize, frac_ice);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_mach_qty as usize, &format!("{}", hold_mach));
        world.ugui.set_text(cache.uidata.ui_cargo_mach_avail as usize, &format!("{}", planet_mach));
        world.ugui.set_text(cache.uidata.ui_cargo_mach_holds as usize, &format!("{:.2}", hold_mach as f32 * ratio_mach));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_mach as usize, frac_mach);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_meds_qty as usize, &format!("{}", hold_meds));
        world.ugui.set_text(cache.uidata.ui_cargo_meds_avail as usize, &format!("{}", planet_meds));
        world.ugui.set_text(cache.uidata.ui_cargo_meds_holds as usize, &format!("{:.2}", hold_meds as f32 * ratio_meds));
        //
        world.ugui.set_slider_offset(cache.uidata.ui_cargo_slider_meds as usize, frac_meds);
        //
        world.ugui.set_text(cache.uidata.ui_cargo_holds_tot as usize, &format!("Total: {:.2}/{}", hold_tot, heap.state_data.ships[0].holds));

        world.ugui.force_reconstruct(cache.uidata.ui_cargo_tab as usize);

    } else if update_research {

        // update title
        world.ugui.set_text(cache.uidata.planet_menu_research_description_title as usize, &format!("{}", get_research_name(cache.uidata.planet_menu_research_selection)));

        // update descriptiong
        // block text
        let s = get_research_description(cache.uidata.planet_menu_research_selection);
        let parts = s.split(' ');
        let mut ss = String::from("   ");    

        // reset text rows
        for i in 0..8 {
            world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + i, &format!(""));
        }

        let mut cc = 0;
        for part in parts {
            let test = format!("{ss} {part}");
            let ww = world.ugui.get_text_width(&test);

            if ww as f32 > 257.0 {
                world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + cc, &format!("{ss}"));
                ss = format!("{part}");
                cc += 1;
            } else {
                ss = test;
            }
        }
        world.ugui.set_text(cache.uidata.planet_menu_research_description_rows as usize + cc, &format!("{ss}"));
        
        if RESEARCH_INVALID == heap.state_data.sectors[sidx].planet_data.researching {
            world.ugui.set_text(cache.uidata.planet_menu_research_button as usize, &format!("Start Research"));
            world.ugui.set_enabled(cache.uidata.planet_menu_research_button as usize, true);
            world.ugui.set_text(cache.uidata.planet_menu_research_percent as usize, &format!("{:.1}% (T-{} Years)", heap.state_data.sectors[sidx].planet_data.sci_research_percent, state::run_sci_sim(cache, heap)));
        }

        world.ugui.force_reconstruct(cache.uidata.ui_research_tab as usize);
    }

    true
}