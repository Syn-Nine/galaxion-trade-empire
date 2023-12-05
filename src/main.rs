mod game;
mod mgfw;

const TITLE: &str = "Galaxion Trade Empire v0.1";
const XRES: i32 = game::enums::SCREEN_XRES as i32;
const YRES: i32 = game::enums::SCREEN_YRES as i32;


fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let mut core = mgfw::Core::new(TITLE, XRES, YRES, &el);

    el.run(move |event, _, control_flow| {
        core.check_events(&event);
        if core.ready_to_quit() {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
    });
}
