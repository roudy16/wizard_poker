mod constants;
mod demo_fns;
mod gameplay;
mod models;
mod utils;

fn main() {
    demo_fns::create_one_of_each_card_type();
    demo_fns::create_one_of_each_basic_land();

    let config = gameplay::r#loop::GameConfig { starting_life: 20 };
    gameplay::r#loop::run_game_loop(config);
}
