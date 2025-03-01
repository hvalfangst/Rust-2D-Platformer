use minifb::Key;

use crate::graphics::sprites::draw_sprite;
use crate::state::Direction::{Left, Right};
use crate::state::*;
use crate::TileType;

pub fn update(context: &mut Context) {
    draw_game_world(context);
    draw_player(context)
}

fn draw_player(context: &mut Context) {

    // Determine the current direction and action of the player
    let direction = context.player.direction;
    context.player.last_key.unwrap_or(Key::D);

    // Determine the sprite to draw
    let sprite_to_draw =



    if context.player.is_kicking {
        context.player.kick_frame_timer += 1;
        if context.player.kick_frame_timer >= KICK_FRAME_DURATION as usize {
            context.player.kick_frame += 1;
            context.player.kick_frame_timer = 0;

            if context.player.kick_frame >= 2 {
                context.player.is_kicking = false;
                context.player.kick_frame = 0;
            }
        }

        // Select the correct kick frame based on direction
        if direction == Right {
            &context.sprites.kick[context.player.kick_frame]
        } else {
            &context.sprites.kick[2 + context.player.kick_frame]
        }
    }
    else if context.player.almost_ground && !context.player.on_obstacle && direction == Right {
        &context.sprites.jump[1]
    } else if context.player.almost_ground && !context.player.on_obstacle && direction == Left {
        &context.sprites.jump[4]
    } else if !context.player.on_ground && !context.player.on_obstacle && direction == Right {
        &context.sprites.jump[2]
    } else if !context.player.on_ground && !context.player.on_obstacle && direction == Left {
        &context.sprites.jump[5]
    } else if direction == Right {
        &context.sprites.player[context.player.right_increment]
    } else if direction == Left {
        &context.sprites.player[context.player.left_increment]
    } else { // Default is moving to the right
        &context.sprites.player[context.player.right_increment]
    };


    // Draw the chosen player sprite
    draw_sprite(
        context.player.x as usize,
        context.player.y as usize - (sprite_to_draw.height - 3) as usize,
        sprite_to_draw,
        context.window_buffer,
        context.all_maps[context.current_map_index].width
    );

    // Draw different sizes of shadows based on player state
    let shadow_sprite = if context.player.on_ground {
            &context.sprites.shadow[0]
    } else if context.player.almost_ground {
            &context.sprites.shadow[2]
    } else { // Player is in the air
            &context.sprites.shadow[1]
    };

    // Draw associated shadow if not on or above obstacle
    if !context.player.on_obstacle && !context.player.above_obstacle {
        draw_sprite(
            context.player.x as usize,
            context.player.y as usize + 3,
            shadow_sprite,
            context.window_buffer,
            context.all_maps[context.current_map_index].width
        );

    }
}

fn draw_game_world(context: &mut Context) {

    // First draw the blue background
    draw_sprite(0, 0, &context.sprites.blue_background[0], context.window_buffer, context.all_maps[context.current_map_index].width);

    // Then draw the grass, which alternates between two sprites to emulate wind
    draw_sprite(0, context.all_maps[context.current_map_index].height - context.sprites.grass[0].height as usize, &context.sprites.grass[context.grass_sprite_index], context.window_buffer, context.all_maps[context.current_map_index].width);

    // Then draw the sky, which alternates between four sprites to emulate clouds
    draw_sprite(0, 0, &context.sprites.sky[context.sky_sprite_index], context.window_buffer, context.all_maps[context.current_map_index].width);

    for (row_index, row) in context.all_maps[context.current_map_index].tiles.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            if tile.tile_type == TileType::Obstacle {
                // println!("Drawing obstacle with id {:?} at x.left: {}, y.bottom: {}", tile.id, tile.x_left, tile.y_bottom);
                if !tile.active {
                    continue;
                }
                    // println!("Drawing obstacle with id {:?} at x.left: {}, y.bottom: {}", tile.id, tile.x_left, tile.y_bottom);
                    let metal_box_sprite =
                        if tile.durability == 2 {
                            &context.sprites.metal_box[0] // undamaged
                        } else if tile.durability == 1 {
                            &context.sprites.metal_box[1] // slightly damaged
                        } else {
                            &context.sprites.metal_box[2] // damaged
                        };

                    draw_sprite(tile.x_left as usize, tile.y_bottom as usize, metal_box_sprite, context.window_buffer, context.all_maps[context.current_map_index].width);
                }
            }
        };


    if context.player.game_over {
        draw_sprite(0, 0, &context.sprites.game_over[context.game_over_index], context.window_buffer, context.all_maps[context.current_map_index].width);
    }
}