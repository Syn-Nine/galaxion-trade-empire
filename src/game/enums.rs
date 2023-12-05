#![allow(dead_code)]

pub const SCREEN_XRES: usize = 900;
pub const SCREEN_XRES_HALF: usize = SCREEN_XRES / 2;
pub const SCREEN_YRES: usize = 480;
pub const SCREEN_YRES_HALF: usize = SCREEN_YRES / 2;

pub const SPLIT_H: u8 = 0;
pub const SPLIT_V: u8 = 1;


// game enums

pub const PLANET_CLASS_INVALID: u8 = 0;
pub const PLANET_CLASS_LUSH: u8 = 1;
pub const PLANET_CLASS_WATER: u8 = 2;
pub const PLANET_CLASS_DESERT: u8 = 3;
pub const PLANET_CLASS_VOLCANO: u8 = 4;

pub const STATION_CLASS_INVALID: u8 = 0;
pub const STATION_CLASS_PORT: u8 = 1;
pub const STATION_CLASS_FEDERAL: u8 = 2;

pub const RESEARCH_INVALID: u8 = 0;
pub const RESEARCH_AGRICULTURE_I: u8 = 1;
pub const RESEARCH_BIOLOGY_I: u8 = 2;
pub const RESEARCH_ENERGY_I: u8 = 3;
pub const RESEARCH_KNOWLEDGE_I: u8 = 4;
pub const RESEARCH_MEDICINE_I: u8 = 5;
pub const RESEARCH_MILITARY_I: u8 = 6;
pub const RESEARCH_TECHNOLOGY_I: u8 = 7;
pub const RESEARCH_AGRICULTURE_II: u8 = 8;
pub const RESEARCH_BIOLOGY_II: u8 = 9;
pub const RESEARCH_ENERGY_II: u8 = 10;
pub const RESEARCH_KNOWLEDGE_II: u8 = 11;
pub const RESEARCH_MEDICINE_II: u8 = 12;
pub const RESEARCH_MILITARY_II: u8 = 13;
pub const RESEARCH_TECHNOLOGY_II: u8 = 14;
//
pub const RESEARCH_COUNT: usize = 15;
pub const ALL_RESEARCH_COMPLETE: u8 = RESEARCH_COUNT as u8;
//
pub const RESEARCH_NOT_STARTED: u8 = 0;
pub const RESEARCH_IN_PROGRESS: u8 = 1;
pub const RESEARCH_NOT_AVAILABLE: u8 = 2;
pub const RESEARCH_COMPLETE: u8 = 3;
