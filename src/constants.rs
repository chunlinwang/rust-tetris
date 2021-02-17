use crate::schema::{Block, MatrixData};

pub const APP_NAME: &'static str = "Tetris";
pub const APP_AUTHOR: &'static str = "Author: Chunlin Wang";
pub const TETRIS_CONTAINER_WIDTH: usize = 10;
pub const TETRIS_CONTAINER_HEIGHT: usize = 20;

pub const MATRIX_WIDTH: usize = 4;
pub const MATRIX_HEIGHT: usize = 4;
pub const ADJUST_MARGIN: usize = 3;

pub const SCREEN_WIDTH: u32 = 480;
pub const SCREEN_HEIGHT: u32 = 768;

pub const MAX_LEVEL: usize = 10;
pub const MIN_LEVEL: usize = 1;
pub const DROP_INTERVAL_LEVEL: usize = 600;
pub const BASE_INTERVAL: usize = 100;
pub const RUSH_DOWN: usize = 16;
pub const SCORE_COEFFICIENT: usize = 10;

pub const TETRIS_BLOCKS: [Block; 7] = [
// I
    Block {
        block_type: "I",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 1, 1, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 1, 1, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
        ],
    },
// O
    Block {
        block_type: "O",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
        ],
    },
// S
    Block {
        block_type: "S",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [1, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [1, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
        ],
    },
// Z
    Block {
        block_type: "Z",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [1, 0, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [1, 0, 0, 0, ],
                ]
            },
        ],
    },
//L
    Block {
        block_type: "L",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 1, 0, ],
                    [1, 0, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 0, 1, 0, ],
                    [1, 1, 1, 0, ],
                ]
            },
        ],
    },
//J
    Block {
        block_type: "J",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [1, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [1, 0, 0, 0, ],
                    [1, 0, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 1, 0, ],
                    [0, 0, 1, 0, ],
                ]
            },
        ],
    },
//T
    Block {
        block_type: "T",
        matrix_data: [
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [1, 1, 1, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [0, 1, 1, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 0, 0, 0, ],
                    [1, 1, 1, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
            MatrixData {
                data: [
                    [0, 0, 0, 0, ],
                    [0, 1, 0, 0, ],
                    [1, 1, 0, 0, ],
                    [0, 1, 0, 0, ],
                ]
            },
        ],
    },
];
