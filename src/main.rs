use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::line;
use std::f64::consts::PI;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const SPLIT_NUM: usize = 100;

const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;
const AMPLITUDE: f64 = 100.0;
const FREQUENCY: f64 = 2.0;

const V_CENTER: f64 = HEIGHT / 2.0;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("水！", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}

struct App {
    gl: GlGraphics,
}


impl App {
    fn new(opengl: shader_version::opengl::OpenGL) -> App {
        App {
            gl: GlGraphics::new(opengl),
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(WHITE, gl);
            for i in 0..SPLIT_NUM {
                line(BLACK, 1.0, water_wave(i), c.transform, gl);
            }
        });
    }
}

fn water_wave(i: usize) -> [f64; 4] {
    let x1 = (i as f64) / (SPLIT_NUM as f64) * WIDTH;
    let x2 = (i as f64 + 1.0) / (SPLIT_NUM as f64) * WIDTH;
    [x1, V_CENTER + wave_func(i), x2, V_CENTER + wave_func(i + 1)]
}

fn wave_func(i: usize) -> f64 {
    ((i as f64) * PI * 2.0 / (SPLIT_NUM as f64) * FREQUENCY).sin() * AMPLITUDE
}
