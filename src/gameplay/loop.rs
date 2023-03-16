use rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::gameplay::demo_fns;
use crate::models::card::Mana;
use crate::models::player::Player;
use crate::utils::create_basic_land;

use super::demo_fns::create_random_deck;

pub struct GameConfig {
    pub starting_life: i32,
}

pub struct GameState {
    cur_turn_num: u32,
    players: Vec<Player>,
}

fn get_card_selection() -> Option<usize> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    if input == "q" {
        return None;
    }

    let selection_result = input.parse::<usize>();
    let selection = if let Ok(selection) = selection_result {
        selection
    } else {
        println!("Invalid selection, try again Muffin");
        return get_card_selection();
    };

    Some(selection)
}

fn draw_card(player: &mut Player) {
    let card = player.library.pop().unwrap();
    player.hand.push(card);
}

fn play_card_maybe(player: &mut Player) {
    show_hand(player);

    let card_selection: Option<usize> = get_card_selection();

    let card = match card_selection {
        Some(card_selection) => player.hand.remove(card_selection),
        None => return,
    };

    println!("Playing card: {:?}", card);
}

fn show_hand(player: &Player) {
    println!("{}'s hand:", player.name);
    for (index, card) in player.hand.iter().enumerate() {
        println!("  [{}]: {}", index, card.name);
    }
}

pub fn player_turn(state: &mut GameState, player_index: usize) {
    let mut player = &mut state.players[player_index];
    //draw_card(player);
    play_card_maybe(player);
}

pub fn run_game_loop(game_config: GameConfig) {
    let mut state = GameState {
        cur_turn_num: 0,
        players: vec![
            Player::new("Player 1".to_string(), game_config.starting_life),
            Player::new("Player 2".to_string(), game_config.starting_life),
        ],
    };

    for player in &mut state.players {
        let mut deck = create_random_deck();
        deck.shuffle(&mut thread_rng());
        player.library = deck.clone();

        for _ in 0..7 {
            draw_card(player);
        }
    }

    loop {
        state.cur_turn_num += 1;
        println!("Turn {}", state.cur_turn_num);

        player_turn(&mut state, 0);
        player_turn(&mut state, 1);
    }
}
