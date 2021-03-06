extern crate env_logger;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_text;

use gfx::traits::{Stream, Device};
use gfx_window_glutin as gfxw;
use glutin::{WindowBuilder, Event, VirtualKeyCode, GL_CORE};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BROWN: [f32; 4] = [0.65, 0.16, 0.16, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const FONT_PATH: &'static str = "examples/assets/Ubuntu-R.ttf";

fn main() {
    env_logger::init().unwrap();

    let (mut stream, mut device, _) = {
        let window = WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_title(format!("gfx_text example"))
            .with_gl(GL_CORE)
            .build()
            .unwrap();
        gfxw::init(window)
    };

    let mut normal_text = gfx_text::new(device.spawn_factory())
        .unwrap();
    let mut big_text = gfx_text::new(device.spawn_factory())
        .with_size(20)
        .unwrap();
    let mut custom_font_text = gfx_text::new(device.spawn_factory())
        .with_size(25)
        .with_font(FONT_PATH)
        .unwrap();

    'main: loop {
        for event in stream.out.window.poll_events() {
            match event {
                Event::Closed => break 'main,
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => break 'main,
                _ => {},
            }
        }
        stream.clear(gfx::ClearData {color: WHITE, depth: 1.0, stencil: 0});

        normal_text.draw("The quick brown fox jumps over the lazy dog", [10, 10], BROWN);
        normal_text.draw("The quick red fox jumps over the lazy dog", [30, 30], RED);
        normal_text.draw_end(&mut stream).unwrap();

        big_text.draw("The big brown fox jumps over the lazy dog", [50, 50], BROWN);
        big_text.draw_end(&mut stream).unwrap();

        custom_font_text.draw("The custom blue fox jumps over the lazy dog", [10, 80], BLUE);
        custom_font_text.draw_end(&mut stream).unwrap();

        //stream.present(&mut device); ICE!
        stream.flush(&mut device);
        stream.out.window.swap_buffers();
        device.cleanup();
    }
}
