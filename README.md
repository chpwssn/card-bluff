# card-bluff

`card-bluff` is a simulated "game" wherein a dealer magically creates some players and asks them to "draw" a card from a deck of cards and and then asks for the value of the card they drew. If the player draws a `*` for either the suit or the card it must select at random until it gets a non-`*` value, however the player cannot use this new value for attestation.

The dealer then asks the peers which card they have. If the dealer encounters a duplicate it will ask the peer to verify its card by building a message that contains a random string and the card face. If the peer cannot produce an answer to the challenge it is kicked out of the game (the dealer doesn't connect to it in the next round).

## Example "game"play

1. Nodes draw cards
   | Node | Card |
   |--|--|
   |A | H8 - 8 of Hearts |
   |B | S0 - 10 of Spades |
   |C | CK - King of Clubs |
   |D | DA - Ace of Diamonds |
   |E | *8 -> H8 First selects `*8`, randomly selects suit and gets 8 of Hearts |

2. Dealer connects to each and asks for cards. Each replies with `H8`, `S0`, `CK`, `DA`, and `H8` respectively
3. The dealer sees duplicates and asks `A` and `E` to respond with `bullshit` concatenated with the player's card
4. A receives the query, builds the message `bullshitH8`, and responds. The message is valid and the dealer moves on.
5. E receives the query, builds the message `bullshit*8`, and response. The message doesn't match so the dealer kicks E from the subsequent rounds

# This "implementation" of this "game" is awful...

Yes, it's simply a crate to play around and learn some Rust. The players can cheat and there can be legitimate duplicates, no one said it was a good implementation...

# Running The Game

```
git clone git@github.com:chpwssn/card-bluff.git
cd card-bluff
cargo run
```

## Example output

If you'd like to see an example of the output without building the binary locally see the [example.txt](example.txt) file.
