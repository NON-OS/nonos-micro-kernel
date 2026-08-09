use tools_2048::{Game, GameMove};

#[test]
fn seeded_games_are_deterministic() {
    let mut a = Game::<4>::new_seeded(42).unwrap();
    let mut b = Game::<4>::new_seeded(42).unwrap();
    assert_eq!(a.board(), b.board());
    a.make_move(GameMove::Left);
    b.make_move(GameMove::Left);
    assert_eq!(a.board(), b.board());
}

#[test]
fn different_seeds_or_no_panic() {
    let mut g = Game::<4>::new_seeded(7).unwrap();
    let before = *g.board();
    let moved = g.make_move(GameMove::Left);
    if moved {
        assert_ne!(&before, g.board());
    }
}
