use std::collections::HashMap;
use std::sync::Arc;

use minifb::Key;

use crate::graphics::sprites::{draw_sprite, Sprites};
use crate::{sort_cloned_obstacles_by_y, sort_obstacles_by_y, Tile, TileType};
use crate::state::{ACCELERATION, Context, GROUND, JUMP_VELOCITY, MAX_VELOCITY, Obstacle, remove_box};
use crate::state::Direction::{Left, Right};
use crate::state::player::Player;

pub trait Command {
    fn execute(&self, context: &mut Context);
}

pub struct MoveLeft;
impl Command for MoveLeft {
    fn execute(&self, context: &mut Context) {

        let (obstacle_left, _id) = check_collision(&context.all_maps[context.current_map_index].tiles, &context.sprites, &context.player, true);

        if !obstacle_left {
            context.player.obstacle_left = false;

            // Update velocity if no collision is detected
            context.player.vx += ACCELERATION * 0.5;
            if context.player.vx > MAX_VELOCITY {
                context.player.vx = MAX_VELOCITY;
            } else {
                context.player.vx *= 0.98;
            }

            context.player.last_key = Some(Key::A);
            context.player.direction = Left;

            // Initialize a new field to track the frame count
            context.player.left_increment_frame_count += 1;

            if context.player.left_increment_frame_count >= 3 {
                context.player.left_increment_frame_count = 0; // Reset the frame count

                match context.player.left_increment {
                    7 => {
                        context.player.left_increment = 4;
                    }
                    _ => {
                        context.player.left_increment += 1;
                    }
                };
            }
        } else {
            // Stop the player from moving left if colliding
            context.player.vx = 0.0;
        }

    }
}

pub struct MoveRight;

impl Command for MoveRight {
    fn execute(&self, context: &mut Context) {


        let (obstacle_right, _id) = check_collision(&context.all_maps[context.current_map_index].tiles, &context.sprites, &context.player, false);

        if !obstacle_right {
            context.player.obstacle_right = false;

            // Update velocity if no collision is detected
            context.player.vx += ACCELERATION * 0.5;
            if context.player.vx > MAX_VELOCITY {
                context.player.vx = MAX_VELOCITY;
            } else {
                context.player.vx *= 0.98;
            }

            context.player.last_key = Some(Key::D);
            context.player.direction = Right;

            // Initialize a new field to track the frame count
            context.player.right_increment_frame_count += 1;

            if context.player.right_increment_frame_count >= 3 {
                context.player.right_increment_frame_count = 0; // Reset the frame count

                match context.player.right_increment {
                    3 => {
                        context.player.right_increment = 0;
                    }
                    _ => {
                        context.player.right_increment += 1;
                    }
                }
            }
        } else {
            // Stop the player from moving right if colliding
            context.player.vx = 0.0;
        }
    }
}

pub fn identify_grid_index_based_on_coordinates(x: f32, y: f32) -> (usize, usize) {
    let x_index = (x / 16.0) as usize;
    let y_index = (y / 16.0) as usize;

    (x_index, y_index)
}

pub fn check_collision(tiles: &Vec<Vec<Tile>>, sprites: &Sprites, player: &Player, is_left: bool) -> (bool, Option<usize>) {
    let mut collision_id: Option<usize> = None;
    let mut collision = false;

    for (y, row) in tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if tile.tile_type == TileType::Obstacle {
                if !tile.active {
                    continue;
                }

                let player_x = if is_left {
                    player.x + (sprites.player[player.left_increment].width as f32 / 3.5)
                } else {
                    player.x + (sprites.player[player.right_increment].width as f32 / 1.5)
                };

                if player_x > tile.x_left && player_x < tile.x_right && player.y > tile.y_bottom {
                    collision_id = Some(x + y);
                    collision = true;
                    break;
                }
            }
        }
        if collision {
            break;
        }
    }


    // print!("Collision detected: {}", collision_id.unwrap_or(0));
    // TODO: May need to identify exactly the y and x coordinates of the obstacle

    (collision, collision_id)
}

pub struct Jump;

impl Command for Jump {
    fn execute(&self, context: &mut Context) {

        if !context.player.is_jumping && (context.player.on_ground || context.player.on_obstacle) {
            context.player.vy = JUMP_VELOCITY;
            context.player.on_ground = false;
            context.player.on_obstacle = false;
            context.player.is_jumping = true;
            context.player.last_key = Some(Key::Space);
        }
    }
}

pub struct Kick;

impl Command for Kick {
    fn execute(&self, context: &mut Context) {
        context.player.is_kicking = true;
        context.player.kick_frame = 0;
        context.player.kick_frame_timer = 0;

        // let sorted_obstacles = sort_cloned_obstacles_by_y(context.all_maps[context.current_map_index].obstacles.clone());
        // let obstacles = &mut context.all_maps[context.current_map_index].obstacles;
        // let tiles = &context.all_maps[context.current_map_index].tiles;
        //
        //
        //
        // let (collision, id) = check_tile_intersection(&context.all_maps[context.current_map_index].tiles, &context.player);
        //
        //
        // //check_collision(&obstacles, &context.sprites, &context.player, context.player.direction == Left);
        //
        // // Check if the player is adjacent to an obstacle to the right
        // if collision {
        //     let collided_obstacle =  context.all_maps[context.current_map_index].obstacles[id.unwrap()].clone();
        //     println!("Obstacle detected with id: {}, x.left {}, x.right {}, y.bottom {} and y.top {}",
        //              id.unwrap(), collided_obstacle.x_left, collided_obstacle.x_right, collided_obstacle.y_bottom, collided_obstacle.y_top);
        //
        //     // println!("Player is adjacent to an obstacle with id {} to the right.", id.unwrap());
        //     if context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability > 0 {
        //         // println!("Obstacle durability: {}", context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability);
        //         context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability -= 1;
        //         println!("Obstacle durability: {}", context.all_maps[context.current_map_index].obstacles[id.unwrap()].durability);
        //     } else {
        //         // println!("Obstacle durability: 0");
        //         remove_box(context, id.unwrap());
        //     }
        //
        // }
    }
}

pub type CommandMap = HashMap<Key, Arc<dyn Command>>;

pub fn initialize_command_map() -> CommandMap {
    let mut commands: CommandMap = HashMap::new();

    commands.insert(Key::A, Arc::new(MoveLeft));
    commands.insert(Key::D, Arc::new(MoveRight));
    commands.insert(Key::Space, Arc::new(Jump));
    commands.insert(Key::X, Arc::new(Kick));

    commands
}

