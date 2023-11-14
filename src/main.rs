use comfy::*;

mod core {
    pub mod player;
    pub mod player_controller;
}

simple_game!("SuperComfy", setup, update);

fn setup(_c: &mut EngineContext) {
    todo!()
}

fn update(_c: &mut EngineContext) {
    core::player::draw_player();
}
