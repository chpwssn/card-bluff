use rand::Rng;
use std::fmt;
#[derive(Clone, Copy, Debug)]
pub struct Draw {
    card: char,
    suit: char,
}
impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.card)
    }
}
impl Draw {
    fn wild(&self) -> bool {
        self.card == '*' || self.suit == '*'
    }
    fn equals(&self, other: &Draw) -> bool {
        self.card == other.card && self.suit == other.suit
    }
}
impl PartialEq for Draw {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
pub struct Play {
    draw: Draw,
    lie: Draw,
}
impl Play {
    fn lying(&self) -> bool {
        self.draw.wild()
    }
    // Play our draw. If we're not lying play the first draw, else play our lie.
    pub fn play_card(&self) -> &Draw {
        if self.lying() {
            &self.lie
        } else {
            &self.draw
        }
    }

    // Respond to a challenge message by concatenating our first draw with the
    // challenge message. If you wanted to cheat.. this is where you'd do it
    pub fn respond(&self, challenge: &String) -> String {
        String::from(format!("{}{}", challenge, self.draw))
    }
    #[inline]
    pub fn start_play() -> Play {
        let mut lie: Draw = draw_card();
        loop {
            if !lie.wild() {
                break;
            }
            lie = draw_card()
        }
        Play {
            draw: draw_card(),
            lie,
        }
    }
}
impl fmt::Display for Play {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Draw: {}, Lie: {}, Lying: {}",
            self.draw,
            self.lie,
            self.lying()
        )
    }
}
fn rand_char(input: Vec<char>) -> char {
    input[rand::thread_rng().gen_range(0, input.len())]
}
fn draw_card() -> Draw {
    let cards = vec![
        '*', 'A', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'J', 'Q', 'K',
    ];
    let _suits = vec!['*', 'H', 'C', 'S', 'D'];
    Draw {
        card: rand_char(cards),
        suit: rand_char(_suits),
    }
}
pub struct Player {
    current_play: Play,
}

impl Player {
    pub fn draw(&mut self) {
        self.current_play = Play::start_play();
    }

    pub fn query(&self) -> &Draw {
        self.current_play.play_card()
    }
    pub fn respond(&self, challenge: &String) -> String {
        self.current_play.respond(challenge)
    }
    #[inline]
    pub fn new() -> Player {
        Player {
            current_play: Play::start_play(),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Current Play {}", self.current_play)
    }
}
