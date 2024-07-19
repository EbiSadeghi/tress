use std::rc;

use crate::tile;

pub fn verify_move(start: tile, end: tile, boundary_x: &u8, boundary_y: &u8) -> bool {
    let mut is_move_valid: bool = false;

    if (start.row == end.row) && (start.col == end.col) {
        // Then there was no move
    } else {
        // check to see if colliding with own team
    }

    return is_move_valid;
}

fn team(param: &tile) -> Option<bool> {
    let is_white: bool = false;

    // get gameboard

    // match against whites -> some true
    // match against blacks -> some false
    // match against other tile, or error -> None

    return None;
}

struct CollisionRes {
    is_collision: bool,
    is_same_team: bool,
}
fn collision(start: &tile, end: tile) -> CollisionRes {
    let res = CollisionRes {
        is_collision: false,
        is_same_team: false,
    };

    return res;
}

fn castle() {}

fn game_bound(bx: &u8, by: &u8) {}

fn en_passant(bx: &u8, by: &u8) {}

fn convert_pawn() {}

fn valid_move(bx: &u8, by: &u8) {}

fn p_move(bx: &u8, by: &u8) {}

fn r_move(start: tile, end: tile, bx: &u8, by: &u8) -> bool {
    let mut is_move_valid: bool = false;

    if start.row == end.row {
        // go along the columns to make sure there is no collision
        for ii in start.col..end.col {
            if ii == end.col {
                is_move_valid = true;
            }

            let cc = collision(
                &start,
                tile {
                    row: end.row,
                    col: ii,
                },
            );

            if cc.is_collision == true {
                break; // Collisions before the destination are not allowed
            }
        }
    } else if start.col == end.col {
        // go along the rows to make sure there is no collision
        for ii in start.row..end.row {
            is_move_valid = true;
            if ii == end.row {
                is_move_valid = true;
            }

            let cc = collision(
                &start,
                tile {
                    row: ii,
                    col: end.row,
                },
            );

            if cc.is_collision {
                break; // Collisions before the destination are not allowed
            }
        }
    }

    // Compare tile wanted with valid

    // Return true if it is
    return is_move_valid;
}

fn b_move(bx: &u8, by: &u8) {}

fn k_move(bx: &u8, by: &u8) {}

fn q_move(bx: &u8, by: &u8) {
    //r_move(bx, by);
    //b_move(bx, by);
}

fn pinned_piece(bx: &u8, by: &u8) {}

fn check(bx: &u8, by: &u8) {}

fn checkmate(bx: &u8, by: &u8) {}
