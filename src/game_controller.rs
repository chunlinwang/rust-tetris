use crate::constants::{MATRIX_WIDTH, TETRIS_CONTAINER_HEIGHT, TETRIS_CONTAINER_WIDTH, MATRIX_HEIGHT, SCORE_COEFFICIENT};
use crate::schema::{Tetris, MatrixData, MoveAction};

pub fn draw_play_ground(tetris: &mut Tetris) {
    let mut back_ground = tetris.back_ground;

    for i in 0..MATRIX_WIDTH {
        for j in 0..MATRIX_HEIGHT {
            let cur_dir_data: MatrixData = tetris.block.matrix_data[tetris.direction];
            if cur_dir_data.data[i][j] == 0 {
                continue;  // 方块4*4数组中数据为0的位置忽略
            }

            let pos_x: isize = i as isize + tetris.pos_x;
            let pos_y: isize = j as isize + tetris.pos_y;

            if pos_x >= 0 && pos_y >= 0 && pos_x < (TETRIS_CONTAINER_HEIGHT as isize) && pos_y < (TETRIS_CONTAINER_WIDTH as isize) {
                back_ground[pos_y as usize][pos_x as usize] = 1;
            }
        }
    }

    &tetris.update_play_ground(back_ground);
}

pub fn move_action(tetris: &mut Tetris, move_action: MoveAction) -> bool {
    draw_play_ground(tetris);
    let back_ground = tetris.back_ground;
    for i in 0..MATRIX_WIDTH {
        for j in 0..MATRIX_HEIGHT {
            let cur_dir_data: MatrixData = tetris.block.matrix_data[tetris.direction];
            if cur_dir_data.data[i][j] == 0 {
                continue;
            }

            let mut pos_x = i as isize + tetris.pos_x;
            let mut pos_y = j as isize + tetris.pos_y;

            match move_action {
                MoveAction::Left => { pos_y -= 1; }
                MoveAction::Right => { pos_y += 1; }
                MoveAction::Down => { pos_x += 1; }
                _ => {}
            }

            if pos_y < 0 {
                return false;
            }

            if pos_x >= TETRIS_CONTAINER_HEIGHT as isize ||
                pos_y >= TETRIS_CONTAINER_WIDTH as isize
            {
                return false;
            }

            if pos_x > 0 && pos_y >= 0 && back_ground[pos_y as usize][pos_x as usize] == 1 {
                return false;
            }
        }
    }

    match move_action {
        MoveAction::Left => { tetris.pos_y -= 1; }
        MoveAction::Right => { tetris.pos_y += 1; }
        MoveAction::Down => {
            tetris.pos_x += 1;
            tetris.round += 1;
        }
        _ => {}
    }

    draw_play_ground(tetris);

    return true;
}

pub fn game_over_checker(tetris: &mut Tetris) {
    let back_ground = tetris.play_ground;

    for i in 0..TETRIS_CONTAINER_WIDTH {
        if back_ground[i][0] == 1 {
            tetris.is_game_over = true;
        }
    }
}

pub fn check_clear_line(tetris: &mut Tetris) {
    let mut back_ground = tetris.play_ground;
    let mut j: usize = 0;
    let mut to_remove_lines = Vec::new();
    let mut count_removed_lines: usize = 0;
    loop {
        let mut is_break: bool = false;
        for i in 0..TETRIS_CONTAINER_WIDTH {
            if back_ground[i][j] != 1 {
                is_break = true;
                break;
            }
        }

        if !is_break {
            to_remove_lines.push(j);
        }

        j += 1;

        if j == TETRIS_CONTAINER_HEIGHT {
            break;
        }
    }

    for r in to_remove_lines {
        let mut k: usize = r;

        loop {
            for x in 0..TETRIS_CONTAINER_WIDTH {
                back_ground[x][k] = back_ground[x][k - 1];
            }

            k -= 1;

            if k == 0 {
                break;
            }
        }

        for i in 0..TETRIS_CONTAINER_WIDTH {
            back_ground[0][i] = 0;
        }

        count_removed_lines += 1;
    }

    if count_removed_lines > 0 {
        tetris.score += SCORE_COEFFICIENT * 2_usize.pow((count_removed_lines - 1) as u32)
    }

    tetris.play_ground = back_ground;
    tetris.back_ground = back_ground;
}