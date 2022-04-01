use bevy::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Component)]
pub struct Block {
    pub level: u32,
}

impl Block {
    /// Each level has a unique color (up to 9).
    /// Returns the color for a given tile.
    /// from https://github.com/tpcstld/2048/tree/master/2048/base/src/main/res/drawable-mdpi
    pub fn color(&self) -> Color {
        match self.level {
            1 => Color::rgb_u8(238, 228, 218),
            2 => Color::rgb_u8(237, 224, 200),
            3 => Color::rgb_u8(242, 177, 121),
            4 => Color::rgb_u8(245, 149, 99),
            5 => Color::rgb_u8(246, 124, 95),
            6 => Color::rgb_u8(246, 94, 59),
            7 => Color::rgb_u8(237, 207, 114),
            8 => Color::rgb_u8(237, 204, 97),
            9 => Color::rgb_u8(237, 200, 80),
            10 => Color::rgb_u8(237, 197, 63),
            11 => Color::rgb_u8(237, 194, 46),
            12 => Color::rgb_u8(60, 58, 50),
            _ => Color::BLACK,
        }
    }

    /// Calculates the score of a given tile (pow(2, level)).
    pub fn score(&self) -> u32 {
        2u32.pow(self.level)
    }
}

impl Default for Block {
    fn default() -> Self {
        Self { level: 1 }
    }
}

#[derive(Component)]
pub struct BlockText;

#[derive(Component)]
pub struct EmptyBlock;

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
