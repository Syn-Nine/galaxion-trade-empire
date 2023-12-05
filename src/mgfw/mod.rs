#![allow(dead_code)]

pub mod cache;
pub mod ecs;
mod fonts;
mod support;

use crate::game::GameWrapper;
use cache::CacheManager;
use std::collections::VecDeque;
use support::Gl;
use gilrs::Gilrs;


#[allow(unused_imports)]
use glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent, MouseScrollDelta};
use glutin::event_loop::EventLoop;
use glutin::window::{Icon, WindowBuilder, Fullscreen};
use glutin::ContextBuilder;

#[allow(dead_code)]
pub const PI: f64 = 3.1415926535897932384626433;
const WINDOW_SCALE: f64 = 2.0;
const WINDOW_FULLSCREEN: bool = false;

#[allow(dead_code)]
pub fn deg2rad(val: f32) -> f32 {
    val * PI as f32 / 180.0
}

struct CoreData {
    initialized: bool,
    running: bool,
    shutdown: bool,
    start_time: std::time::Instant,
    last_update: std::time::Instant,
    last_render: std::time::Instant,
    last_physics: std::time::Instant,
    blown_update_frames: usize,
    blown_update_frames_expected: usize,
    blown_update_frames_significant: usize,
    count_update_frames: usize,
    blown_render_frames: usize,
    count_render_frames: usize,
    completed_first_frame: bool,
    update_frame_load: f64,
    render_frame_load: f64,
    scale_factor: f64,
    ready_to_quit: bool,
    quit_requested: bool,
}

#[allow(dead_code)]
pub const EVENT_INVALID: u8 = 0;
pub const EVENT_INPUT_MOUSE_BUTTON_DOWN: u8 = 1;
pub const EVENT_INPUT_MOUSE_BUTTON_UP: u8 = 2;
pub const EVENT_INPUT_MOUSE_BUTTON_LEFT_UP: u8 = 3;
pub const EVENT_INPUT_MOUSE_BUTTON_LEFT_DOWN: u8 = 4;
pub const EVENT_INPUT_MOUSE_BUTTON_RIGHT_UP: u8 = 5;
pub const EVENT_INPUT_MOUSE_BUTTON_RIGHT_DOWN: u8 = 6;
pub const EVENT_INPUT_MOUSE_WHEEL_UP: u8 = 7;
pub const EVENT_INPUT_MOUSE_WHEEL_DOWN: u8 = 8;

pub const EVENT_INPUT_GAMEPAD_AXIS_MOVEMENT: u8 = 10;
pub const EVENT_INPUT_GAMEPAD_PRESSED_A: u8 = 11;
pub const EVENT_INPUT_GAMEPAD_PRESSED_B: u8 = 12;
pub const EVENT_INPUT_GAMEPAD_PRESSED_X: u8 = 13;
pub const EVENT_INPUT_GAMEPAD_PRESSED_Y: u8 = 14;
pub const EVENT_INPUT_GAMEPAD_RELEASED_A: u8 = 15;
pub const EVENT_INPUT_GAMEPAD_RELEASED_B: u8 = 16;
pub const EVENT_INPUT_GAMEPAD_RELEASED_X: u8 = 17;
pub const EVENT_INPUT_GAMEPAD_RELEASED_Y: u8 = 18;

pub const EVENT_INPUT_KEYBOARD_PRESSED_ESCAPE: u8 = 20;
pub const EVENT_INPUT_KEYBOARD_PRESSED_UP: u8 = 21;
pub const EVENT_INPUT_KEYBOARD_PRESSED_DOWN: u8 = 22;
pub const EVENT_INPUT_KEYBOARD_PRESSED_LEFT: u8 = 23;
pub const EVENT_INPUT_KEYBOARD_PRESSED_RIGHT: u8 = 24;
pub const EVENT_INPUT_KEYBOARD_PRESSED_SPACE: u8 = 25;
pub const EVENT_INPUT_KEYBOARD_PRESSED_BACKSPACE: u8 = 26;
pub const EVENT_INPUT_KEYBOARD_PRESSED_TAB: u8 = 27;
pub const EVENT_INPUT_KEYBOARD_PRESSED_LCTRL: u8 = 28;
pub const EVENT_INPUT_KEYBOARD_PRESSED_LSHIFT: u8 = 29;
pub const EVENT_INPUT_KEYBOARD_PRESSED_PLUS: u8 = 30;
pub const EVENT_INPUT_KEYBOARD_PRESSED_MINUS: u8 = 31;
pub const EVENT_INPUT_KEYBOARD_PRESSED_LALT: u8 = 32;

pub const EVENT_INPUT_KEYBOARD_PRESSED_0: u8 = 48;
pub const EVENT_INPUT_KEYBOARD_PRESSED_9: u8 = 57;

pub const EVENT_INPUT_KEYBOARD_PRESSED_A: u8 = 65;
pub const EVENT_INPUT_KEYBOARD_PRESSED_Z: u8 = 90;
pub const EVENT_INPUT_KEYBOARD_RELEASED_0: u8 = 148;
pub const EVENT_INPUT_KEYBOARD_RELEASED_1: u8 = 149;
pub const EVENT_INPUT_KEYBOARD_RELEASED_2: u8 = 150;
pub const EVENT_INPUT_KEYBOARD_RELEASED_3: u8 = 151;
pub const EVENT_INPUT_KEYBOARD_RELEASED_4: u8 = 152;
pub const EVENT_INPUT_KEYBOARD_RELEASED_5: u8 = 153;
pub const EVENT_INPUT_KEYBOARD_RELEASED_6: u8 = 154;
pub const EVENT_INPUT_KEYBOARD_RELEASED_7: u8 = 155;
pub const EVENT_INPUT_KEYBOARD_RELEASED_8: u8 = 156;
pub const EVENT_INPUT_KEYBOARD_RELEASED_9: u8 = 157;

pub const EVENT_INPUT_KEYBOARD_RELEASED_A: u8 = 165;
pub const EVENT_INPUT_KEYBOARD_RELEASED_B: u8 = 166;
pub const EVENT_INPUT_KEYBOARD_RELEASED_C: u8 = 167;
pub const EVENT_INPUT_KEYBOARD_RELEASED_D: u8 = 168;
pub const EVENT_INPUT_KEYBOARD_RELEASED_E: u8 = 169;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F: u8 = 170;
pub const EVENT_INPUT_KEYBOARD_RELEASED_G: u8 = 171;
pub const EVENT_INPUT_KEYBOARD_RELEASED_H: u8 = 172;
pub const EVENT_INPUT_KEYBOARD_RELEASED_I: u8 = 173;
pub const EVENT_INPUT_KEYBOARD_RELEASED_J: u8 = 174;
pub const EVENT_INPUT_KEYBOARD_RELEASED_K: u8 = 175;
pub const EVENT_INPUT_KEYBOARD_RELEASED_L: u8 = 176;
pub const EVENT_INPUT_KEYBOARD_RELEASED_M: u8 = 177;
pub const EVENT_INPUT_KEYBOARD_RELEASED_N: u8 = 178;
pub const EVENT_INPUT_KEYBOARD_RELEASED_O: u8 = 179;
pub const EVENT_INPUT_KEYBOARD_RELEASED_P: u8 = 180;
pub const EVENT_INPUT_KEYBOARD_RELEASED_Q: u8 = 181;
pub const EVENT_INPUT_KEYBOARD_RELEASED_R: u8 = 182;
pub const EVENT_INPUT_KEYBOARD_RELEASED_S: u8 = 183;
pub const EVENT_INPUT_KEYBOARD_RELEASED_T: u8 = 184;
pub const EVENT_INPUT_KEYBOARD_RELEASED_U: u8 = 185;
pub const EVENT_INPUT_KEYBOARD_RELEASED_V: u8 = 186;
pub const EVENT_INPUT_KEYBOARD_RELEASED_W: u8 = 187;
pub const EVENT_INPUT_KEYBOARD_RELEASED_X: u8 = 188;
pub const EVENT_INPUT_KEYBOARD_RELEASED_Y: u8 = 189;
pub const EVENT_INPUT_KEYBOARD_RELEASED_Z: u8 = 190;

pub const EVENT_INPUT_KEYBOARD_RELEASED_LALT: u8 = 118;
pub const EVENT_INPUT_KEYBOARD_RELEASED_GRAVE: u8 = 119;
pub const EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE: u8 = 120;
pub const EVENT_INPUT_KEYBOARD_RELEASED_UP: u8 = 121;
pub const EVENT_INPUT_KEYBOARD_RELEASED_DOWN: u8 = 122;
pub const EVENT_INPUT_KEYBOARD_RELEASED_LEFT: u8 = 123;
pub const EVENT_INPUT_KEYBOARD_RELEASED_RIGHT: u8 = 124;
pub const EVENT_INPUT_KEYBOARD_RELEASED_SPACE: u8 = 125;
pub const EVENT_INPUT_KEYBOARD_RELEASED_PGUP: u8 = 126;
pub const EVENT_INPUT_KEYBOARD_RELEASED_PGDN: u8 = 127;
pub const EVENT_INPUT_KEYBOARD_RELEASED_BACKSPACE: u8 = 128;
pub const EVENT_INPUT_KEYBOARD_RELEASED_TAB: u8 = 129;
pub const EVENT_INPUT_KEYBOARD_RELEASED_LCTRL: u8 = 130;
pub const EVENT_INPUT_KEYBOARD_RELEASED_LSHIFT: u8 = 131;
pub const EVENT_INPUT_KEYBOARD_RELEASED_PLUS: u8 = 132;
pub const EVENT_INPUT_KEYBOARD_RELEASED_MINUS: u8 = 133;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F1: u8 = 134;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F2: u8 = 135;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F3: u8 = 136;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F4: u8 = 137;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F5: u8 = 138;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F6: u8 = 139;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F7: u8 = 140;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F8: u8 = 141;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F9: u8 = 142;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F10: u8 = 143;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F11: u8 = 144;
pub const EVENT_INPUT_KEYBOARD_RELEASED_F12: u8 = 145;
pub const EVENT_INPUT_KEYBOARD_RELEASED_INSERT: u8 = 146;
pub const EVENT_INPUT_KEYBOARD_RELEASED_DELETE: u8 = 147;


#[allow(dead_code)]
pub struct Core {
    data: *mut CoreData,
    // WARNING: Anything below this line is not in cache!
    pub windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    gl: std::boxed::Box<Gl>,
    game: std::boxed::Box<GameWrapper>,
    cache: std::boxed::Box<CacheManager>,
    world: std::boxed::Box<ecs::World>,
    render_system: std::boxed::Box<ecs::RenderSystem>,
    physics_system: std::boxed::Box<ecs::PhysicsSystem>,
    easing_system: std::boxed::Box<ecs::EasingSystem>,
    events: std::boxed::Box<VecDeque<u8>>,
    gamepad:std::boxed::Box<Gilrs>,
    show_window: bool,
}

impl Core {
    pub fn new(title: &str, xres: i32, yres: i32, el: &EventLoop<()>) -> Core {
        log(format!("Constructing MGFW Core"));

        // Construct a new RGB ImageBuffer with the specified width and height.
        let icon: image::RgbaImage = image::open("assets/mgfw/mgfw_64_trim.ico")
            .unwrap()
            .to_rgba8();
        let w = icon.dimensions().0 as u32;
        let h = icon.dimensions().1 as u32;
        let b = Some(Icon::from_rgba(icon.into_vec(), w, h).unwrap());

        // img.into_raw().as_ptr() as *const _,

        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(false)
            .with_window_icon(b)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                xres as f64 * WINDOW_SCALE,
                yres as f64 * WINDOW_SCALE,
            ))
            .with_visible(false)
            .with_fullscreen(match WINDOW_FULLSCREEN {
                false => None,
                true => Some(Fullscreen::Borderless(None)),
            });

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(2)
            .build_windowed(window, &el)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        windowed_context.window().set_outer_position(glutin::dpi::LogicalPosition::new(60.0, 20.0));

        let scale_factor = windowed_context.window().scale_factor();
        //windowed_context.window().set_cursor_visible(false);

        let start_time = std::time::Instant::now();

        let gl = Box::new(support::load(
            &windowed_context.context(),
            xres,
            yres,
            (scale_factor * WINDOW_SCALE) as f32,
        ));
        let mut cache = Box::new(CacheManager::new());

        // force clear the display buffers
        gl.clear_frame();
        windowed_context.swap_buffers().unwrap();
        gl.clear_frame();
        windowed_context.swap_buffers().unwrap();

        let sz_bytes = std::mem::size_of::<CoreData>();
        let data = cache.allocate(sz_bytes) as *mut CoreData;
        unsafe {
            *data = CoreData {
                running: false,
                last_update: start_time,
                last_render: start_time,
                last_physics: start_time,
                initialized: false,
                shutdown: false,
                blown_update_frames: 0,
                blown_update_frames_expected: 0,
                blown_update_frames_significant: 0,
                count_update_frames: 0,
                blown_render_frames: 0,
                count_render_frames: 0,
                completed_first_frame: false,
                start_time,
                update_frame_load: 0.0,
                render_frame_load: 0.0,
                scale_factor,
                ready_to_quit: false,
                quit_requested: false,
            };
        }

        let world = Box::new(ecs::World::new(&mut cache));
        let render_system = Box::new(ecs::RenderSystem::new(&mut cache, &gl));
        let physics_system = Box::new(ecs::PhysicsSystem::new(&mut cache));
        let easing_system = Box::new(ecs::EasingSystem::new(&mut cache));
        let game = Box::new(GameWrapper::new(&mut cache));
        let events = Box::new(VecDeque::new());
        //let mut gilrs = Gilrs::new().unwrap();
        let gamepad = Box::new(Gilrs::new().unwrap());


        cache.print_loading();

        Core {
            windowed_context,
            gl,
            data,
            game,
            cache,
            world,
            render_system,
            physics_system,
            easing_system,
            events,
            gamepad,
            show_window: false,
        }
    }

    pub fn check_events(&mut self, event: &glutin::event::Event<()>) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        if !cache.initialized {
            self.initialize();
        }

        //log(format!("{:?}", event));
        match event {
            Event::LoopDestroyed => cache.quit_requested = true,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    cache.scale_factor = *scale_factor;
                }
                WindowEvent::Resized(physical_size) => self.windowed_context.resize(*physical_size),
                WindowEvent::CloseRequested => cache.quit_requested = true,
                WindowEvent::CursorMoved { position, .. } => {
                    self.update_mouse_xy(
                        (position.x / (cache.scale_factor * WINDOW_SCALE)) as i32,
                        (position.y / (cache.scale_factor * WINDOW_SCALE)) as i32,
                    );
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    self.update_mouse_wheel(delta);
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    self.update_mouse_button(button, state);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    self.update_keyboard_input(&input);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => self.render(std::time::Instant::now()),
            _ => (),
        }

        /*for (_id, gamepad) in self.gamepad.gamepads() {
            println!("{} is {:?}", gamepad.name(), gamepad.power_info());
        }*/

        // check for gamepad events
        self.update_gamepad_input();

        cache.quit_requested |= self.game.quit_requested();
 
        if !cache.quit_requested {
            self.update();
        } else {
            if !cache.shutdown {
                self.shutdown();
            }
        }
        
    }

    fn update_gamepad_input(&mut self) {
        while let Some(gilrs::Event { id, event, time }) = self.gamepad.next_event() {
            //println!("{:?} New event from {}: {:?}", time, id, event);
            match event {
                gilrs::EventType::AxisChanged(axis, offset, code) => {
                    //println!("Axis changed, {:?}, {:?}, {:?}", a, b, c),
                    match axis {
                        gilrs::Axis::LeftStickX => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_AXIS_MOVEMENT);
                            self.world.gamepad_x = offset;
                        }
                        gilrs::Axis::LeftStickY => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_AXIS_MOVEMENT);
                            self.world.gamepad_y = offset;
                        }
                        _ => (),
                    }

                },
                gilrs::EventType::ButtonPressed(button, code) => {
                    println!("Button pressed, {:?}, {:?}", button, code);
                    match button {
                        gilrs::Button::South => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_PRESSED_A);
                        }
                        gilrs::Button::East => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_PRESSED_B);
                        }
                        gilrs::Button::North => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_PRESSED_Y);
                        }
                        gilrs::Button::West => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_PRESSED_X);
                        }
                        _ => (),
                    }
                },
                gilrs::EventType::ButtonReleased(button, code) => {
                    println!("Button released, {:?}, {:?}", button, code);
                    match button {
                        gilrs::Button::South => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_RELEASED_A);
                        }
                        gilrs::Button::East => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_RELEASED_B);
                        }
                        gilrs::Button::North => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_RELEASED_Y);
                        }
                        gilrs::Button::West => {
                            self.events.push_back(EVENT_INPUT_GAMEPAD_RELEASED_X);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }

    }

    fn update_keyboard_input(&mut self, input: &KeyboardInput) {
        if ElementState::Pressed == input.state {
            match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_ESCAPE)
                }
                Some(VirtualKeyCode::Up) => self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_UP),
                Some(VirtualKeyCode::Down) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_DOWN)
                }
                Some(VirtualKeyCode::Left) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_LEFT)
                }
                Some(VirtualKeyCode::Right) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_RIGHT)
                }
                Some(VirtualKeyCode::Space) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_SPACE)
                }
                Some(VirtualKeyCode::Tab) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_TAB)
                }
                Some(VirtualKeyCode::LControl) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_LCTRL)
                }
                Some(VirtualKeyCode::LShift) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_LSHIFT)
                }
                Some(VirtualKeyCode::LAlt) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_LALT)
                }
                Some(VirtualKeyCode::NumpadAdd) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_PLUS)
                }
                Some(VirtualKeyCode::NumpadSubtract) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_MINUS)
                }
                Some(VirtualKeyCode::Minus) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_MINUS)
                }
                _ => (),
            }
        } else if ElementState::Released == input.state {
            match input.virtual_keycode {
                Some(VirtualKeyCode::Grave) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_GRAVE)
                }
                Some(VirtualKeyCode::Escape) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE)
                }
                Some(VirtualKeyCode::Up) => self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_UP),
                Some(VirtualKeyCode::Down) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_DOWN)
                }
                Some(VirtualKeyCode::Left) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_LEFT)
                }
                Some(VirtualKeyCode::Right) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_RIGHT)
                }
                Some(VirtualKeyCode::Space) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_SPACE)
                }
                Some(VirtualKeyCode::PageUp) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_PGUP)
                }
                Some(VirtualKeyCode::PageDown) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_PGDN)
                }
                Some(VirtualKeyCode::Back) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_BACKSPACE)
                }
                Some(VirtualKeyCode::Tab) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_TAB)
                }
                Some(VirtualKeyCode::LControl) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_LCTRL)
                }
                Some(VirtualKeyCode::LShift) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_LSHIFT)
                }
                Some(VirtualKeyCode::LAlt) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_LALT)
                }
                Some(VirtualKeyCode::NumpadAdd) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_PLUS)
                }
                Some(VirtualKeyCode::NumpadSubtract) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_MINUS)
                }
                Some(VirtualKeyCode::Minus) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_MINUS)
                }
                Some(VirtualKeyCode::F1) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F1)
                }
                Some(VirtualKeyCode::F2) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F2)
                }
                Some(VirtualKeyCode::F3) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F3)
                }
                Some(VirtualKeyCode::F4) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F4)
                }
                Some(VirtualKeyCode::F5) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F5)
                }
                Some(VirtualKeyCode::F6) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F6)
                }
                Some(VirtualKeyCode::F7) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F7)
                }
                Some(VirtualKeyCode::F8) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F8)
                }
                Some(VirtualKeyCode::F9) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F9)
                }
                Some(VirtualKeyCode::F10) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F10)
                }
                Some(VirtualKeyCode::F11) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F11)
                }
                Some(VirtualKeyCode::F12) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F12)
                }
                Some(VirtualKeyCode::Insert) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_INSERT)
                }
                Some(VirtualKeyCode::Delete) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_DELETE)
                }
                Some(VirtualKeyCode::Key0) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0)
                }
                Some(VirtualKeyCode::Key1) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 1)
                }
                Some(VirtualKeyCode::Key2) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 2)
                }
                Some(VirtualKeyCode::Key3) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 3)
                }
                Some(VirtualKeyCode::Key4) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 4)
                }
                Some(VirtualKeyCode::Key5) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 5)
                }
                Some(VirtualKeyCode::Key6) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 6)
                }
                Some(VirtualKeyCode::Key7) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 7)
                }
                Some(VirtualKeyCode::Key8) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 8)
                }
                Some(VirtualKeyCode::Key9) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_9)
                }
                //
                Some(VirtualKeyCode::Numpad0) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0)
                }
                Some(VirtualKeyCode::Numpad1) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 1)
                }
                Some(VirtualKeyCode::Numpad2) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 2)
                }
                Some(VirtualKeyCode::Numpad3) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 3)
                }
                Some(VirtualKeyCode::Numpad4) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 4)
                }
                Some(VirtualKeyCode::Numpad5) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 5)
                }
                Some(VirtualKeyCode::Numpad6) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 6)
                }
                Some(VirtualKeyCode::Numpad7) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 7)
                }
                Some(VirtualKeyCode::Numpad8) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_0 + 8)
                }
                Some(VirtualKeyCode::Numpad9) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_9)
                }
                //
                Some(VirtualKeyCode::A) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_A)
                }
                Some(VirtualKeyCode::B) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_B)
                }
                Some(VirtualKeyCode::C) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_C)
                }
                Some(VirtualKeyCode::D) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_D)
                }
                Some(VirtualKeyCode::E) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_E)
                }
                Some(VirtualKeyCode::F) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_F)
                }
                Some(VirtualKeyCode::G) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_G)
                }
                Some(VirtualKeyCode::H) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_H)
                }
                Some(VirtualKeyCode::I) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_I)
                }
                Some(VirtualKeyCode::J) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_J)
                }
                Some(VirtualKeyCode::K) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_K)
                }
                Some(VirtualKeyCode::L) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_L)
                }
                Some(VirtualKeyCode::M) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_M)
                }
                Some(VirtualKeyCode::N) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_N)
                }
                Some(VirtualKeyCode::O) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_O)
                }
                Some(VirtualKeyCode::P) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_P)
                }
                Some(VirtualKeyCode::Q) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_Q)
                }
                Some(VirtualKeyCode::R) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_R)
                }
                Some(VirtualKeyCode::S) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_S)
                }
                Some(VirtualKeyCode::T) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_T)
                }
                Some(VirtualKeyCode::U) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_U)
                }
                Some(VirtualKeyCode::V) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_V)
                }
                Some(VirtualKeyCode::W) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_W)
                }
                Some(VirtualKeyCode::X) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_X)
                }
                Some(VirtualKeyCode::Y) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_Y)
                }
                Some(VirtualKeyCode::Z) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_Z)
                }
                _ => (),
            }
        }
    }

    fn update_mouse_wheel(&mut self, delta: &MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta (_, y) => {
                if *y > 0.0 {
                    self.events.push_back(EVENT_INPUT_MOUSE_WHEEL_UP);
                } else if *y < 0.0 {
                    self.events.push_back(EVENT_INPUT_MOUSE_WHEEL_DOWN);
                }
            }
            _ => (),
        }
    }

    fn update_mouse_xy(&mut self, x: i32, y: i32) {
        self.world.mouse_x = x;
        self.world.mouse_y = y;
    }

    fn update_mouse_button(
        &mut self,
        button: &glutin::event::MouseButton,
        state: &glutin::event::ElementState,
    ) {
        //let cache = unsafe { &mut *(self.data.offset(0)) };
        if MouseButton::Left == *button && ElementState::Released == *state {
            //log(format!("mouse clicked at {}, {}", cache.mouse_x, cache.mouse_y);

            // insert message into the input FIFO
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_UP);
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_LEFT_UP);
        }
        else if MouseButton::Left == *button && ElementState::Pressed == *state {
            //log(format!("mouse clicked at {}, {}", cache.mouse_x, cache.mouse_y);

            // insert message into the input FIFO
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_DOWN);
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_LEFT_DOWN);
        }
        else if MouseButton::Right == *button && ElementState::Released == *state {
            //log(format!("mouse clicked at {}, {}", cache.mouse_x, cache.mouse_y);

            // insert message into the input FIFO
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_UP);
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_RIGHT_UP);
        }
        else if MouseButton::Right == *button && ElementState::Pressed == *state {
            //log(format!("mouse clicked at {}, {}", cache.mouse_x, cache.mouse_y);

            // insert message into the input FIFO
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_DOWN);
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_RIGHT_DOWN);
        }
    }

    fn initialize(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        self.game.initialize(&mut self.world);
        cache.initialized = true;
        let ms = std::time::Instant::now()
            .duration_since(cache.start_time)
            .as_micros() as f32
            / 1000.0;

        log(format!("Initialization Complete {:.2} ms", ms));

        const INIT_LIMIT: f32 = 1000.0;
        if ms > INIT_LIMIT {
            log(format!(
                "WARNING: blown Initilization time limit ({:} ms)",
                INIT_LIMIT
            ));
        }
    }

    pub fn ready_to_quit(&mut self) -> bool {
        let cache = unsafe { &mut *(self.data.offset(0)) };
        cache.ready_to_quit
    }

    fn render(&mut self, start_time: std::time::Instant) {
        self.gl.clear_frame();
        self.render_system.render(&self.gl, &mut self.world, start_time);

        // ok to show window?
        if !self.show_window {
            self.windowed_context.window().set_visible(true);
            self.show_window = true;
        }
    }

    fn shutdown(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        self.game.shutdown();

        if 0 < cache.blown_update_frames_significant {
            log(format!(
                "Blown Update frames: Total: {}, Sig: {} ({}%), Expected: ({}%)",
                cache.blown_update_frames,
                cache.blown_update_frames_significant,
                (cache.blown_update_frames_significant as f32 * 100.0
                    / cache.blown_update_frames as f32) as i32,
                (cache.blown_update_frames_expected as f32 * 100.0
                    / cache.blown_update_frames as f32) as i32
            ));
        }

        if 10 < cache.blown_render_frames {
            log(format!(
                "WARNING: {} blown Render frames ({}%)",
                cache.blown_render_frames,
                (cache.blown_render_frames as f32 * 100.0 / cache.count_render_frames as f32)
                    as i32
            ));
        }

        log(format!(
            "Avg Update frame loading: {}%",
            (cache.update_frame_load * 100.0 / cache.count_render_frames as f64) as i32
        ));
        log(format!(
            "Avg Render frame loading: {}%",
            (cache.render_frame_load * 100.0 / cache.count_render_frames as f64) as i32
        ));
        cache.shutdown = true;

        cache.ready_to_quit = true;

    }

    fn update(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        if !cache.running {
            cache.last_update = std::time::Instant::now();
            cache.last_render = std::time::Instant::now();
            cache.running = true;

            // pre-update for lazy loading
            self.world.mgui.update(&self.gl, 0);
            self.world.ugui.update(&self.gl, -1, -1, 0);
            self.game.update(&mut self.world, 0);
            self.physics_system.update(&mut self.world, 0);
            self.render_system.update(&self.gl, &mut self.world);
            self.easing_system.update(&mut self.world, 0);
        }

        // inner update loop
        let mut loop_counter = 0;
        const UPDATE_DT: u128 = 833; // microseconds
        loop {
            let delta = std::time::Instant::now().duration_since(cache.last_update);

            // break out of loop if stuck or finished
            loop_counter += 1;
            if UPDATE_DT > delta.as_micros() || loop_counter > 100 {
                break;
            }

            cache.last_update += std::time::Duration::from_micros(UPDATE_DT as u64);
            let timer_start = std::time::Instant::now();

            let mut expect_blown = false;

            // update game
            expect_blown |= self.game.update(&mut self.world, UPDATE_DT);

            // update systems
            if 0 == cache.count_update_frames % 1 {
                // priority 1 systems                
                expect_blown |= self.world.mgui.update(&self.gl, UPDATE_DT);
                expect_blown |= self.world.ugui.update(&self.gl, self.world.mouse_x, self.world.mouse_y, UPDATE_DT);
            }

            if 0 == cache.count_update_frames % 2 {
                // priority 2 systems
                expect_blown |= self.render_system.update(&self.gl, &mut self.world);
            }

            if 1 == cache.count_update_frames % 4 {
                // priority 3 systems
                expect_blown |= self.physics_system.update(&mut self.world, UPDATE_DT * 4);
                cache.last_physics = std::time::Instant::now();

                if let Some(val) = self.events.pop_front() {
                    expect_blown |= self.world.mgui.event(self.world.mouse_x, self.world.mouse_y, val);
                    expect_blown |= self.world.ugui.event(self.world.mouse_x, self.world.mouse_y, val);
                    expect_blown |= self.game.event(&mut self.world, val);
                }

                expect_blown |= self.easing_system.update(&mut self.world, UPDATE_DT * 4);

                /*if cfg!(debug_assertions) {
                    // artificial jitter
                    if rand::random::<f32>() < 0.01 {
                        let now = std::time::Instant::now();
                        let delta = (rand::random::<f32>() * 30.0) as u128;
                        loop {
                            if std::time::Instant::now().duration_since(now).as_millis() > delta {
                                break;
                            }
                        }
                        expect_blown = true;
                    }
                }*/
            }

            let delta = std::time::Instant::now()
                .duration_since(timer_start)
                .as_micros();
            if UPDATE_DT < delta {
                if expect_blown {
                    cache.blown_update_frames_expected += 1;
                }
                if UPDATE_DT * 3 < delta {
                    cache.blown_update_frames_significant += 1;
                }
                cache.blown_update_frames += 1;
            }
            cache.count_update_frames += 1;
            cache.update_frame_load += delta as f64 / UPDATE_DT as f64;
        }

        // outter render loop
        let delta = std::time::Instant::now().duration_since(cache.last_render);

        const RENDER_DT: u128 = 16666; // microseconds

        if RENDER_DT < delta.as_micros() {
            cache.last_render = std::time::Instant::now();

            // render frame
            self.render(cache.last_physics);

            let delta = std::time::Instant::now()
                .duration_since(cache.last_render)
                .as_micros();
            if RENDER_DT < delta {
                cache.blown_render_frames += 1;
            }
            cache.count_render_frames += 1;
            cache.render_frame_load += delta as f64 / RENDER_DT as f64;

            self.windowed_context.swap_buffers().unwrap();

            if !cache.completed_first_frame {
                cache.completed_first_frame = true;
                let ms = std::time::Instant::now()
                    .duration_since(cache.start_time)
                    .as_micros() as f32
                    / 1000.0;

                log(format!("Time to first Render frame: {:.2} ms", ms));

                const FIRST_FRAME_LIMIT: f32 = 4000.0;
                if ms > FIRST_FRAME_LIMIT {
                    log(format!(
                        "WARNING: blown time to first Render frame limit ({} ms)",
                        FIRST_FRAME_LIMIT
                    ));
                }
            }
        }
    }
}

pub fn log(output: String) {
    if cfg!(debug_assertions) {
        println!("{}", output);
    }
}

pub fn sort_enumerate<T>(data: &Vec<T>) -> Vec<(usize, T)> 
where T: PartialOrd<T> + Copy
{

    let mut temp: Vec<(usize, T)> = Vec::new();
    temp.reserve_exact(data.len());

    for i in 0..data.len() {
        temp.push((i, data[i]));
    }

    temp.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    temp
}