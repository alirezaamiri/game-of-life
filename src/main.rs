extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::Rng;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    cells: Vec<Vec<bool>>,
    scale: usize,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // self.cells = vec![vec![false; self.scale]; self.scale];
        let cells = &self.cells;
        let cell_size = args.window_size[0] / self.scale as f64;
        // self.cell_size = cell_size;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
            let transform = c.transform.trans(0.0, 0.0);
            // Draw a box rotating around the middle of the screen.
            for x in 0..cells.len() {
                for y in 0..cells[x].len() {
                    if cells[x][y] {
                        let dims = rectangle::square(
                            x as f64 * cell_size,
                            y as f64 * cell_size,
                            cell_size,
                        );
                        rectangle(RED, dims, transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut rng = rand::thread_rng();

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        // Draw a box rotating around the middle of the screen.
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                let n1: u8 = rng.gen_range(0, 10);
                if n1 < 5 {
                    self.cells[x][y] = true;
                } else {
                    self.cells[x][y] = false;
                }
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Game of Life", [600, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let scale = 100;
    let cells = vec![vec![false; scale]; scale];
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        cells: cells,
        scale: scale,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
