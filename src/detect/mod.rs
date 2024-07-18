pub fn verify_move(boundary_x: &u8, boundary_y: &u8) {}

fn collision() {}

fn castle() {}

fn game_bound(bx: &u8, by: &u8) {}

fn en_passant(bx: &u8, by: &u8) {}

fn convert_pawn() {}

fn valid_move(bx: &u8, by: &u8) {}

fn p_move(bx: &u8, by: &u8) {}

fn r_move(bx: &u8, by: &u8) {}

fn b_move(bx: &u8, by: &u8) {}

fn k_move(bx: &u8, by: &u8) {}

fn q_move(bx: &u8, by: &u8) {
    r_move(bx, by);
    b_move(bx, by);
}

fn pinned_piece(bx: &u8, by: &u8) {}

fn check(bx: &u8, by: &u8) {}

fn checkmate(bx: &u8, by: &u8) {}
