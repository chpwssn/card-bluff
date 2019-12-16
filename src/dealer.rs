use super::player::Player;
use crate::player::Draw;
use std::fmt;

pub struct Participant {
    name: String,
    player: Player,
}
impl fmt::Display for Participant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Participant {} Player: {}", self.name, self.player)
    }
}
impl PartialEq for Participant {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
pub struct Dealer {
    pub round: u32,
    pub participants: Vec<Participant>,
}

impl Dealer {
    #[inline]
    pub fn new() -> Dealer {
        Dealer {
            round: 0,
            participants: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player, name: String) {
        self.participants.push(Participant { player, name })
    }
    pub fn challenge_string(&self, draw: &Draw, challenge: &String) -> String {
        String::from(format!("{}{}", challenge, draw))
    }
    pub fn can_play_round(&self) -> bool {
        self.participants.len() > 1
    }
    pub fn play_round(&mut self) {
        // Trigger new Draws
        for participant in self.participants.iter_mut() {
            participant.player.draw()
        }
        // Print for the user
        for participant in self.participants.iter() {
            println!("{}", participant)
        }
        // New memory
        let mut memory: Vec<&Draw> = Vec::new();
        let mut liars: Vec<usize> = Vec::new();
        // Iterate
        for (i, participant) in self.participants.iter().enumerate() {
            let participant_draw: &Draw = participant.player.query();
            if memory.contains(&participant_draw) {
                println!("Found a duplicate!");
                let challenge: String = String::from("bullshit");
                let expected_response: String = self.challenge_string(participant_draw, &challenge);
                let response: String = participant.player.respond(&challenge);
                let response_ok: bool = response == expected_response;
                println!(
                    "({}) challenge {}, expected response {}, response {}",
                    response_ok, challenge, expected_response, response
                );
                if !response_ok {
                    println!("Liar! Participant {} will be removed!", participant.name);
                    liars.push(i)
                }
            }
            memory.push(participant_draw);
        }
        for liar in liars.iter() {
            self.participants.remove(*liar);
        }
        self.round += 1;
        println!("Round {} complete", self.round);
    }
}

impl fmt::Display for Dealer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "On Round: {}", self.round)
    }
}
