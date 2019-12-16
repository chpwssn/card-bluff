use card_bluff::dealer::Dealer;
use card_bluff::player::Player;

fn main() {
    let mut dealer: Dealer = Dealer::new();
    dealer.add_player(Player::new(), String::from("A"));
    dealer.add_player(Player::new(), String::from("B"));
    dealer.add_player(Player::new(), String::from("C"));
    dealer.add_player(Player::new(), String::from("D"));
    while dealer.can_play_round() {
        dealer.play_round();
    }
    println!("Made it {} rounds", dealer.round)
}
