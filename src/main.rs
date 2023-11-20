use kdam::tqdm;
use rand::Rng;

#[derive(Debug)]
enum Player {
    A,
    B,
}

fn generate_cards() -> Vec<u8> {
    let mut cards = vec![];
    for card in 7..=14 {
        for _ in 0..4 {
            cards.push(card)
        }
    }
    shuffle(&mut cards);
    cards
}

fn shuffle<TCard>(cards: &mut Vec<TCard>) {
    let mut rng = rand::thread_rng();
    for i in (1..cards.len()).rev() {
        let j = rng.gen_range(0..(i + 1));
        cards.swap(i, j);
    }
}

fn maybe_winner<TCard: std::cmp::PartialOrd>(
    player_a: &Vec<TCard>,
    player_b: &Vec<TCard>,
) -> Option<Player> {
    if player_a.is_empty() {
        Some(Player::B)
    } else if player_b.is_empty() {
        Some(Player::A)
    } else {
        None
    }
}

fn collect_cards<TCard: Copy>(
    winner: &mut Vec<TCard>,
    player_a_bet: &Vec<TCard>,
    player_b_bet: &Vec<TCard>,
) {
    let mut cards = vec![];
    cards.extend(player_a_bet);
    cards.extend(player_b_bet);
    if cards.len() == 2 {
        shuffle(&mut cards);
    }
    winner.splice(0..0, cards);
}

fn war<TCard: std::cmp::PartialOrd + Copy>(
    player_a: &mut Vec<TCard>,
    player_a_bet: &mut Vec<TCard>,
    player_b: &mut Vec<TCard>,
    player_b_bet: &mut Vec<TCard>,
) -> Option<Player> {
    //println!("War started, {}, {}", player_a.len(), player_b.len());
    for _ in 0..3 {
        if let Some(winner) = maybe_winner(player_a, player_b) {
            return Some(winner);
        }
        player_a_bet.push(player_a.pop().unwrap());
        player_b_bet.push(player_b.pop().unwrap());
    }

    let player_a_card = player_a_bet.last().unwrap();
    let player_b_card = player_b_bet.last().unwrap();

    if player_a_card > player_b_card {
        collect_cards(player_a, player_a_bet, player_b_bet);
        //println!("player A win war!, {}, {}", player_a.len(), player_b.len());
        None
    } else if player_b_card > player_a_card {
        collect_cards(player_b, player_a_bet, player_b_bet);
        //println!("player B win war!, {}, {}", player_a.len(), player_b.len());
        None
    } else {
        war(player_a, player_a_bet, player_b, player_b_bet)
    }
}

fn round<TCard: std::cmp::PartialOrd + Copy>(
    player_a: &mut Vec<TCard>,
    player_b: &mut Vec<TCard>,
) -> Option<Player> {
    if let Some(winner) = maybe_winner(player_a, player_b) {
        return Some(winner);
    }

    let player_a_card = player_a.pop().unwrap();
    let player_b_card = player_b.pop().unwrap();

    if player_a_card > player_b_card {
        collect_cards(player_a, &mut vec![player_a_card], &mut vec![player_b_card]);
        //println!("player A win round!");
    } else if player_b_card > player_a_card {
        collect_cards(player_b, &mut vec![player_a_card], &mut vec![player_b_card]);
        //println!("player B win round!");
    } else {
        let mut player_a_bet = vec![];
        let mut player_b_bet = vec![];
        return war(player_a, &mut player_a_bet, player_b, &mut player_b_bet);
    }

    None
}

fn game() -> bool {
    let cards = generate_cards();
    let cards = cards.split_at(cards.len() / 2);
    let mut player_a_cards = cards.0.to_vec();
    let mut player_b_cards = cards.1.to_vec();

    for _i in 1..1_000_000 {
        //println!("round {}: {:?} vs. {:?}", i, player_a_cards, player_b_cards);
        if let Some(_winner) = round(&mut player_a_cards, &mut player_b_cards) {
            //println!("After {} rounds player {:?} wins", i, winner);
            return true;
        }
    }

    false
}
fn main() {
    let mut finished = 0;
    let total_games = 1_000_000;
    for _ in tqdm!(0..total_games) {
        if game() {
            finished += 1;
        }
    }
    println!("Finished games {}/{}", finished, total_games);
}
