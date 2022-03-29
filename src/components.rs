use bevy::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Component)]
pub struct Block {
    pub value: u32,
}

#[derive(Component)]
pub struct BlockText;

#[derive(Component)]
pub struct Board {
    pub size: u8,
}

#[derive(Default, Component)]
pub struct Game {
    pub score: u32,
    pub score_best: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum RunState {
    Playing,
    GameOver,
}

#[derive(Component)]
pub struct ScoreDisplay;

#[derive(Component)]
pub struct BestScoreDisplay;
