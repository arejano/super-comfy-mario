use core::player;

mod core {
    pub mod player;
    pub mod player_controller;
}

fn main() {
    let _player = player::Player::new();
}
