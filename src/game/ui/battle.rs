use crate::game::state::{BATTLE_TURN_PLAYER, BATTLE_TURN_ENEMY, HOUSE_PLAYER};
use crate::mgfw::ecs::ugui::Callback;
use crate::game::ui;
use super::*;


pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.entity_set_scale_xy(cache.uidata.battle_tiles_0_blue as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_0_blue as usize, 0.2, 0.4, 0.8, 0.5);
    world.entity_set_position_xy(cache.uidata.battle_tiles_0_blue as usize, SCREEN_XRES_HALF as f32 - 280.0, SCREEN_YRES_HALF as f32 - 144.0);

    world.entity_set_scale_xy(cache.uidata.battle_tiles_1_blue as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_1_blue as usize, 0.2, 0.4, 0.8, 0.5);
    world.entity_set_position_xy(cache.uidata.battle_tiles_1_blue as usize, SCREEN_XRES_HALF as f32 - 248.0, SCREEN_YRES_HALF as f32 - 160.0);

    world.entity_set_scale_xy(cache.uidata.battle_tiles_0_red as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_0_red as usize, 0.8, 0.2, 0.2, 0.5);
    world.entity_set_position_xy(cache.uidata.battle_tiles_0_red as usize, SCREEN_XRES_HALF as f32 - 280.0, SCREEN_YRES_HALF as f32 - 144.0);

    world.entity_set_scale_xy(cache.uidata.battle_tiles_1_red as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_1_red as usize, 0.8, 0.2, 0.2, 0.5);
    world.entity_set_position_xy(cache.uidata.battle_tiles_1_red as usize, SCREEN_XRES_HALF as f32 - 248.0, SCREEN_YRES_HALF as f32 - 160.0);


    const W: usize = 48;
    const H: usize = 22;
    let mut data: [u16; W * H] = [0; W * H];

    let ops = [
        0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
        0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
        0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
        0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,
        0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
    ];

    for i in 0..9 {
        let x0 = 0 + 4 * i;
        for j in 0..9 {
            if 0 == ops[j * 17 + i * 2 + 0] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 1;
            data[y0 * W + x0 + 1] = 2;
            data[y0 * W + x0 + 2] = 3;
            data[y0 * W + x0 + W + 0] = 9;
            data[y0 * W + x0 + W + 1] = 10;
            data[y0 * W + x0 + W + 2] = 11;
        }
    }


    world.entity_set_tilemap(cache.uidata.battle_tiles_0 as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());
    world.entity_set_scale_xy(cache.uidata.battle_tiles_0 as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_0 as usize, 0.9, 0.8, 0.9, 0.8);
    let f = 2.0;
    let fa = 0.5;
    world.entity_set_color_rgba(cache.uidata.battle_tiles_0 as usize, 0.25 * f, 0.25 * f, 0.5 * f, fa);
    world.entity_set_position_xy(cache.uidata.battle_tiles_0 as usize, SCREEN_XRES_HALF as f32 - 280.0, SCREEN_YRES_HALF as f32 - 144.0);

    let mut data: [u16; W * H] = [0; W * H];

    for i in 0..8 {
        let x0 = 0 + 4 * i;
        for j in 0..10 {
            if 0 == ops[j * 17 + i * 2 + 1] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 1;
            data[y0 * W + x0 + 1] = 2;
            data[y0 * W + x0 + 2] = 3;
            data[y0 * W + x0 + W + 0] = 9;
            data[y0 * W + x0 + W + 1] = 10;
            data[y0 * W + x0 + W + 2] = 11;
        }
    }

    world.entity_set_tilemap(cache.uidata.battle_tiles_1 as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());
    world.entity_set_scale_xy(cache.uidata.battle_tiles_1 as usize, 16.0, 16.0);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_1 as usize, 0.9, 0.8, 0.9, 0.8);
    world.entity_set_color_rgba(cache.uidata.battle_tiles_1 as usize, 0.25 * f, 0.25 * f, 0.5 * f, fa);
    world.entity_set_position_xy(cache.uidata.battle_tiles_1 as usize, SCREEN_XRES_HALF as f32 - 248.0, SCREEN_YRES_HALF as f32 - 160.0);
    

    // player battle entities
    for i in 0..17 {
        let e = cache.uidata.battle_player_entity_0 as usize + i;
        if 0 == i {
            world.entity_set_billboard(e, String::from("assets/player_freighter.png"));
            world.entity_set_scale_xy(e, 80.0, 48.0);
        } else {
            world.entity_set_billboard(e, String::from("assets/player_fighter.png"));
            world.entity_set_scale_xy(e, 48.0, 32.0);
        }
    }

    // enemy battle entities
    for i in 0..17 {
        let e = cache.uidata.battle_enemy_entity_0 as usize + i;
        if 0 == i {
            world.entity_set_billboard(e, String::from("assets/enemy_freighter.png"));
            world.entity_set_scale_xy(e, 80.0, 48.0);
        } else {
            world.entity_set_billboard(e, String::from("assets/enemy_fighter.png"));
            world.entity_set_scale_xy(e, 48.0, 32.0);
        }
    }

    cache.battle_cache.selected_unit = SELECTED_UNIT_INVALID;
    cache.uidata.battle_animation_lock = false;
    cache.battle_cache.turn_order = [SELECTED_UNIT_INVALID; 32];
    cache.battle_cache.turn_order_index = 0;
    
    
}


pub fn battle_launch(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.entity_set_tileset(cache.uidata.battle_menu_tileset as usize, format!("assets/sm-nebula-{}.png", heap.state_data.ships[0].location % 6), 960, 540, 960, 540);
    
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_hover_ent as usize, false);
    world.entity_set_visibility(cache.uidata.planet_menu_tilemap as usize, false);
    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);

    for i in 0..heap.state_data.ships.len() {
        let e = cache.uidata.galaxy_ship_entities as usize + i;
        world.entity_set_visibility(e, false);
    }
    
    world.entity_set_visibility(cache.uidata.battle_tiles_0 as usize, true);
    world.entity_set_visibility(cache.uidata.battle_tiles_1 as usize, true);

    let fighter_count = usize::min(16, heap.state_data.ships[0].fighters as usize);
    cache.battle_cache.player_units_count = fighter_count as u8;
    cache.battle_cache.auto_battle = false;
    cache.battle_cache.tribute_multiplier = heap.state_data.sectors[cache.state_cache.location as usize].tribute_multiplier;

    // player battle entities
    for i in 0..=fighter_count {
        let e = cache.uidata.battle_player_entity_0 as usize + i;
        world.entity_set_visibility(e, true);
    }

    // enemy battle entities
    for i in 0..17 {
        let e = cache.uidata.battle_enemy_entity_0 as usize + i;
        world.entity_set_visibility(e, true);
    }

    // place entities
    let mut mask = [
        0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
        0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
        0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
        1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
        0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,
        0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
    ];

    let e = cache.uidata.battle_player_entity_0 as usize + 0;
    let (sx, sy) = get_map_to_screen(0, 4);
    world.entity_set_position_xy(e, sx as f32 + 32.0, sy as f32);

    let e = cache.uidata.battle_enemy_entity_0 as usize + 0;
    let (sx, sy) = get_map_to_screen(14, 4);
    world.entity_set_position_xy(e, sx as f32 + 32.0, sy as f32);

    let mut shields_base = 1;
    let mut shields_rem = 0;
    if 16 == fighter_count {
        let nfighters = heap.state_data.ships[0].fighters;
        shields_rem = nfighters % 16;
        shields_base = (nfighters - shields_rem) / 16;
    }

    // player
    for i in 1..=fighter_count {

        let e = cache.uidata.battle_player_entity_0 as usize + i;
        loop {
            let x = world.rnd_range(0..6) as i32;
            let y = world.rnd_range(0..10) as i32;
            let idx = (y * 17 + x) as usize;
            if 1 == mask[idx] {
                mask[idx] = 0;
                let (sx, sy) = get_map_to_screen(x, y);
                world.entity_set_position_xy(e, sx as f32, sy as f32);
                cache.battle_cache.player_units[i - 1].x = x as u8;
                cache.battle_cache.player_units[i - 1].y = y as u8;
                cache.battle_cache.player_units[i - 1].shields = shields_base;
                if 0 < shields_rem {
                    cache.battle_cache.player_units[i - 1].shields += 1;
                    shields_rem -= 1;
                }
                
                let e = cache.uidata.battle_player_fighter_count_ent_0 as usize + i - 1;
                world.entity_set_text(e, format!("{}", cache.battle_cache.player_units[i - 1].shields));
                let w = world.text_get_width(e) as f32 * 0.5;
                world.entity_set_position_xy(e, sx as f32 - w, sy as f32);
                world.entity_set_visibility(e, true);

                break;
            }
        }
    }

    // how many planets are left?
    let nplanets = state::get_num_planets(cache);
    let pplanets = state::get_num_planets_player(heap);
    
    let mut nfighters = 16 + ((0.9 + 0.3 * world.rnd()) * 40.0 * pplanets as f32) as i32;
    if PLANET_CLASS_VOLCANO == heap.state_data.sectors[cache.state_cache.location as usize].planet_class {
        nfighters = (nfighters as f32 * 1.2).floor() as i32;
    }

    let mut shields_rem = nfighters % 16;
    let shields_base = (nfighters - shields_rem) / 16;
    

    // enemy
    for i in 1..17 {
        let e = cache.uidata.battle_enemy_entity_0 as usize + i;
        loop {
            let x = world.rnd_range(11..17) as i32;
            let y = world.rnd_range(0..10) as i32;
            let idx = (y * 17 + x) as usize;
            if 1 == mask[idx] {
                mask[idx] = 0;
                let (sx, sy) = get_map_to_screen(x, y);
                world.entity_set_position_xy(e, sx as f32, sy as f32);
                cache.battle_cache.enemy_units[i - 1].x = x as u8;
                cache.battle_cache.enemy_units[i - 1].y = y as u8;
                cache.battle_cache.enemy_units[i - 1].shields = shields_base;
                if 0 < shields_rem {
                    cache.battle_cache.enemy_units[i - 1].shields += 1;
                    shields_rem -= 1;
                }

                let e = cache.uidata.battle_enemy_fighter_count_ent_0 as usize + i - 1;
                world.entity_set_text(e, format!("{}", cache.battle_cache.enemy_units[i - 1].shields));
                let w = world.text_get_width(e) as f32 * 0.5;
                world.entity_set_position_xy(e, sx as f32 - w, sy as f32);
                world.entity_set_visibility(e, true);

                break;
            }
        }
    }

    // cache map locations in the heap
    heap.battle_data.battle_map_to_screen_lookup.clear();

    for y in 0..10 {
        for x in 0..17 {
            let s = get_map_to_screen(x, y);
            heap.battle_data.battle_map_to_screen_lookup.push(s);
        }
    }

    cache.battle_cache.selected_unit = SELECTED_UNIT_INVALID;
    cache.uidata.battle_animation_lock = false;

    let mut turns: Vec<usize> = Vec::new();
    for i in 0..fighter_count {
        turns.push(i);
    }
    for i in 0..16 {
        turns.push(16 + i);
    }
    for _ in 0..1000 {
        let i0 = world.rnd_range(0..turns.len());
        let i1 = world.rnd_range(0..turns.len());
        let temp = turns[i0];
        turns[i0] = turns[i1];
        turns[i1] = temp;
    }

    cache.battle_cache.turn_order = [SELECTED_UNIT_INVALID; 32];
    cache.battle_cache.turn_count = turns.len() as u8;
    let tm2 = cache.battle_cache.turn_count as f32 / 2.0;

    // battle turn order
    let y0 = 16.0;
    for i in 0..turns.len() {
        let e = cache.uidata.battle_turn_order_ent as usize + i;
        let x0 = SCREEN_XRES_HALF as f32 + (i as f32 - tm2) * 26.0 + 13.0;

        cache.battle_cache.turn_order[i] = turns[i] as u8;
        if 16 > turns[i] {
            world.entity_set_billboard(e, String::from("assets/player_fighter_sm.png"));
        } else {
            world.entity_set_billboard(e, String::from("assets/enemy_fighter_sm.png"));
        }
        world.entity_set_position_xy(e, x0, y0);
        world.entity_set_visibility(e, true);
        world.entity_set_alpha(e, 1.0);

        let e = cache.uidata.battle_turn_order_count_ent as usize + i;
        if 16 > turns[i] {
            world.entity_set_text(e, format!("{}", cache.battle_cache.player_units[turns[i]].shields));
        } else {
            world.entity_set_text(e, format!("{}", cache.battle_cache.enemy_units[turns[i] - 16].shields));
        }
        let w = world.text_get_width(e) as f32 * 0.5;
        world.entity_set_position_xy(e, x0 as f32 - w, y0 as f32);
        world.entity_set_visibility(e, true);
    }

    cache.battle_cache.turn_order_index = 0;
    let x0 = SCREEN_XRES_HALF as f32 + (cache.battle_cache.turn_order_index as f32 - tm2) * 26.0 + 13.0;
    let y0 = y0 + 8.0;
    let e = cache.uidata.battle_turn_order_selector_ent as usize;
    world.entity_set_position_xy(e, x0, y0);
    
    world.entity_set_visibility(cache.uidata.battle_turn_order_selector_ent as usize, true);
    world.entity_set_visibility(cache.uidata.battle_selector_unit_ent as usize, true);
    world.entity_set_visibility(cache.uidata.battle_selector_ent as usize, true);

    highlight_unit_for_turn(cache, heap, world);

    cache.uidata.battle_hold = true;

    cache.battle_cache.trigger_enemy_surrender = false;
    cache.battle_cache.trigger_player_surrender = false;

    let mut bios = 100;
    let mut fuel = 100;
    let mut ice = 100;
    let mut mach = 100;
    let mut meds = 100;

    match heap.state_data.sectors[cache.state_cache.location as usize].planet_class {
        PLANET_CLASS_LUSH => {
            meds = 200;
            ice = 50;
            mach = 30;
            bios = 300;
        }
        PLANET_CLASS_WATER => {
            ice = 200;
            mach = 30;
            bios = 50;
        }
        PLANET_CLASS_DESERT => {
            meds = 10;
            fuel = 300;
            ice = 0;
            bios = 10;
        }
        PLANET_CLASS_VOLCANO => {
            meds = 10;
            mach = 300;
            ice = 0;
            bios = 10;
        }
        _ => (),
    }
    
    cache.battle_cache.enemy_tribute_cash = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * 10000.0).round() as i32;
    cache.battle_cache.enemy_tribute_colonists = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * 1000.0).round() as i32;
    cache.battle_cache.enemy_tribute_bios = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * bios as f32).round() as i32;
    cache.battle_cache.enemy_tribute_fuel = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * fuel as f32).round() as i32;
    cache.battle_cache.enemy_tribute_ice = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * ice as f32).round() as i32;
    cache.battle_cache.enemy_tribute_mach = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * mach as f32).round() as i32;
    cache.battle_cache.enemy_tribute_meds = (pplanets as f32 * (0.8 + 0.3 * world.rnd()) * meds as f32).round() as i32;

}



pub fn battle_close(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.entity_set_visibility(cache.uidata.battle_tiles_0 as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_1 as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_0_blue as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_blue as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_0_red as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_red as usize, false);
    
    // player battle entities
    for i in 0..17 {
        let e = cache.uidata.battle_player_entity_0 as usize + i;
        world.entity_set_visibility(e, false);
        if i < 16 {
            let e = cache.uidata.battle_player_fighter_count_ent_0 as usize + i;
            world.entity_set_visibility(e, false);
            cache.battle_cache.player_units[i].animating = false;
        }
    }

    // enemy battle entities
    for i in 0..17 {
        let e = cache.uidata.battle_enemy_entity_0 as usize + i;
        world.entity_set_visibility(e, false);
        if i < 16 {
            let e = cache.uidata.battle_enemy_fighter_count_ent_0 as usize + i;
            world.entity_set_visibility(e, false);
            cache.battle_cache.enemy_units[i].animating = false;
        }
    }

    // battle turn order
    for i in 0..32 {
        let e = cache.uidata.battle_turn_order_ent as usize + i;
        world.entity_set_visibility(e, false);
        let e = cache.uidata.battle_turn_order_count_ent as usize + i;
        world.entity_set_visibility(e, false);
    }

    world.entity_set_visibility(cache.uidata.battle_turn_order_selector_ent as usize, false);
    world.entity_set_visibility(cache.uidata.battle_selector_unit_ent as usize, false);
    world.entity_set_visibility(cache.uidata.battle_selector_ent as usize, false);

    // return to map
    world.ugui.clear();
    build_galaxy_ui_year(cache, world);
    build_galaxy_ui_quit(cache, world);
    cache.uidata.is_menu_open = false;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    world.entity_set_visibility(cache.uidata.galaxy_map as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_geom as usize, true);
    world.entity_set_visibility(cache.uidata.galaxy_territory as usize, true);
    cache.uidata.curr_menu = MENU_NONE;


}


pub fn get_enemy_move_attack_options(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    // create map mask
    let mut mask = [
        0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
        0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
        0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        2,2,2,1,1,1,1,1,1,1,1,1,1,1,4,4,4,
        1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
        0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,
        0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
    ];

    for i in 0..16 {
        if 0 == cache.battle_cache.player_units[i].shields { continue; }
        let mx = cache.battle_cache.player_units[i].x;
        let my = cache.battle_cache.player_units[i].y;
        let midx = my * 17 + mx;
        mask[midx as usize] = 2;
    }

    for i in 0..16 {
        if 0 == cache.battle_cache.enemy_units[i].shields { continue; }
        let mx = cache.battle_cache.enemy_units[i].x;
        let my = cache.battle_cache.enemy_units[i].y;
        let midx = my * 17 + mx;
        mask[midx as usize] = 3;
    }

    let mut ops_mov: [i32; 170] = [0; 170];
    let mut ops_atk: [i32; 170] = [0; 170];

    let tidx = cache.battle_cache.turn_order[cache.battle_cache.turn_order_index as usize] as usize - 16;
    
    let px = cache.battle_cache.enemy_units[tidx].x as i32;
    let py = cache.battle_cache.enemy_units[tidx].y as i32;
    let (sx0, sy0) = get_map_to_screen(px, py);

    heap.battle_data.battle_map_move_options.clear();
    heap.battle_data.battle_map_attack_options.clear();

    for x in 0..17 {
        for y in 0..10 {
            let bidx = y * 17 + x;
            if 1 == mask[bidx] || (x as i32 == px && y as i32 == py) {
                let (sx, sy) = heap.battle_data.battle_map_to_screen_lookup[bidx];
                let dx = (sx - sx0).abs();
                let dy = (sy - sy0).abs();
                let d = dx * dx + dy * dy;
                if d < 7000 {
                    ops_mov[bidx] = 1;
                    heap.battle_data.battle_map_move_options.push(bidx);
                }
            } else if 2 == mask[bidx] {
                let (sx, sy) = heap.battle_data.battle_map_to_screen_lookup[bidx];
                let dx = (sx - sx0).abs();
                let dy = (sy - sy0).abs();
                let d = dx * dx + dy * dy;
                if d < 7000 {
                    ops_atk[bidx] = 1;
                    heap.battle_data.battle_map_attack_options.push(bidx);
                }
            }
        }
    }

    world.entity_set_visibility(cache.uidata.battle_tiles_0_blue as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_blue as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_0_red as usize, false);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_red as usize, false);

}

pub fn show_move_attack_options(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    // create map mask
    let mut mask = [
        0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
        0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
        0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
        1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        2,2,2,1,1,1,1,1,1,1,1,1,1,1,4,4,4,
        1,2,1,1,1,1,1,1,1,1,1,1,1,1,1,4,1,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
        0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,
        0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
    ];

    for i in 0..16 {
        if 0 == cache.battle_cache.player_units[i].shields { continue; }
        let mx = cache.battle_cache.player_units[i].x;
        let my = cache.battle_cache.player_units[i].y;
        let midx = my * 17 + mx;
        mask[midx as usize] = 2;
    }

    for i in 0..16 {
        if 0 == cache.battle_cache.enemy_units[i].shields { continue; }
        let mx = cache.battle_cache.enemy_units[i].x;
        let my = cache.battle_cache.enemy_units[i].y;
        let midx = my * 17 + mx;
        mask[midx as usize] = 3;
    }

    const W: usize = 48;
    const H: usize = 22;

    let mut ops_mov: [i32; 170] = [0; 170];
    let mut ops_atk: [i32; 170] = [0; 170];

    let uidx = cache.battle_cache.selected_unit as usize;

    let px = cache.battle_cache.player_units[uidx].x as i32;
    let py = cache.battle_cache.player_units[uidx].y as i32;
    let (sx0, sy0) = get_map_to_screen(px, py);

    heap.battle_data.battle_map_move_options.clear();
    heap.battle_data.battle_map_attack_options.clear();

    for y in 0..10 {
        for x in 0..17 {
            let bidx = y * 17 + x;
            if 1 == mask[bidx] || (x as i32 == px && y as i32 == py) {
                let (sx, sy) = heap.battle_data.battle_map_to_screen_lookup[bidx];
                let dx = (sx - sx0).abs();
                let dy = (sy - sy0).abs();
                let d = dx * dx + dy * dy;
                if d < 7000 {
                    ops_mov[bidx] = 1;
                    heap.battle_data.battle_map_move_options.push(bidx);
                }
            } else if 2 < mask[bidx] {
                let (sx, sy) = heap.battle_data.battle_map_to_screen_lookup[bidx];
                let dx = (sx - sx0).abs();
                let dy = (sy - sy0).abs();
                let d = dx * dx + dy * dy;
                if d < 7000 {
                    ops_atk[bidx] = 1;
                    heap.battle_data.battle_map_attack_options.push(bidx);
                }
            }
        }
    }
    

    // movement options
    let mut data: [u16; W * H] = [0; W * H];

    for i in 0..9 {
        let x0 = 0 + 4 * i;
        for j in 0..9 {
            if 0 == ops_mov[j * 17 + i * 2 + 0] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 17;
            data[y0 * W + x0 + 1] = 18;
            data[y0 * W + x0 + 2] = 19;
            data[y0 * W + x0 + W + 0] = 25;
            data[y0 * W + x0 + W + 1] = 26;
            data[y0 * W + x0 + W + 2] = 27;
        }
    }

    world.entity_set_tilemap(cache.uidata.battle_tiles_0_blue as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());

    let mut data: [u16; W * H] = [0; W * H];

    for i in 0..8 {
        let x0 = 0 + 4 * i;
        for j in 0..10 {
            if 0 == ops_mov[j * 17 + i * 2 + 1] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 17;
            data[y0 * W + x0 + 1] = 18;
            data[y0 * W + x0 + 2] = 19;
            data[y0 * W + x0 + W + 0] = 25;
            data[y0 * W + x0 + W + 1] = 26;
            data[y0 * W + x0 + W + 2] = 27;
        }
    }

    world.entity_set_tilemap(cache.uidata.battle_tiles_1_blue as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());


    // attack options
    let mut data: [u16; W * H] = [0; W * H];

    for i in 0..9 {
        let x0 = 0 + 4 * i;
        for j in 0..9 {
            if 0 == ops_atk[j * 17 + i * 2 + 0] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 17;
            data[y0 * W + x0 + 1] = 18;
            data[y0 * W + x0 + 2] = 19;
            data[y0 * W + x0 + W + 0] = 25;
            data[y0 * W + x0 + W + 1] = 26;
            data[y0 * W + x0 + W + 2] = 27;
        }
    }

    world.entity_set_tilemap(cache.uidata.battle_tiles_0_red as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());

    let mut data: [u16; W * H] = [0; W * H];

    for i in 0..8 {
        let x0 = 0 + 4 * i;
        for j in 0..10 {
            if 0 == ops_atk[j * 17 + i * 2 + 1] { continue; }

            let y0 = 0 + 2 * j;
            // add hex
            data[y0 * W + x0 + 0] = 17;
            data[y0 * W + x0 + 1] = 18;
            data[y0 * W + x0 + 2] = 19;
            data[y0 * W + x0 + W + 0] = 25;
            data[y0 * W + x0 + W + 1] = 26;
            data[y0 * W + x0 + W + 2] = 27;
        }
    }

    world.entity_set_tilemap(cache.uidata.battle_tiles_1_red as usize, cache.uidata.battle_map_tileset as usize, W, &data.to_vec());

    world.entity_set_visibility(cache.uidata.battle_tiles_0_blue as usize, true);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_blue as usize, true);
    world.entity_set_visibility(cache.uidata.battle_tiles_0_red as usize, true);
    world.entity_set_visibility(cache.uidata.battle_tiles_1_red as usize, true);

}

pub fn get_map_to_screen(x: i32, y: i32) -> (i32, i32) {
    let sx = SCREEN_XRES_HALF as i32 - 280 + x * 32 + 24;
    let sy = SCREEN_YRES_HALF as i32 - 144 + y * 32 - (x % 2) * 16 + 16;
    (sx, sy)
}

pub fn get_screen_to_map(heap: &GameDataHeap, sx: i32, sy: i32) -> (i32, i32) {
    let mut retx = -1;
    let mut rety = -1;

    let mut mindist = 1e+9;

    'outer: for y in 0..10 {
        for x in 0..17 {
            let bidx = y * 17 + x;
            let (cx, cy) = heap.battle_data.battle_map_to_screen_lookup[bidx];
            let dx = (sx - cx) as f32;
            let dy = (sy - cy) as f32;
            if dx.abs() < 24.0 && dy.abs() < 16.0 {
                let dist = dx * dx + dy * dy;
                if dist < mindist {
                    mindist = dist;
                    retx = x as i32;
                    rety = y as i32;
                    if mindist < 512.0 { break 'outer; }
                }
            }
        }
    }
    
    (retx, rety)
}


pub fn build_battle_ui_base(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.entity_set_visibility(cache.uidata.dim_ent as usize, false);
    
    cache.uidata.curr_menu = MENU_BATTLE_BASE;
    cache.uidata.battle_click_delay = 600;
    
    world.entity_set_visibility(cache.uidata.battle_menu_tilemap as usize, true);

    world.entity_set_tilemap(cache.uidata.battle_menu_tilemap as usize, cache.uidata.battle_menu_tileset as usize, 1, &vec![1]);

    world.ugui.clear();
    // full window grid
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);

    // split for 3 primary columns
    let main = grid.split_vec(SPLIT_H, vec![0.19, 0.81]);
    
    // player menu
    let vsplit = main[0].split(SPLIT_V, 0.8);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("House {}", heap.state_data.house_names[HOUSE_PLAYER as usize])).highlight(true));

    let vsplit = main[0].split(SPLIT_V, 0.8);
    let mut temp = vsplit[1].clone();
    temp.x0 = temp.x1;
    temp.x1 = temp.x0 + 72.0;
    temp.y0 = temp.y1 - 72.0;
    world.ugui.add(ugui::PARENT_WINDOW, ugui::panel(&temp));
    let player_avatar = heap.state_data.house_avatars[HOUSE_PLAYER as usize].clone();
    let tile_set = world.ugui.add(ugui::PARENT_WINDOW, ugui::tileset(format!("assets/avatars_sm/{}", player_avatar), 0, 0));
    cache.uidata.battle_ui_avatar_player = world.ugui.add(ugui::PARENT_WINDOW, ugui::tilemap(&temp.pad(), tile_set, 1, &vec![1])) as u8;

    temp.x0 = temp.x1;
    temp.x1 = temp.x0 + 72.0;
    temp.y0 = temp.y1 - 22.0;
    cache.uidata.battle_ui_auto_toggle = world.ugui.add(ugui::PARENT_WINDOW, ugui::button(&temp, &format!("Auto: OFF")).callback(CALLBACK_BATTLE_AUTO_TOGGLE)) as u8;

    cache.uidata.battle_ui_reconstruct_avatars = true;

    let subgrid = vsplit[1].tab_pad();
    let mut vsplit = subgrid.split_vec(SPLIT_V, vec![0.25, 0.3, 0.65]);
    vsplit[2].y1 -= 1.0;
    vsplit[3].y0 += 1.0;

    let hsplit = vsplit[0].split_vec(SPLIT_H, vec![0.3]);

    // update UI unit counts
    let mut n = 0;
    for i in 0..cache.battle_cache.player_units_count as usize {
        n += cache.battle_cache.player_units[i].shields;
    }
    
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    cache.uidata.battle_ui_player_fighter_count = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{n}")).halign(HALIGN_RIGHT)) as u8;

    world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Offer Tribute")).callback(CALLBACK_BATTLE_OFFER_TRIBUTE));
    world.ugui.add(menu_tab, ugui::button(&vsplit[3], &format!("Surrender")).callback(CALLBACK_BATTLE_SURRENDER));

    

    // enemy menu
    let vsplit = main[2].split(SPLIT_V, 0.8);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("House {}", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));

    let subgrid = vsplit[1].tab_pad();
    let mut vsplit = subgrid.split_vec(SPLIT_V, vec![0.25, 0.3, 0.65]);
    vsplit[2].y1 -= 1.0;
    vsplit[3].y0 += 1.0;

    let hsplit = vsplit[0].split_vec(SPLIT_H, vec![0.3]);

    let mut n = 0;
    for i in 0..16 as usize {
        n += cache.battle_cache.enemy_units[i].shields;
    }
    
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    cache.uidata.battle_ui_enemy_fighter_count = world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{n}")).halign(HALIGN_RIGHT)) as u8;
    
    world.ugui.add(menu_tab, ugui::button(&vsplit[2], &format!("Demand Tribute")).callback(CALLBACK_BATTLE_DEMAND_TRIBUTE));
    world.ugui.add(menu_tab, ugui::button(&vsplit[3], &format!("Demand Surrender")).callback(CALLBACK_BATTLE_DEMAND_SURRENDER));

    let vsplit = main[2].split(SPLIT_V, 0.8);
    let mut temp = vsplit[1].clone();
    temp.x1 = temp.x0;
    temp.x0 = temp.x1 - 72.0;
    temp.y0 = temp.y1 - 72.0;
    world.ugui.add(ugui::PARENT_WINDOW, ugui::panel(&temp));
    let enemy_avatar = heap.state_data.house_avatars[heap.state_data.sectors[cache.state_cache.location as usize].house as usize].clone();
    let tile_set = world.ugui.add(ugui::PARENT_WINDOW, ugui::tileset(format!("assets/avatars_sm/{}", enemy_avatar), 0, 0));
    cache.uidata.battle_ui_avatar_enemy = world.ugui.add(ugui::PARENT_WINDOW, ugui::tilemap(&temp.pad(), tile_set, 1, &vec![1])) as u16;
    

}


pub fn build_battle_ui_enemy_rejects_tribute_demand(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_REJECTS_TRIBUTE_DEMAND;
    
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);

    world.ugui.clear();
    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    // Demand Tribute/Surrender
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.35, 0.65]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 4);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("You Demand 10% Hold Tribute!")).highlight(true));
    
    vsplit[2].y0 -= 10.0;
    vsplit[2].y1 -= 10.0;
    vsplit[3].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("House {} Response:", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit[3], &format!("Rejected!")).callback(CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE_DEMAND));


}


pub fn build_battle_ui_enemy_offers_tribute(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_OFFERS_TRIBUTE;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    
    world.ugui.clear();
    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.25, 0.75]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 12);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("House {} Offers 10% Hold Tribute:", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));

    let ratio = 0.1 * cache.battle_cache.tribute_multiplier;
    let tribute_cash = (cache.battle_cache.enemy_tribute_cash as f32 * ratio).floor() as i32;
    let tribute_colonists = (cache.battle_cache.enemy_tribute_colonists as f32 * ratio).floor() as i32;
    let tribute_bios = (cache.battle_cache.enemy_tribute_bios as f32 * ratio).floor() as i32;
    let tribute_fuel = (cache.battle_cache.enemy_tribute_fuel as f32 * ratio).floor() as i32;
    let tribute_ice = (cache.battle_cache.enemy_tribute_ice as f32 * ratio).floor() as i32;
    let tribute_mach = (cache.battle_cache.enemy_tribute_mach as f32 * ratio).floor() as i32;
    let tribute_meds = (cache.battle_cache.enemy_tribute_meds as f32 * ratio).floor() as i32;
    let tribute_fighters = (count_fighters_enemy(cache) as f32 * ratio).floor() as i32;

    
    let hsplit = vsplit[1].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cash:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", ui::get_cash_string(tribute_cash))).halign(HALIGN_RIGHT));

    let hsplit = vsplit[2].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_colonists}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[3].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_bios}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[4].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fuel}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[5].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_ice}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[6].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_mach}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[7].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_meds}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[8].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fighters}")).halign(HALIGN_RIGHT));
    
    vsplit[10].y0 -= 10.0;
    vsplit[10].y1 -= 10.0;
    vsplit[11].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit[10], &format!("Your Response:")).highlight(true));
    let mut hsplit = vsplit[11].split_even(SPLIT_H, 2);
    hsplit[0].x1 -= 1.0;
    hsplit[1].x0 += 1.0;
    world.ugui.add(menu_tab, ugui::button(&hsplit[0], &format!("Rejected!")).callback(CALLBACK_BATTLE_REJECT_ENEMY_TRIBUTE));
    world.ugui.add(menu_tab, ugui::button(&hsplit[1], &format!("Accepted!")).callback(CALLBACK_BATTLE_ACCEPT_ENEMY_TRIBUTE));


}



pub fn build_battle_ui_enemy_rejects_surrender_demand(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_REJECTS_SURRENDER_DEMAND;
    
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    // Demand Tribute/Surrender
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.35, 0.65]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 4);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("You Demand Surrender!")).highlight(true));
    
    vsplit[2].y0 -= 10.0;
    vsplit[2].y1 -= 10.0;
    vsplit[3].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("House {} Response:", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit[3], &format!("Rejected!")).callback(CALLBACK_BATTLE_REJECT_PLAYER_SURRENDER_DEMAND));


}


pub fn build_battle_ui_enemy_surrenders(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_SURRENDER;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.23, 0.77]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 14);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("House {} Surrenders!", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[1], &format!("You Take Planet {}!", heap.state_data.planet_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("Captured Freighter Hold:")));
    
    let ratio = 0.5;
    let tribute_cash = (cache.battle_cache.enemy_tribute_cash as f32 * ratio).floor() as i32;
    let tribute_colonists = (cache.battle_cache.enemy_tribute_colonists as f32 * ratio).floor() as i32;
    let tribute_bios = (cache.battle_cache.enemy_tribute_bios as f32 * ratio).floor() as i32;
    let tribute_fuel = (cache.battle_cache.enemy_tribute_fuel as f32 * ratio).floor() as i32;
    let tribute_ice = (cache.battle_cache.enemy_tribute_ice as f32 * ratio).floor() as i32;
    let tribute_mach = (cache.battle_cache.enemy_tribute_mach as f32 * ratio).floor() as i32;
    let tribute_meds = (cache.battle_cache.enemy_tribute_meds as f32 * ratio).floor() as i32;
    let tribute_fighters = (count_fighters_enemy(cache) as f32 * ratio).floor() as i32;

    
    let hsplit = vsplit[3].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cash:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", ui::get_cash_string(tribute_cash))).halign(HALIGN_RIGHT));

    let hsplit = vsplit[4].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_colonists}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[5].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_bios}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[6].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fuel}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[7].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_ice}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[8].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_mach}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[9].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_meds}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[10].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fighters}")).halign(HALIGN_RIGHT));
    
    vsplit[12].y0 -= 10.0;
    vsplit[12].y1 -= 10.0;
    vsplit[13].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::button(&vsplit[13], &format!("Close")).callback(CALLBACK_BATTLE_CLOSE_BATTLE));

    cache.state_cache.cash += tribute_cash;
    heap.state_data.ships[0].fighters += tribute_fighters;
    if heap.state_data.ships[0].fighters > 3000 { heap.state_data.ships[0].fighters = 3000; }

    get_hold_tribute_from_enemey(cache, heap, ratio);
    

}


pub fn build_battle_ui_enemy_rejects_player_tribute(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_REJECTS_PLAYER_TRIBUTE;
    
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);

    // Demand Tribute/Surrender
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.35, 0.65]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 4);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("You Offer 10% Hold Tribute!")).highlight(true));
    
    vsplit[2].y0 -= 10.0;
    vsplit[2].y1 -= 10.0;
    vsplit[3].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("House {} Response:", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit[3], &format!("Rejected!")).callback(CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE));


}


pub fn build_battle_ui_enemy_accepts_player_tribute(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_ENEMY_ACCEPTS_PLAYER_TRIBUTE;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.22, 0.78]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    let ratio = 0.1;
    let tribute_cash = (cache.state_cache.cash as f32 * ratio).floor() as i32;
    let tribute_colonists = (heap.state_data.ships[0].hold_colonists as f32 * ratio).floor() as i32;
    let tribute_bios = (heap.state_data.ships[0].hold_bios as f32 * ratio).floor() as i32;
    let tribute_fuel = (heap.state_data.ships[0].hold_fuel as f32 * ratio).floor() as i32;
    let tribute_ice = (heap.state_data.ships[0].hold_ice as f32 * ratio).floor() as i32;
    let tribute_mach = (heap.state_data.ships[0].hold_mach as f32 * ratio).floor() as i32;
    let tribute_meds = (heap.state_data.ships[0].hold_meds as f32 * ratio).floor() as i32;
    let tribute_fighters = (heap.state_data.ships[0].fighters as f32 * ratio).floor() as i32;

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 12);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("You Offer 10% Hold Tribute:")).highlight(true));
    
    let hsplit = vsplit[1].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cash:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", ui::get_cash_string(tribute_cash))).halign(HALIGN_RIGHT));

    let hsplit = vsplit[2].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_colonists}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[3].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_bios}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[4].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fuel}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[5].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_ice}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[6].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_mach}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[7].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_meds}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[8].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fighters}")).halign(HALIGN_RIGHT));
    
    vsplit[10].y0 -= 10.0;
    vsplit[10].y1 -= 10.0;
    vsplit[11].y0 -= 10.0;
    world.ugui.add(menu_tab, ugui::label(&vsplit[10], &format!("House {} Response:", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));
    world.ugui.add(menu_tab, ugui::button(&vsplit[11], &format!("Accepted!")).callback(CALLBACK_BATTLE_ACCEPT_PLAYER_TRIBUTE));
    
    cache.state_cache.cash -= tribute_cash;
    heap.state_data.ships[0].hold_colonists -= tribute_colonists;
    heap.state_data.ships[0].hold_bios -= tribute_bios;
    heap.state_data.ships[0].hold_fuel -= tribute_fuel;
    heap.state_data.ships[0].hold_ice -= tribute_ice;
    heap.state_data.ships[0].hold_mach -= tribute_mach;
    heap.state_data.ships[0].hold_meds -= tribute_meds;
    heap.state_data.ships[0].fighters -= tribute_fighters;

}



pub fn build_battle_ui_player_surrenders(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_PLAYER_SURRENDER;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.22, 0.78]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("Diplomacy")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    let ratio = if heap.state_data.sectors[cache.state_cache.location as usize].enemy_aggressive { 0.6 } else { 0.4 };
    let tribute_cash = (cache.state_cache.cash as f32 * ratio).floor() as i32;
    let tribute_colonists = (heap.state_data.ships[0].hold_colonists as f32 * ratio).floor() as i32;
    let tribute_bios = (heap.state_data.ships[0].hold_bios as f32 * ratio).floor() as i32;
    let tribute_fuel = (heap.state_data.ships[0].hold_fuel as f32 * ratio).floor() as i32;
    let tribute_ice = (heap.state_data.ships[0].hold_ice as f32 * ratio).floor() as i32;
    let tribute_mach = (heap.state_data.ships[0].hold_mach as f32 * ratio).floor() as i32;
    let tribute_meds = (heap.state_data.ships[0].hold_meds as f32 * ratio).floor() as i32;
    let tribute_fighters = (heap.state_data.ships[0].fighters as f32 * ratio).floor() as i32;

    // offer tribute
    let mut vsplit = subgrid.split_even(SPLIT_V, 12);

    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("You Surrender! You Lose:")).highlight(true));
    
    let hsplit = vsplit[1].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Cash:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{}", ui::get_cash_string(tribute_cash))).halign(HALIGN_RIGHT));

    let hsplit = vsplit[2].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Colonists:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_colonists}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[3].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Biologics:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_bios}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[4].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fuel:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fuel}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[5].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Ice:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_ice}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[6].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Machinery:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_mach}")).halign(HALIGN_RIGHT));

    let hsplit = vsplit[7].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Medicine:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_meds}")).halign(HALIGN_RIGHT));
    
    let hsplit = vsplit[8].split(SPLIT_H, 0.4);
    world.ugui.add(menu_tab, ugui::label(&hsplit[0], &format!("Fighters:")).halign(HALIGN_LEFT));
    world.ugui.add(menu_tab, ugui::label(&hsplit[1], &format!("{tribute_fighters}")).halign(HALIGN_RIGHT));
    
    vsplit[10].y0 -= 10.0;
    vsplit[10].y1 -= 10.0;
    vsplit[11].y0 -= 10.0;

    world.ugui.add(menu_tab, ugui::button(&vsplit[11], &format!("Close")).callback(CALLBACK_BATTLE_CLOSE_BATTLE));

    cache.state_cache.cash -= tribute_cash;
    heap.state_data.ships[0].hold_colonists -= tribute_colonists;
    heap.state_data.ships[0].hold_bios -= tribute_bios;
    heap.state_data.ships[0].hold_fuel -= tribute_fuel;
    heap.state_data.ships[0].hold_ice -= tribute_ice;
    heap.state_data.ships[0].hold_mach -= tribute_mach;
    heap.state_data.ships[0].hold_meds -= tribute_meds;
    heap.state_data.ships[0].fighters -= tribute_fighters;
    

}


pub fn build_battle_ui_game_over(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_GAME_OVER;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.3, 0.7]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("GAME OVER")).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    let mut vsplit = subgrid.split_even(SPLIT_V, 5);
    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("All Fighters Destroyed")).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[1], &format!("GAME OVER!")).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("Thank you for playing!")).highlight(true));
    
    vsplit[4].y0 -= 22.0;
    world.ugui.add(menu_tab, ugui::button(&vsplit[4], &format!("Quit Game")).callback(CALLBACK_QUIT_GAME));
    

}

pub fn build_battle_ui_game_win(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.uidata.curr_menu = MENU_BATTLE_GAME_WIN;
    world.entity_set_visibility(cache.uidata.dim_ent as usize, true);
    world.ugui.clear();

    // diplomacy menu
    let grid = mgfw::ecs::uigrid::new(0.0, 0.0, SCREEN_XRES as f32, SCREEN_YRES as f32);
    let main = grid.split_vec(SPLIT_H, vec![0.35, 0.65]);
    
    let vsplit = main[1].split_vec(SPLIT_V, vec![0.3, 0.7]);
    let menu_tab_group = world.ugui.add(ugui::PARENT_WINDOW, ugui::tab_group(&vsplit[1]));
    let menu_tab = world.ugui.add(menu_tab_group, ugui::tab(&format!("House {} Surrenders!", heap.state_data.house_names[cache.state_cache.location as usize])).highlight(true));

    let subgrid = vsplit[1].tab_pad().pad();

    let mut vsplit = subgrid.split_even(SPLIT_V, 5);
    world.ugui.add(menu_tab, ugui::label(&vsplit[0], &format!("All Planets Claimed")).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[1], &format!("YOU WIN!")).highlight(true));
    world.ugui.add(menu_tab, ugui::label(&vsplit[2], &format!("Thank you for playing!")).highlight(true));
    
    vsplit[4].y0 -= 22.0;
    world.ugui.add(menu_tab, ugui::button(&vsplit[4], &format!("Quit Game")).callback(CALLBACK_QUIT_GAME));
    

}

pub fn event_battle(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) {
    
    if 0 < cache.uidata.battle_click_delay || cache.uidata.battle_animation_lock || !is_player_turn(cache) || cache.battle_cache.auto_battle { return; }

    if MENU_BATTLE_BASE != cache.uidata.curr_menu { return; }

    let mut attack_idx: i32 = -1;

    if mgfw::EVENT_INPUT_KEYBOARD_PRESSED_SPACE == event_id {        
        // skip turn
        increment_turn(cache, heap, world);
        cache.uidata.battle_click_delay = 300;
    
    } else if mgfw::EVENT_INPUT_MOUSE_BUTTON_UP == event_id {

        // click on map cell
        let (mx, my) = get_screen_to_map(heap, world.mouse_x, world.mouse_y);
        if -1 < mx && -1 < my {

            let midx = (my * 17 + mx) as usize;
            let uidx = cache.battle_cache.selected_unit as usize;

            // valid movement cell?
            for i in 0..heap.battle_data.battle_map_move_options.len() {
                if midx == heap.battle_data.battle_map_move_options[i] {
                    // move unit
                    cache.battle_cache.player_units[uidx].x = mx as u8;
                    cache.battle_cache.player_units[uidx].y = my as u8;
                    cache.battle_cache.player_units[uidx].animating = true;

                    // increment unit
                    increment_turn(cache, heap, world);
                    cache.uidata.battle_click_delay = 300;

                    break;
                }
            }

            // valid attack cell?
            for i in 0..heap.battle_data.battle_map_attack_options.len() {
                if midx == heap.battle_data.battle_map_attack_options[i] {
                    attack_idx = midx as i32;
                    break;
                }
            }
        }

    } else if mgfw::EVENT_INPUT_KEYBOARD_RELEASED_F == event_id {

        let options = [1, -16, 19, -15, 2];
        let mut found = false;
        let mut fx = 0;
        let mut fy = 0;

        let uidx = cache.battle_cache.selected_unit as usize;
        let pidx = cache.battle_cache.player_units[uidx].y as i32 * 17 + cache.battle_cache.player_units[uidx].x as i32;

        for j in 0..options.len() {
            for i in 0..heap.battle_data.battle_map_move_options.len() {
                let midx = heap.battle_data.battle_map_move_options[i] as i32;
                if pidx + options[j] == midx {
                    found = true;
                    fx = midx % 17;
                    fy = (midx - fx) / 17;
                }
            }
        }

        if found {
            // move unit
            cache.battle_cache.player_units[uidx].x = fx as u8;
            cache.battle_cache.player_units[uidx].y = fy as u8;
            cache.battle_cache.player_units[uidx].animating = true;

            // increment unit
            increment_turn(cache, heap, world);
            cache.uidata.battle_click_delay = 300;
        }

    } else if mgfw::EVENT_INPUT_KEYBOARD_RELEASED_A == event_id {

        // find weakest
        let mut shields = 0;
        for i in 0..heap.battle_data.battle_map_attack_options.len() {
            let midx = heap.battle_data.battle_map_attack_options[i] as i32;
            let mx = midx % 17;
            let my = (midx - mx) / 17;

            for j in 0..16 {
                if mx == cache.battle_cache.enemy_units[j].x as i32 && my == cache.battle_cache.enemy_units[j].y as i32 && 0 < cache.battle_cache.enemy_units[j].shields {
                    if -1 == attack_idx || shields > cache.battle_cache.enemy_units[j].shields {
                        shields = cache.battle_cache.enemy_units[j].shields;
                        attack_idx = midx;
                    }
                }
            }
        }
    }

    if -1 != attack_idx { attack_enemy(cache, heap, world, attack_idx); }

}


fn attack_enemy(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, attack_idx: i32) {
    let uidx = cache.battle_cache.selected_unit as usize;
    let hits = cache.battle_cache.player_units[uidx].shields as usize;
    let midx = attack_idx;
    let mx = midx % 17;
    let my = (midx - mx) / 17;

    // attack location
    cache.battle_cache.explosion_counter = u8::min(3, hits as u8);
    cache.battle_cache.explosion_frame = 0;
    let (sx, sy) = get_map_to_screen(mx, my);
    cache.battle_cache.explosion_screen_location = (sx - 16, sy - 16);
    let sx = cache.battle_cache.explosion_screen_location.0 as f32 + world.rnd() * 24.0 - 12.0;
    let sy = cache.battle_cache.explosion_screen_location.1 as f32 + world.rnd() * 12.0 - 6.0;
    world.entity_set_position_xy(cache.uidata.battle_explosion_ent as usize, sx, sy);

    // is this the freighter?
    if midx == 5*17-1 || midx == 5*17-2 || midx == 5*17-3 || midx == 6*17-2 {
        cache.battle_cache.trigger_enemy_surrender = true;

    } else {
        // hit enemy unit
        for j in 0..16 {
            if mx == cache.battle_cache.enemy_units[j].x as i32 && my == cache.battle_cache.enemy_units[j].y as i32 && 0 < cache.battle_cache.enemy_units[j].shields {

                let pidx = cache.battle_cache.player_units[uidx].y as i32 * 17 + cache.battle_cache.player_units[uidx].x as i32;
                let eidx = midx;
                let phit = if eidx == pidx - 1 || eidx == pidx + 1 || eidx == pidx + 17 || eidx == pidx - 16 || eidx == pidx - 17 || eidx == pidx - 18 {
                    0.75
                } else {
                    0.5
                };
                for _ in 0..hits {
                    if 0 < cache.battle_cache.enemy_units[j].shields && world.rnd() < phit {
                        cache.battle_cache.enemy_units[j].shields -= 1;
                    }
                }
                break;
            }
        }
    }
    
    cache.uidata.battle_click_delay = 1200;
    cache.uidata.battle_animation_lock = true;
}

// called at 1200 hz
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {

    if cache.uidata.battle_ui_reconstruct_avatars {
        cache.uidata.battle_ui_reconstruct_avatars = false;
        world.ugui.force_reconstruct(cache.uidata.battle_ui_avatar_player as usize);
        world.ugui.force_reconstruct(cache.uidata.battle_ui_avatar_enemy as usize);
    }

    if MENU_BATTLE_BASE != cache.uidata.curr_menu {
        world.entity_set_visibility(cache.uidata.battle_selector_ent as usize, false);
        return false;
    }
    
    if cache.uidata.battle_hold {
        cache.uidata.battle_hold = false;
        cache.uidata.battle_click_delay = 1200;
    }

    if 0 < cache.uidata.battle_click_delay {
        cache.uidata.battle_click_delay -= 1;
    }

    if 1 == cache.frame % 32 {
        // update cursor
        let selent = cache.uidata.battle_selector_ent as usize;
        world.entity_set_visibility(selent, false);
        let (mx, my) = get_screen_to_map(heap, world.mouse_x, world.mouse_y);
        if -1 < mx && -1 < my {
            let mask = [
                0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,
                0,0,0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,
                0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
                0,0,0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,
                0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,
                0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,
            ];
            if 1 == mask[(my * 17 + mx) as usize] {
                let (sx, sy) = get_map_to_screen(mx, my);
                world.entity_set_position_xy(selent, sx as f32, sy as f32);
                world.entity_set_visibility(selent, true);
            }
        }

        for i in 0..16 {
            if cache.battle_cache.player_units[i].animating {
                let e = cache.uidata.battle_player_entity_0 as usize + i + 1;
                let p = world.entity_get_position(e);
                let mx = cache.battle_cache.player_units[i].x as i32;
                let my = cache.battle_cache.player_units[i].y as i32;
                let (sx, sy) = get_map_to_screen(mx, my);
                let mut sx = sx as f32;
                let mut sy = sy as f32;
                let dx = sx - p.x;
                let dy = sy - p.y;
                let mut mag = (dx * dx + dy * dy).sqrt();
                if mag > 10.0 { mag = 10.0; }
                //let spd = 2.0;
                if 1.0 < mag {
                    let dx = dx / mag;// * spd;
                    let dy = dy / mag;// * spd;
                    sx = p.x + dx;
                    sy = p.y + dy;
                    world.entity_set_position_xy(e as usize, sx as f32, sy as f32);
                } else {
                    world.entity_set_position_xy(e as usize, sx as f32, sy as f32);
                    cache.battle_cache.player_units[i].animating = false;
                }
                let e = cache.uidata.battle_player_fighter_count_ent_0 as usize + i;
                world.entity_set_text(e, format!("{}", cache.battle_cache.player_units[i].shields));
                let w = world.text_get_width(e) as f32 * 0.5;
                world.entity_set_position_xy(e, sx - w, sy);
            }
            if cache.battle_cache.enemy_units[i].animating {
                let e = cache.uidata.battle_enemy_entity_0 as usize + i + 1;
                let p = world.entity_get_position(e);
                let mx = cache.battle_cache.enemy_units[i].x as i32;
                let my = cache.battle_cache.enemy_units[i].y as i32;
                let (sx, sy) = get_map_to_screen(mx, my);
                let mut sx = sx as f32;
                let mut sy = sy as f32;
                let dx = sx as f32 - p.x;
                let dy = sy as f32 - p.y;
                let mut mag = (dx * dx + dy * dy).sqrt();
                if mag > 10.0 { mag = 10.0; }
                //let spd = 2.0;
                if 1.0 < mag {
                    let dx = dx / mag;// * spd;
                    let dy = dy / mag;// * spd;
                    sx = p.x + dx;
                    sy = p.y + dy;
                    world.entity_set_position_xy(e as usize, sx as f32, sy as f32);
                } else {
                    world.entity_set_position_xy(e as usize, sx as f32, sy as f32);
                    cache.battle_cache.enemy_units[i].animating = false;
                }
                let e = cache.uidata.battle_enemy_fighter_count_ent_0 as usize + i;
                world.entity_set_text(e, format!("{}", cache.battle_cache.enemy_units[i].shields));
                let w = world.text_get_width(e) as f32 * 0.5;
                world.entity_set_position_xy(e, sx - w, sy);
            }        
        }

    }

    if 2 == cache.frame % 32 {
        if 0 < cache.battle_cache.explosion_counter {
            cache.battle_cache.explosion_frame += 1;
            if 9 >= cache.battle_cache.explosion_frame {
                world.entity_set_tilemap(cache.uidata.battle_explosion_ent as usize, cache.uidata.battle_explosion_tileset_ent as usize, 1, &vec![cache.battle_cache.explosion_frame as u16]);
                world.entity_set_visibility(cache.uidata.battle_explosion_ent as usize, true);
            } else {
                cache.battle_cache.explosion_frame = 0;
                cache.battle_cache.explosion_counter -= 1;
                let sx = cache.battle_cache.explosion_screen_location.0 as f32 + world.rnd() * 24.0 - 12.0;
                let sy = cache.battle_cache.explosion_screen_location.1 as f32 + world.rnd() * 12.0 - 6.0;
                world.entity_set_position_xy(cache.uidata.battle_explosion_ent as usize, sx, sy);

                if 0 == cache.battle_cache.explosion_counter {
                    world.entity_set_visibility(cache.uidata.battle_explosion_ent as usize, false);

                    // is this a forced surrender?
                    if cache.battle_cache.trigger_enemy_surrender {
                        perform_enemy_surrender(cache, heap, world);

                    } else if cache.battle_cache.trigger_player_surrender {
                        perform_player_surrender(cache, heap, world);
                        
                    } else {
                        // update sprite visibility
                        for i in 0..16 {
                            if 0 == cache.battle_cache.player_units[i].shields {
                                let e = cache.uidata.battle_player_entity_0 as usize + i + 1;
                                world.entity_set_visibility(e, false);
                                let e = cache.uidata.battle_player_fighter_count_ent_0 as usize + i;
                                world.entity_set_visibility(e, false);
                            }
                            if 0 == cache.battle_cache.enemy_units[i].shields {
                                let e = cache.uidata.battle_enemy_entity_0 as usize + i + 1;
                                world.entity_set_visibility(e, false);
                                let e = cache.uidata.battle_enemy_fighter_count_ent_0 as usize + i;
                                world.entity_set_visibility(e, false);
                            }
                        }

                        increment_turn(cache, heap, world);
                    }

                    cache.uidata.battle_animation_lock = false;
                    cache.uidata.battle_click_delay = 30;
                }
            }
        }
    }

    if 1 == cache.frame % 64 {
        // unit flash
        cache.battle_cache.selected_unit_flash += 1;
        if 5 == cache.battle_cache.selected_unit_flash {
            cache.battle_cache.selected_unit_flash = 0;
            cache.battle_cache.selected_unit_flash_visibile = !cache.battle_cache.selected_unit_flash_visibile;
            world.entity_set_visibility(cache.uidata.battle_selector_unit_ent as usize, cache.battle_cache.selected_unit_flash_visibile);
        }

    }

    // player move auto-battler
    if is_player_turn(cache) && !cache.uidata.battle_animation_lock && cache.battle_cache.auto_battle && 0 == cache.uidata.battle_click_delay {
        
        // any attack options available?
        if !heap.battle_data.battle_map_attack_options.is_empty() {
            // find weakest
            let mut attack_idx = -1;
            let mut shields = 0;
            for i in 0..heap.battle_data.battle_map_attack_options.len() {
                let midx = heap.battle_data.battle_map_attack_options[i] as i32;
                let mx = midx % 17;
                let my = (midx - mx) / 17;

                for j in 0..16 {
                    if mx == cache.battle_cache.enemy_units[j].x as i32 && my == cache.battle_cache.enemy_units[j].y as i32 && 0 < cache.battle_cache.enemy_units[j].shields {
                        if -1 == attack_idx || shields > cache.battle_cache.enemy_units[j].shields {
                            shields = cache.battle_cache.enemy_units[j].shields;
                            attack_idx = midx;
                        }
                    }
                }
            }

            if -1 != attack_idx { attack_enemy(cache, heap, world, attack_idx); }
        
        // else move forward
        } else {
            let options = [1, -16, 19, -15, 2];
            let mut found = false;
            let mut fx = 0;
            let mut fy = 0;

            let uidx = cache.battle_cache.selected_unit as usize;
            let pidx = cache.battle_cache.player_units[uidx].y as i32 * 17 + cache.battle_cache.player_units[uidx].x as i32;

            for j in 0..options.len() {
                for i in 0..heap.battle_data.battle_map_move_options.len() {
                    let midx = heap.battle_data.battle_map_move_options[i] as i32;
                    if pidx + options[j] == midx {
                        found = true;
                        fx = midx % 17;
                        fy = (midx - fx) / 17;
                    }
                }
            }

            // move unit or skip turn
            if found {
                cache.battle_cache.player_units[uidx].x = fx as u8;
                cache.battle_cache.player_units[uidx].y = fy as u8;
                cache.battle_cache.player_units[uidx].animating = true;
            }

            // increment unit
            increment_turn(cache, heap, world);
            cache.uidata.battle_click_delay = 300;
        }

    }

    // enemy move
    if !is_player_turn(cache) && !cache.uidata.battle_animation_lock && 0 == cache.uidata.battle_click_delay {

        // get mov and atk options
        get_enemy_move_attack_options(cache, heap, world);

        // any attack options? choose to attack 75% of the time
        let percentile = if heap.state_data.sectors[cache.state_cache.location as usize].enemy_aggressive { 1.0 } else { 0.75 };

        if !heap.battle_data.battle_map_attack_options.is_empty() && world.rnd() < percentile {
            let idx = world.rnd_range(0..heap.battle_data.battle_map_attack_options.len()) as usize;
            let midx = heap.battle_data.battle_map_attack_options[idx] as i32;
            let mx = midx % 17;
            let my = (midx - mx) / 17;

            let uidx = cache.battle_cache.turn_order[cache.battle_cache.turn_order_index as usize] as usize - 16;
            let hits = cache.battle_cache.enemy_units[uidx].shields as usize;
            
            // attack location
            cache.battle_cache.explosion_counter = u8::min(3, hits as u8);
            cache.battle_cache.explosion_frame = 0;
            let (sx, sy) = get_map_to_screen(mx, my);
            cache.battle_cache.explosion_screen_location = (sx - 16, sy - 16);
            let sx = cache.battle_cache.explosion_screen_location.0 as f32 + world.rnd() * 24.0 - 12.0;
            let sy = cache.battle_cache.explosion_screen_location.1 as f32 + world.rnd() * 12.0 - 6.0;
            world.entity_set_position_xy(cache.uidata.battle_explosion_ent as usize, sx, sy);

            if midx == 4*17+0 || midx == 4*17+1 || midx == 4*17+2 || midx == 5*17+1 {
                cache.battle_cache.trigger_player_surrender = true;
                
            } else {
                // hide enemy unit
                for j in 0..16 {
                    if mx == cache.battle_cache.player_units[j].x as i32 && my == cache.battle_cache.player_units[j].y as i32 && 0 < cache.battle_cache.player_units[j].shields {

                        let pidx = cache.battle_cache.player_units[j].y as i32 * 17 + cache.battle_cache.player_units[j].x as i32;
                        let eidx = cache.battle_cache.enemy_units[uidx].y as i32 * 17 + cache.battle_cache.enemy_units[uidx].x as i32;
                        let phit = if pidx == eidx - 1 || pidx == eidx + 1 || pidx == eidx + 17 || pidx == eidx - 16 || pidx == eidx - 17 || pidx == eidx - 18 {
                            0.75
                        } else {
                            0.5
                        };
                        for _ in 0..hits {
                            if 0 < cache.battle_cache.player_units[j].shields && world.rnd() < phit {
                                cache.battle_cache.player_units[j].shields -= 1;
                            }
                        }
                        break;
                    }
                }
            }

            heap.state_data.ships[0].fighters = count_fighters_player(cache);
            
            cache.uidata.battle_click_delay = 1200;
            cache.uidata.battle_animation_lock = true;

        } else {
            // determine new move location
            let mut idx = world.rnd_range(0..heap.battle_data.battle_map_move_options.len()) as usize;

            // prefer moving left
            if idx > heap.battle_data.battle_map_move_options.len() / 2 {
                idx = world.rnd_range(0..heap.battle_data.battle_map_move_options.len()) as usize;
                if idx > heap.battle_data.battle_map_move_options.len() / 2 {
                    idx = world.rnd_range(0..heap.battle_data.battle_map_move_options.len()) as usize;
                }
            }

            let idx = heap.battle_data.battle_map_move_options[idx] as i32;
            let mx = idx % 17;
            let my = (idx - mx) / 17;

            let uidx = cache.battle_cache.turn_order[cache.battle_cache.turn_order_index as usize] as usize - 16;

            // move unit
            cache.battle_cache.enemy_units[uidx].x = mx as u8;
            cache.battle_cache.enemy_units[uidx].y = my as u8;
            cache.battle_cache.enemy_units[uidx].animating = true;

            // increment unit
            increment_turn(cache, heap, world);
            cache.uidata.battle_click_delay = 300;
        }

    }


    return false;

}

fn perform_enemy_surrender(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    build_battle_ui_enemy_surrenders(cache, heap, world);
    state::claim_planet(cache, heap, world);
    state::update_galaxy_geometry(cache, heap, world);

    let nplanets = state::get_num_planets(cache);
    let pplanets = state::get_num_planets_player(heap);
    if pplanets == nplanets { build_battle_ui_game_win(cache, heap, world); }
}

fn perform_player_surrender(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    build_battle_ui_player_surrenders(cache, heap, world);
}

fn increment_turn(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    for _ in 0..32 {
        cache.battle_cache.turn_order_index = (cache.battle_cache.turn_order_index + 1) % 32;
        if SELECTED_UNIT_INVALID == cache.battle_cache.turn_order[cache.battle_cache.turn_order_index as usize] {
            cache.battle_cache.turn_order_index = 0;
        }

        let idx = cache.battle_cache.turn_order[cache.battle_cache.turn_order_index as usize];
        
        if idx > 15 {
            // enemy turn option
            if 0 < cache.battle_cache.enemy_units[idx as usize - 16].shields { break; }
        
        } else {
            // player turn option
            if 0 < cache.battle_cache.player_units[idx as usize].shields { break; }

        }
    }

    highlight_unit_for_turn(cache, heap, world);
}


fn is_player_turn(cache: &GameData) -> bool {
    cache.battle_cache.whos_turn_is_it == BATTLE_TURN_PLAYER
}


fn highlight_unit_for_turn(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    let tidx = cache.battle_cache.turn_order_index as usize;

    cache.battle_cache.whos_turn_is_it = BATTLE_TURN_PLAYER;
    if cache.battle_cache.turn_order[tidx] > 15 {
        cache.battle_cache.whos_turn_is_it = BATTLE_TURN_ENEMY;
    }

    let sx;
    let sy;

    if is_player_turn(cache) {
        let idx = cache.battle_cache.turn_order[tidx];
        cache.battle_cache.selected_unit = idx;
        sx = cache.battle_cache.player_units[idx as usize].x as i32;
        sy = cache.battle_cache.player_units[idx as usize].y as i32;
        world.entity_set_billboard(cache.uidata.battle_selector_unit_ent as usize, String::from("assets/sel_fighter_player.png"));

    } else {
        let idx = cache.battle_cache.turn_order[tidx] - 16;
        sx = cache.battle_cache.enemy_units[idx as usize].x as i32;
        sy = cache.battle_cache.enemy_units[idx as usize].y as i32;
        world.entity_set_billboard(cache.uidata.battle_selector_unit_ent as usize, String::from("assets/sel_fighter_enemy.png"));
    }
    
    // move background flash entity
    cache.battle_cache.selected_unit_flash = 0;
    cache.battle_cache.selected_unit_flash_visibile = false;
    world.entity_set_visibility(cache.uidata.battle_selector_unit_ent as usize, cache.battle_cache.selected_unit_flash_visibile);

    let (sx, sy) = get_map_to_screen(sx, sy);
    world.entity_set_position_xy(cache.uidata.battle_selector_unit_ent as usize, sx as f32, sy as f32);

    if is_player_turn(cache) {
        show_move_attack_options(cache, heap, world);
    }

    let tmc = cache.battle_cache.turn_count as usize;
    let tm2 = cache.battle_cache.turn_count as f32 / 2.0;

    // update fighter group counts
    for i in 0..cache.battle_cache.player_units_count as usize {
        let e = cache.uidata.battle_player_fighter_count_ent_0 as usize + i;
        world.entity_set_text(e, format!("{}", cache.battle_cache.player_units[i].shields));
    }
    
    for i in 0..16 {
        let e = cache.uidata.battle_enemy_fighter_count_ent_0 as usize + i;
        world.entity_set_text(e, format!("{}", cache.battle_cache.enemy_units[i].shields));
    }

    // update turn order visualizer
    let y0 = 16.0;
    for i in 0..tmc {
        let e = cache.uidata.battle_turn_order_ent as usize + i;
        let tidx = cache.battle_cache.turn_order[i] as usize;

        if 16 > tidx {
            if 0 == cache.battle_cache.player_units[tidx].shields { world.entity_set_alpha(e, 0.25); }
        } else {
            if 0 == cache.battle_cache.enemy_units[tidx-16].shields { world.entity_set_alpha(e, 0.25); }
        }

        let e = cache.uidata.battle_turn_order_count_ent as usize + i;
        if 16 > tidx {
            world.entity_set_text(e, format!("{}", cache.battle_cache.player_units[tidx].shields));
            if 0 == cache.battle_cache.player_units[tidx].shields { world.entity_set_visibility(e, false); }
        } else {
            world.entity_set_text(e, format!("{}", cache.battle_cache.enemy_units[tidx-16].shields));
            if 0 == cache.battle_cache.enemy_units[tidx-16].shields { world.entity_set_visibility(e, false); }
        }
        let w = world.text_get_width(e) as f32 * 0.5;
        let x0 = SCREEN_XRES_HALF as f32 + (i as f32 - tm2) * 26.0 + 13.0;
        world.entity_set_position_xy(e, x0 - w, 16.0);
    }

    let x0 = SCREEN_XRES_HALF as f32 + (cache.battle_cache.turn_order_index as f32 - tm2) * 26.0 + 13.0;
    let y0 = y0 + 8.0;
    let e = cache.uidata.battle_turn_order_selector_ent as usize;
    world.entity_set_position_xy(e, x0, y0);

    // update UI unit counts
    let pfighters = count_fighters_player(cache);
    let efighters = count_fighters_enemy(cache);

    world.ugui.set_text(cache.uidata.battle_ui_player_fighter_count as usize, &format!("{pfighters}"));
    world.ugui.force_reconstruct(cache.uidata.battle_ui_player_fighter_count as usize);

    world.ugui.set_text(cache.uidata.battle_ui_enemy_fighter_count as usize, &format!("{efighters}"));
    world.ugui.force_reconstruct(cache.uidata.battle_ui_enemy_fighter_count as usize);

    if 0 == pfighters { build_battle_ui_game_over(cache, heap, world); }

    
}

fn count_fighters_player(cache: &mut GameData) -> i32 {
    let mut pfighters = 0;
    for i in 0..cache.battle_cache.player_units_count as usize {
        pfighters += cache.battle_cache.player_units[i].shields;
    }
    pfighters
}

fn count_fighters_enemy(cache: &mut GameData) -> i32 {
    let mut efighters = 0;
    for i in 0..16 {
        efighters += cache.battle_cache.enemy_units[i].shields;
    }
    efighters
}

pub fn check_callback_battle(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, c: &Callback) -> bool {
    
    let pfighters = count_fighters_player(cache) as f32;
    let efighters = count_fighters_enemy(cache) as f32;

    match c.id {
        CALLBACK_BATTLE_DEMAND_TRIBUTE => {
            // does enemy offer tribute?
            if efighters > pfighters * 0.33 {
                build_battle_ui_enemy_rejects_tribute_demand(cache, heap, world);
            } else {
                build_battle_ui_enemy_offers_tribute(cache, heap, world);
            }
        }
        CALLBACK_BATTLE_DEMAND_SURRENDER => {
            // does enemy offer surrender?
            if efighters > pfighters * 0.1 {
                build_battle_ui_enemy_rejects_surrender_demand(cache, heap, world);
            } else {
                perform_enemy_surrender(cache, heap, world);
            }
        }
        CALLBACK_BATTLE_OFFER_TRIBUTE => {
            // does enemy accept tribute?
            if heap.state_data.sectors[cache.state_cache.location as usize].enemy_aggressive {
                build_battle_ui_enemy_rejects_player_tribute(cache, heap, world);
            } else {
                build_battle_ui_enemy_accepts_player_tribute(cache, heap, world);
            }
        }
        CALLBACK_BATTLE_SURRENDER => perform_player_surrender(cache, heap, world),
        CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE_DEMAND => build_battle_ui_base(cache, heap, world),
        CALLBACK_BATTLE_REJECT_PLAYER_SURRENDER_DEMAND => build_battle_ui_base(cache, heap, world),
        CALLBACK_BATTLE_REJECT_ENEMY_TRIBUTE => build_battle_ui_base(cache, heap, world),
        CALLBACK_BATTLE_ACCEPT_ENEMY_TRIBUTE => {
            let ratio = 0.1 * cache.battle_cache.tribute_multiplier;
            let tribute_cash = (cache.battle_cache.enemy_tribute_cash as f32 * ratio).floor() as i32;
            cache.state_cache.cash += tribute_cash;
            let tribute_fighters = (count_fighters_enemy(cache) as f32 * ratio).floor() as i32;
            heap.state_data.ships[0].fighters += tribute_fighters;
            if heap.state_data.ships[0].fighters > 3000 { heap.state_data.ships[0].fighters = 3000; }
            heap.state_data.sectors[cache.state_cache.location as usize].tribute_multiplier *= 0.5;
            get_hold_tribute_from_enemey(cache, heap, ratio);
            battle_close(cache, heap, world);
        }
        CALLBACK_BATTLE_ACCEPT_PLAYER_TRIBUTE | CALLBACK_BATTLE_CLOSE_BATTLE => {
            battle_close(cache, heap, world);
        }
        CALLBACK_BATTLE_REJECT_PLAYER_TRIBUTE => build_battle_ui_base(cache, heap, world),
        CALLBACK_BATTLE_AUTO_TOGGLE => {
            cache.battle_cache.auto_battle = ! cache.battle_cache.auto_battle;
            let txt = if cache.battle_cache.auto_battle { "ON" } else { "OFF" };
            world.ugui.set_text(cache.uidata.battle_ui_auto_toggle as usize, &format!("Auto: {txt}"));
            world.ugui.force_reconstruct(cache.uidata.battle_ui_auto_toggle as usize);
        }
        _ => return false,
    }


    true
}

fn get_hold_tribute_from_enemey(cache: &mut GameData, heap: &mut GameDataHeap, ratio: f32) {
    
    let mut tribute_colonists = (cache.battle_cache.enemy_tribute_colonists as f32 * ratio).floor() as i32;
    let mut tribute_bios = (cache.battle_cache.enemy_tribute_bios as f32 * ratio).floor() as i32;
    let mut tribute_fuel = (cache.battle_cache.enemy_tribute_fuel as f32 * ratio).floor() as i32;
    let mut tribute_ice = (cache.battle_cache.enemy_tribute_ice as f32 * ratio).floor() as i32;
    let mut tribute_mach = (cache.battle_cache.enemy_tribute_mach as f32 * ratio).floor() as i32;
    let mut tribute_meds = (cache.battle_cache.enemy_tribute_meds as f32 * ratio).floor() as i32;

    let mut step_size = 10;
    loop {
        let hold_colonists = heap.state_data.ships[0].hold_colonists;
        let hold_bios = heap.state_data.ships[0].hold_bios;
        let hold_fuel = heap.state_data.ships[0].hold_fuel;
        let hold_ice = heap.state_data.ships[0].hold_ice;
        let hold_mach = heap.state_data.ships[0].hold_mach;
        let hold_meds = heap.state_data.ships[0].hold_meds;
        //
        let planet_colonists = tribute_colonists;
        let planet_bios = tribute_bios;
        let planet_fuel = tribute_fuel;
        let planet_ice = tribute_ice;
        let planet_mach = tribute_mach;
        let planet_meds = tribute_meds;
        //
        let ratio_colonists = 0.005;
        let ratio_bios = 0.003;
        let ratio_ice = 0.004;
        let ratio_fuel = 0.002;
        let ratio_mach = 0.003;
        let ratio_meds = 0.001;

        let holds = heap.state_data.ships[0].holds as f32;
        let mut keep_going = false;

        if step_size <= planet_colonists {
            let hold_tot = (hold_colonists + step_size) as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_colonists += step_size;
                tribute_colonists -= step_size;
                keep_going = true;
            }
        }
        if step_size <= planet_bios {
            let hold_tot = hold_colonists as f32 * ratio_colonists + (hold_bios + step_size) as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_bios += step_size;
                tribute_bios -= step_size;
                keep_going = true;
            }
        }
        if step_size <= planet_fuel {
            let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + (hold_fuel + step_size) as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_fuel += step_size;
                tribute_fuel -= step_size;
                keep_going = true;
            }
        }
        if step_size <= planet_ice {
            let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + (hold_ice + step_size) as f32 * ratio_ice + hold_mach as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_ice += step_size;
                tribute_ice -= step_size;
                keep_going = true;
            }
        }
        if step_size <= planet_mach {
            let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + (hold_mach + step_size) as f32 * ratio_mach + hold_meds as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_mach += step_size;
                tribute_mach -= step_size;
                keep_going = true;
            }
        }
        if step_size <= planet_meds {
            let hold_tot = hold_colonists as f32 * ratio_colonists + hold_bios as f32 * ratio_bios + hold_fuel as f32 * ratio_fuel + hold_ice as f32 * ratio_ice + hold_mach as f32 * ratio_mach + (hold_meds + step_size) as f32 * ratio_meds;
            if hold_tot < holds {
                heap.state_data.ships[0].hold_meds += step_size;
                tribute_meds -= step_size;
                keep_going = true;
            }
        }
        if !keep_going {
            step_size -= 1;
            if 0 == step_size { break; }
        }
    }

}