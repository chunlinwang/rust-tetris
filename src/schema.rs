use crate::constants::{TETRIS_CONTAINER_HEIGHT, TETRIS_CONTAINER_WIDTH, TETRIS_BLOCKS, DROP_INTERVAL_LEVEL, MATRIX_HEIGHT, MATRIX_WIDTH, MIN_LEVEL, MAX_LEVEL, BASE_INTERVAL};
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct MatrixData
{
    pub data: [[usize; MATRIX_HEIGHT]; MATRIX_WIDTH],
}

#[derive(Debug, Copy, Clone)]
pub struct Block
{
    pub block_type: &'static str,
    pub matrix_data: [MatrixData; 4], // 最多4个方向数据
}

#[derive(Debug, Copy, Clone)]
pub struct Tetris
{
    pub back_ground: [[usize; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH],
    pub play_ground: [[usize; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH],
    pub direction: usize,
    pub block: Block,
    pub next_block: Block,
    pub pos_x: isize,
    pub pos_y: isize,
    pub score: usize,
    pub speed: usize,
    pub level: usize,
    pub round: usize,
    pub blocks: usize,
    pub is_game_over: bool,
}

impl Tetris {
    pub fn update_back_ground(&mut self, back_ground: [[usize; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH]) {
        self.back_ground = back_ground;
    }

    pub fn update_play_ground(&mut self, play_ground: [[usize; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH]) {
        self.play_ground = play_ground;
    }

    pub fn update_speed(&mut self, speed: usize) {
        self.speed = speed;
    }

    pub fn increase_level(&mut self) {
        if self.level < MAX_LEVEL {
            self.level += 1;
        }

        self.speed = BASE_INTERVAL + (DROP_INTERVAL_LEVEL / self.level);
    }

    pub fn decrease_level(&mut self) {
        if self.level > MIN_LEVEL {
            self.level -= 1;
        }

        self.speed = BASE_INTERVAL + (DROP_INTERVAL_LEVEL / self.level);
    }

    pub fn reset_speed(&mut self) {
        self.speed = BASE_INTERVAL + (DROP_INTERVAL_LEVEL / self.level);
    }

    pub fn create_new_block(&mut self) {
        let mut rng: ThreadRng = thread_rng();
        self.back_ground = self.play_ground;
        self.pos_x = -4; // 对应ui的Y轴
        self.pos_y = 3; // 对应ui的X轴
        self.block = self.next_block;
        self.next_block = TETRIS_BLOCKS[rng.gen_range(0..7)];
        self.direction = rng.gen_range(0..4);
        self.speed = BASE_INTERVAL + (DROP_INTERVAL_LEVEL / self.level);
        self.blocks += 1;
        self.round = 0;
    }

    pub fn new() -> Tetris {
        let mut rng: ThreadRng = thread_rng();

        Tetris {
            back_ground: [[0; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH],
            play_ground: [[0; TETRIS_CONTAINER_HEIGHT]; TETRIS_CONTAINER_WIDTH],
            direction: 0,
            block: TETRIS_BLOCKS[rng.gen_range(0..7)],
            next_block: TETRIS_BLOCKS[rng.gen_range(0..7)],
            pos_x: -4, // 对应ui的Y轴
            pos_y: 3, // 对应ui的X轴
            score: 0,
            speed: BASE_INTERVAL + (DROP_INTERVAL_LEVEL / 5),
            round: 0,
            blocks: 1,
            is_game_over: false,
            level: 5,
        }
    }
}

pub enum MoveAction {
    Left,
    Right,
    Down,
}