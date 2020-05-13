use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("水！", [640, 480])
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
    println!("Hello, world!");
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
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(WHITE, gl);
        });
    }
}
