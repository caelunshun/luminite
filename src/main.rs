extern crate glm;
extern crate winit;
#[macro_use]
extern crate log;
extern crate simple_logger;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(all(feature = "vulkan", not(any(feature = "dx12", feature = "metal"))))]
extern crate gfx_backend_vulkan as back;

extern crate gfx_hal;

pub use glm::*;
use std::cell::RefCell;
use std::rc::Rc;
use winit::{Event, WindowEvent};

pub mod render;

pub struct Game {
    pub render: render::context::RenderContext<back::Backend>,
    pub running: bool,
}

fn main() {
    simple_logger::init().unwrap();
    let mut game = Game {
        render: render::create_context(),
        running: true,
    };

    main_loop(&mut game);
    render::destroy(game.render);
}

fn main_loop(game: &mut Game) {
    while game.running {
        poll_events(game);
        render::render(&mut game.render);
    }
}

fn poll_events(game: &mut Game) {
    // Yes, yes, very unsafe. There was a weird issue
    // with variables not being mutated, so I'm just
    // going to use raw pointers.
    unsafe {
        let running_ptr = &mut true as *mut bool;
        let events_loop = &mut game.render.events_loop;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *running_ptr = false,
                _ => (),
            },
            _ => (),
        });
        game.running = *running_ptr;
    }
}
