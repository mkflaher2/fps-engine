mod game;
mod materials;
mod objects;
mod player;

use std::f32::consts::PI;
use macroquad::prelude::*;
use crate::objects::{PhysicsObject, CollisionBox, CollisionRamp, Skybox};

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
    state.objects.push(
        Box::new(
            CollisionRamp::new(
                vec3(0., 1., -5.),
                0.,
                vec3(1., 1., 1.,)
            )
        )
    );
    state.objects.push(
        Box::new(
            CollisionRamp::new(
                vec3(1., 1., 5.),
                PI/2.,
                vec3(1., 1., 2.,)
            )
        )
    );

    set_cursor_grab(true);
    show_mouse(false);

    let skybox_texture = load_texture("textures/skybox3.png").await.unwrap();
    let skybox = Skybox::new(100., skybox_texture.clone());

    loop {
        clear_background(BLACK);
        set_default_camera();
        //draw_texture(&skybox_texture, 0., 0., WHITE);


        player.update(&mut state);

        draw_grid_ex(20, 1., GRAY, DARKGRAY, vec3(0., 0.01, 0.), quat(1., 0., 0., 0.));

        for obj in state.objects.iter() {
            obj.draw();        
        }

        //let material = materials::default_world();
        //gl_use_material(&material);
        gl_use_default_material();
        skybox.draw();
        
        //crosshair
        gl_use_default_material();
        set_default_camera();
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
