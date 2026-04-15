use doom_engine::{engine::Engine, *};
use raylib::prelude::*;

fn main() {
    // create main window, scaled to correct size
    let (mut rl, thread) = raylib::init()
        .size(WIDTH * SCALE, HEIGHT * SCALE)
        .title("")
        .build();

    // locks fps to 20 to make things look more "retro"
    rl.set_target_fps(20);

    // create framebuffer at intended resolution
    let mut texture = rl
        .load_render_texture(&thread, WIDTH as _, HEIGHT as _)
        .unwrap();

    // init function
    let mut engine = Engine::new();
    engine.read_world_from_file("world.txt");

    while !rl.window_should_close() {
        // move player
        engine.p.handle_inputs(&rl);

        // draw everything to lower resolution texture
        let mut dh = rl.begin_drawing(&thread);
        dh.draw_texture_mode(&thread, &mut texture, |mut d| {
            engine.draw(&mut d);
        });

        // draws scaled texture to main window
        dh.clear_background(Color::RED); // shows red if something went wrong
        dh.draw_texture_ex(&texture, Vector2::zero(), 0.0, SCALE as _, Color::WHITE);
    }
}
