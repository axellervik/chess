use chess::game::*;

// check test framework
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn game_in_progress_after_init() {

    let game = Game::new();

    println!("{:?}", game);

    assert_eq!(game.get_game_state(), GameState::InProgress);
}