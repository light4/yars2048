use bevy::prelude::*;
use bevy_easings::*;
use itertools::Itertools;
use rand::prelude::*;

use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Range;

mod components;
mod ui;

use components::*;
use ui::*;

const TILE_SPACER: f32 = 10.0;
const TILE_SIZE: f32 = 60.0;

pub struct NewTileEvent;

struct Palette {
    board: Color,
    tile_placeholder: Color,
}

#[bevy_main]
pub fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Yars 2048".to_string(),
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.1)))
        .init_resource::<Game>()
        .add_startup_system(setup)
        // .add_startup_system(setup_ui)
        .add_plugins(DefaultPlugins)
        .add_plugin(GameUiPlugin)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_stage("board_setup", SystemStage::single(spawn_board))
        .add_state(RunState::Playing)
        .add_system_set(
            SystemSet::on_update(RunState::Playing)
                .with_system(board_shift)
                .with_system(render_blocks)
                .with_system(render_block_color)
                .with_system(new_tile_handler),
        )
        // setup when entering the state
        .add_system_set(
            SystemSet::on_enter(RunState::Playing)
                .with_system(game_reset)
                .with_system(spawn_tiles.after(game_reset)),
        )
        .add_event::<NewTileEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(Palette {
        board: Color::rgb_u8(187, 173, 160),
        tile_placeholder: Color::rgb_u8(214, 205, 196),
    });
}

fn spawn_board(mut commands: Commands, palette: Res<Palette>) {
    let board = Board { size: 4 };
    let physical_board_size = {
        // size of all tiles
        f32::from(board.size) * TILE_SIZE
        // size of all spacers
        + f32::from(board.size) * TILE_SPACER
        // extra spacer on the off side to round out the board
        + TILE_SPACER
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(physical_board_size, physical_board_size)),
                color: palette.board,
                ..default()
            },
            ..default()
        })
        .with_children(|child_builder| {
            for tile in (0..board.size).cartesian_product(0..board.size) {
                let position = Position {
                    x: tile.0,
                    y: tile.1,
                };
                child_builder
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            color: palette.tile_placeholder,
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            // true position
                            f32::from(tile.0) * TILE_SIZE
                        // moved left because it is at board center
                            - (f32::from(board.size)
                                * TILE_SIZE
                                / 2.0)
                                // moved right because it's even numbered
                                // (odd would be centered)
                            + (0.5 * TILE_SIZE)
                            // account for in-between spacing by applying N
                            // spacers
                            + f32::from(tile.0)
                                * TILE_SPACER
                                - TILE_SPACER * 1.5,
                            f32::from(tile.1) * TILE_SIZE
                                - (f32::from(board.size) * TILE_SIZE / 2.0)
                                + (0.5 * TILE_SIZE)
                                + f32::from(tile.1) * TILE_SPACER
                                - TILE_SPACER * 1.5,
                            1.0,
                        ),
                        ..default()
                    })
                    .insert(position)
                    .insert(EmptyBlock);
            }
        })
        .insert(board);
}

fn game_reset(mut commands: Commands, blocks: Query<Entity, With<Block>>, mut game: ResMut<Game>) {
    for entity in blocks.iter() {
        commands.entity(entity).despawn_recursive();
    }
    game.score = 0;
}

fn spawn_tiles(mut commands: Commands, query_board: Query<&Board>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let board = query_board.single();
    // insert new tile
    let mut rng = rand::thread_rng();
    let starting_tiles: Vec<Position> = (0..board.size)
        .cartesian_product(0..board.size)
        .map(|tile_pos| Position {
            x: tile_pos.0,
            y: tile_pos.1,
        })
        .choose_multiple(&mut rng, 2);
    for Position { x, y } in starting_tiles.iter() {
        let pos = Position { x: *x, y: *y };
        let new_block = Block::default();
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    color: new_block.color(),
                    ..default()
                },
                transform: Transform::from_xyz(
                    block_pos_to_transform(board.size, pos.x),
                    block_pos_to_transform(board.size, pos.y),
                    1.0,
                ),
                ..default()
            })
            .with_children(|child_builder| {
                child_builder
                    .spawn_bundle(Text2dBundle {
                        text: Text::with_section(
                            new_block.score().to_string(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::BLACK,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 0.0, 1.0),
                        ..default()
                    })
                    .insert(BlockText);
            })
            .insert(new_block)
            .insert(pos);
    }
}

fn block_pos_to_transform(board_size: u8, pos: u8) -> f32 {
    f32::from(pos) * TILE_SIZE
        // moved left because it is at board center
            - (f32::from(board_size)
                * TILE_SIZE
                / 2.0)
                // moved right because it's even numbered
                // (odd would be centered)
            + (0.5 * TILE_SIZE)
            // account for in-between spacing by applying N
            // spacers
            + f32::from(pos)
                * TILE_SPACER
        - TILE_SPACER * 1.5
}

fn render_blocks(
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut Transform, &Position, Changed<Position>), With<Block>>,
    query_board: Query<&Board>,
) {
    let board = query_board.single();
    for (entity, transform, pos, pos_changed) in blocks.iter_mut() {
        if pos_changed {
            let x = block_pos_to_transform(board.size, pos.x);
            let y = block_pos_to_transform(board.size, pos.y);
            let mut ent = commands.entity(entity);
            ent.insert(transform.ease_to(
                Transform::from_xyz(x, y, transform.translation.z),
                EaseFunction::QuadraticInOut,
                EasingType::Once {
                    duration: std::time::Duration::from_millis(100),
                },
            ));
        }
    }
}

fn render_block_color(mut block_sprite: Query<(&mut Sprite, &Block), With<Block>>) {
    for (mut sp, block) in block_sprite.iter_mut() {
        sp.color = block.color();
    }
}

#[derive(Debug)]
enum MergeStatus {
    Merge,
    DifferentRows,
    DifferentValues,
}
// u32 is the block value
// u8 is the "vertical" y position,
// which is different for each direction
fn should_merge(block: (u32, u8), block_next: (u32, u8)) -> MergeStatus {
    if block.1 != block_next.1 {
        // if the blocks aren't on the same vertical level
        // they can't collide, so skip to next loop
        // iteration
        MergeStatus::DifferentRows
    } else if block.0 != block_next.0 {
        // if the block values don't match,
        // they can't merge, so skip to the
        // next loop iteration
        MergeStatus::DifferentValues
    } else {
        MergeStatus::Merge
    }
}

fn board_shift(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    touches: Res<Touches>,
    // mut query_world: Query<&mut World>,
    mut texts: Query<(Entity, &mut Text), With<BlockText>>,
    mut blocks: Query<(Entity, &mut Position, &mut Block, &Children)>,
    query_board: Query<&Board>,
    mut tile_writer: EventWriter<NewTileEvent>,
    mut game: ResMut<Game>,
    mut run_state: ResMut<State<RunState>>,
) {
    // Normal Processing
    let board = query_board.single();

    // EndGameCheck
    if blocks.iter_mut().len() == 16 {
        let mut map: HashMap<(u8, u8), u32> = HashMap::new();
        for tile in (0..board.size).cartesian_product(0..board.size) {
            map.insert(tile, 0);
        }
        for (_, position, block, _) in blocks.iter_mut() {
            map.insert((position.x, position.y), block.level);
        }
        let has_move = map.iter().any(|((x, y), value)| {
            vec![(-1, 0), (0, 1), (1, 0), (0, -1)]
                .iter()
                .flat_map(|(x2, y2)| {
                    let new_x = (*x as i8) - x2;
                    let new_y = (*y as i8) - y2;

                    let board_range: Range<i8> = 0..(board.size as i8);

                    if !board_range.contains(&new_x) && !board_range.contains(&new_y) {
                        return None;
                    };

                    match (new_x.try_into(), new_y.try_into()) {
                        (Ok(x), Ok(y)) => Some(map.get(&(x, y))),
                        _ => None,
                    }
                })
                .flatten()
                .any(|v| v == value)
        });

        if !has_move {
            run_state.set(RunState::GameOver).unwrap();
        }
    };

    let touch_event = touches.iter_just_released().next();

    let move_left = {
        keyboard_input.just_pressed(KeyCode::Left)
            || touch_event.map_or(false, |t| {
                let a = t.distance().to_array();
                a[0].is_sign_negative() && a[0].abs() > a[1].abs()
            })
    };

    let move_right = {
        keyboard_input.just_pressed(KeyCode::Right)
            || touch_event.map_or(false, |t| {
                let a = t.distance().to_array();
                a[0].is_sign_positive() && a[0].abs() > a[1].abs()
            })
    };

    let move_down = {
        keyboard_input.just_pressed(KeyCode::Down)
            || touch_event.map_or(false, |t| {
                let a = t.distance().to_array();
                a[1].is_sign_negative() && a[1].abs() > a[0].abs()
            })
    };

    let move_up = {
        keyboard_input.just_pressed(KeyCode::Up)
            || touch_event.map_or(false, |t| {
                let a = t.distance().to_array();
                a[1].is_sign_positive() && a[1].abs() > a[0].abs()
            })
    };

    if move_left {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| match Ord::cmp(&a.1.y, &b.1.y) {
                std::cmp::Ordering::Equal => Ord::cmp(&a.1.x, &b.1.x),
                ordering => ordering,
            })
            // .sorted_by_key(|v| v.1.y)
            .peekable();
        let mut x: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.x = x;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.level, block.1.y),
                        (block_next.2.level, block_next.1.y),
                    ) {
                        MergeStatus::Merge => {
                            // despawn the next block, and
                            // merge it with the current
                            // block.
                            let real_next_block = it
                                .next()
                                .expect("A peeked block should always exist when we .next here");
                            block.2.level += 1;
                            block.1.x = x;

                            // update score
                            game.score += block.2.score();
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts.get_mut(*child).expect("text to exist");
                                let mut section = text
                                    .1
                                    .sections
                                    .first_mut()
                                    .expect("expect a single section in text");
                                section.value = block.2.score().to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek() {
                                if block.1.y != future.1.y {
                                    x = 0;
                                } else {
                                    x += 1;
                                }
                            }

                            commands.entity(real_next_block.0).despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.x = x;
                            x = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.x = x;
                            x += 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if move_right {
        let mut it = blocks
            .iter_mut()
            // we want our sorting to first sort by x,
            // then break x ties with y's comparison
            .sorted_by(|a, b| match Ord::cmp(&b.1.y, &a.1.y) {
                std::cmp::Ordering::Equal => Ord::cmp(&b.1.x, &a.1.x),
                a => a,
            })
            .peekable();
        let mut x: u8 = 0;
        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    break;
                }
                (Some(mut block), None) => {
                    block.1.x = board.size - 1 - x;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.level, block.1.y),
                        (block_next.2.level, block_next.1.y),
                    ) {
                        MergeStatus::Merge => {
                            let real_next_block = it
                                .next()
                                .expect("A peeked block should always exist when we .next here");
                            block.2.level += 1;
                            block.1.x = board.size - 1 - x;

                            // update score
                            game.score += block.2.score();
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts.get_mut(*child).expect("text to exist");
                                let mut section = text
                                    .1
                                    .sections
                                    .first_mut()
                                    .expect("expect a single section in text");
                                section.value = block.2.score().to_string();
                            }

                            if let Some(future) = it.peek() {
                                if block.1.y != future.1.y {
                                    x = 0;
                                } else {
                                    x += 1;
                                }
                            }
                            commands.entity(real_next_block.0).despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.x = board.size - 1 - x;
                            x = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.x = board.size - 1 - x;
                            x += 1;
                            continue;
                        }
                    }
                }
            }

            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if move_down {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| match Ord::cmp(&a.1.x, &b.1.x) {
                std::cmp::Ordering::Equal => Ord::cmp(&a.1.y, &b.1.y),
                ordering => ordering,
            })
            .peekable();
        let mut y: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.y = y;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.level, block.1.x),
                        (block_next.2.level, block_next.1.x),
                    ) {
                        MergeStatus::Merge => {
                            // _recursive the next block,
                            // and
                            // merge it with the current
                            // block.
                            let real_next_block = it
                                .next()
                                .expect("A peeked block should always exist when we .next here");
                            block.2.level += 1;
                            block.1.y = y;

                            // update score
                            game.score += block.2.score();
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts.get_mut(*child).expect("text to exist");
                                let mut section = text
                                    .1
                                    .sections
                                    .first_mut()
                                    .expect("expect a single section in text");
                                section.value = block.2.score().to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek() {
                                if block.1.x != future.1.x {
                                    y = 0;
                                } else {
                                    y += 1;
                                }
                            }
                            commands.entity(real_next_block.0).despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.y = y;
                            y = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.y = y;
                            y += 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    } else if move_up {
        let mut it = blocks
            .iter_mut()
            .sorted_by(|a, b| match Ord::cmp(&b.1.x, &a.1.x) {
                std::cmp::Ordering::Equal => Ord::cmp(&b.1.y, &a.1.y),
                ordering => ordering,
            })
            .peekable();
        let mut y: u8 = 0;

        loop {
            match (it.next(), it.peek()) {
                (None, _) => {
                    // nothing left to process
                    break;
                }
                (Some(mut block), None) => {
                    block.1.y = board.size - 1 - y;
                }
                (Some(mut block), Some(block_next)) => {
                    match should_merge(
                        (block.2.level, block.1.x),
                        (block_next.2.level, block_next.1.x),
                    ) {
                        MergeStatus::Merge => {
                            // despawn the next block, and
                            // merge it with the current
                            // block.
                            let real_next_block = it
                                .next()
                                .expect("A peeked block should always exist when we .next here");
                            block.2.level += 1;
                            block.1.y = board.size - 1 - y;

                            // update score
                            game.score += block.2.score();
                            // update text
                            for child in block.3.iter() {
                                let mut text = texts.get_mut(*child).expect("text to exist");
                                let mut section = text
                                    .1
                                    .sections
                                    .first_mut()
                                    .expect("expect a single section in text");
                                section.value = block.2.score().to_string();
                            }
                            // if the next, next block
                            // (block #3 of 3)
                            // isn't in the same row, reset
                            // x
                            // otherwise increment by one
                            if let Some(future) = it.peek() {
                                if block.1.x != future.1.x {
                                    y = 0;
                                } else {
                                    y += 1;
                                }
                            }
                            commands.entity(real_next_block.0).despawn_recursive();
                            continue;
                        }
                        MergeStatus::DifferentRows => {
                            block.1.y = board.size - 1 - y;
                            y = 0;
                            continue;
                        }
                        MergeStatus::DifferentValues => {
                            block.1.y = board.size - 1 - y;
                            y += 1;
                            continue;
                        }
                    }
                }
            }
            break;
        }
        // insert new block
        tile_writer.send(NewTileEvent);
    }
    if game.score_best < game.score {
        game.score_best = game.score;
    }
}

fn new_tile_handler(
    mut tile_reader: EventReader<NewTileEvent>,
    mut commands: Commands,
    query_board: Query<&Board>,
    asset_server: Res<AssetServer>,
    blocks: Query<(&Position, &Block)>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let board = query_board.single();

    if tile_reader.iter().next().is_some() {
        // insert new tile
        let mut rng = rand::thread_rng();
        let possible_position = (0..board.size)
            .cartesian_product(0..board.size)
            .filter(|tile_pos| {
                !blocks
                    .iter()
                    .any(|(pos, _)| pos.x == tile_pos.0 && pos.y == tile_pos.1)
            })
            .choose(&mut rng);

        if let Some((x, y)) = possible_position {
            let pos = Position { x, y };
            let new_block = Block::default();
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        color: new_block.color(),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        block_pos_to_transform(board.size, pos.x),
                        block_pos_to_transform(board.size, pos.y),
                        1.0,
                    ),
                    ..default()
                })
                .with_children(|child_builder| {
                    child_builder
                        .spawn_bundle(Text2dBundle {
                            text: Text::with_section(
                                new_block.score().to_string(),
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::BLACK,
                                },
                                TextAlignment {
                                    vertical: VerticalAlign::Center,
                                    horizontal: HorizontalAlign::Center,
                                },
                            ),
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        })
                        .insert(BlockText);
                })
                .insert(new_block)
                .insert(pos);
        };
    };
}
