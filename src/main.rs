mod player;
mod objects;
mod game;

use macroquad::prelude::*;
use crate::objects::{PhysicsObject, CollisionBox};

#[macroquad::main("fps-engine")]
async fn main() {

    let mut state = game::State::new();
    let mut player = player::Player::new();

    state.objects.push(
        Box::new(
            CollisionBox::new(
                vec3(0., -0.5, 0.),
                vec3(10., 0.5, 10.)
            )
        )
    );
    state.objects.push(
        Box::new(
            CollisionBox::new(
                vec3(5., 1., 5.),
                vec3(2., 1., 2.)
            )
        )
    );

    set_cursor_grab(true);
    show_mouse(false);


    loop {
        clear_background(BLACK);

        player.update(&mut state);

        draw_grid_ex(200, 0.1, GRAY, DARKGRAY, vec3(0., 0.01, 0.), quat(1., 0., 0., 0.));

        for obj in state.objects.iter() {
            obj.draw();        
        }
        set_default_camera();

        //crosshair
        draw_line(
            0.95 * screen_width() / 2.,
            screen_height() / 2.,
            1.05 * screen_width() /2.,
            screen_height() / 2.,
            3.,
            GREEN
        );
        draw_line(
            screen_width() / 2.,
            0.95 * screen_height() / 2.,
            screen_width() /2.,
            1.05 * screen_height() / 2.,
            3.,
            GREEN
        );
        next_frame().await
    }

}
