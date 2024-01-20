use crate::network::packet;
use crate::network::{self, player};
use rand;
use std::io::Error;
use std::thread;
use std::time;
use std::vec;

//////////////////////////////////////////////
///
///
/// Modules
///
///
//////////////////////////////////////////////
mod ball;
mod game;
mod racket;

//////////////////////////////////////////////
///
///
/// Constants
///
///
//////////////////////////////////////////////

const BALL_SIZE: usize = 20;


//////////////////////////////////////////////
///
///
/// Entry point/Game loop thread
///
///
//////////////////////////////////////////////

pub fn ping_loop(players: &mut [network::player::Player]) -> Result<(), Error> {
    let map = game::gen_map(players);

    let mut width: f32 = 0.;
    let mut height: f32 = 0.;
    for p in players.iter() {
        width += p.physical_width;
        height = height.max(p.physical_height);
    }

    let mut bullets = vec::Vec::new();

    let mut internal_timer = time::Instant::now();

    let mut powerups = vec::Vec::new();
    let mut last_modifier_gen = time::Instant::now();

    for p in players.iter() {
        for _ in 0..3 {
            let x: f32 = ((rand::random::<f32>() * p.physical_width) as usize
                + maze::WALL_LENGTH / 2
                - ((rand::random::<f32>() * p.physical_width) as usize % maze::WALL_LENGTH))
                as f32;
            let y: f32 = ((rand::random::<f32>() * p.physical_height) as usize
                + maze::WALL_LENGTH / 2
                - ((rand::random::<f32>() * p.physical_height) as usize % maze::WALL_LENGTH))
                as f32;
            let powerup = (rand::random::<f32>() * powerup::POWERUP_COUNT as f32) as usize;

            powerups.push(powerup::PowerUp::new(
                powerup.into(),
                Vector2 {
                    x: x + p.top_left_x,
                    y: y + p.top_left_y,
                },
            ));
        }
    }

    let mut sprites = sprite::Sprite::create_sprites(players);
    for p in players.iter_mut() {
        let packed_maze = maze::pack_maze(p, &maze);
        p.send(&packed_maze)?;
        let data = sprite::Sprite::pack_sprites(&sprites);
        p.send(&data)?;
    }

    loop {
        for s in sprites.iter_mut() {
            s.update_sprite_status(&maze, &mut bullets, internal_timer);
        }

        update_bullet_status(&mut bullets, &maze, internal_timer);

        for s in sprites.iter_mut() {
            s.update_dead_status(&mut bullets);
            s.update_powerup_status(&mut powerups);
        }

        if last_modifier_gen.elapsed().as_secs() > 5 {
            last_modifier_gen = time::Instant::now();
            generate_new_modifiers(&mut powerups, width, height);
        }

        internal_timer = time::Instant::now();

        for p in players.iter_mut() {
            send_game_data(p, &bullets, &powerups, &sprites)?;
            recv_game_data(p, &mut sprites);
        }
        thread::sleep(time::Duration::from_millis(10));
    }
}