/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `aux`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

fn mv(num_discs: u32, moves: &mut Vec<Move>, src: Peg, aux: Peg, dst: Peg) {
    if num_discs > 0 {
        // Move n - 1 disks from source to auxiliary, so they are out of the way
        mv(num_discs - 1, moves, src, dst, aux);

        // Move the nth disk from source to target
        moves.push((src, dst));

        // Move the n - 1 disks that we left on auxiliary onto target
        mv(num_discs - 1, moves, aux, src, dst);
    }
}

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    mv(num_discs, &mut moves, src, aux, dst);

    moves
}
