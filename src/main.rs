mod renderer;
//use renderer::{Renderer, RenderOptions};
use renderer::Renderer;
use winit::event_loop::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new();

    /*
    let mut renderer = Renderer::init(
        RenderOptions {
            title: "Snake Game",
            ..RenderOptions::default()
        },
    );
    */

    let mut renderer = Renderer::init()
        .setup(&mut event_loop);

    loop {
        let exit = renderer.handle_events(&mut event_loop);
        renderer.run();

        if exit == true {
            break;
        }
    }
}
