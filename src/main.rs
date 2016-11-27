#![feature(link_args)]

#[link_args = "-s USE_SDL=2"]
extern {}

extern crate sdl2;
extern crate libc;
extern crate emscripten;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use emscripten::em;
use std::mem::transmute;
use rand::ThreadRng;
use rand::Rng;

struct App<'a> {
    renderer: Renderer<'a>,
    event_pump: EventPump,
    rng: ThreadRng,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let rng = rand::thread_rng();
        let renderer = window.renderer().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        em::run_script(r#"alert("demo started, press any key to change the color.")"#);

        App {
            renderer: renderer,
            event_pump: event_pump,
            rng: rng,
        }
    }

    pub fn render(renderer: &mut Renderer, rng: &mut ThreadRng) {
        renderer.set_draw_color(Color::RGB(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()));
        renderer.clear();
        renderer.present();
    }
    pub fn quit() {
        em::cancel_main_loop();
        em::run_script(r#"alert("demo exited, no change will happen from now on.")"#)
    }

    pub fn poll_event(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    App::quit();
                },
                Event::KeyDown { .. } => {
                    App::render(&mut self.renderer, &mut self.rng);
                },
                _ => {}
            }
        }
    }
}

extern fn main_loop(arg: *mut libc::c_void) {
    let app = unsafe { transmute::<*mut libc::c_void, &mut App>(arg) }; 
    app.poll_event();
}

pub fn main() {
    let mut app = App::new();
    em::set_main_loop_arg(main_loop, unsafe { transmute::<&mut App, *mut libc::c_void>(&mut app) }, 60, true);
}
