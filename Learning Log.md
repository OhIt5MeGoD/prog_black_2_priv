[18/02/2026]

Working with Oliver Veal and Jed Nicholson

Initial setup was simply a public repo, a private repo and a Rust file.

# Planning phase 1

- Decided to learn Rust
    - Popular modern language that's infamously fast and robust
    - I know exactly 0 Rust so this will be a good learning experience

## Initial ideas

- Calculator (GUI maybe?)
- Audio processing (fourier transform etc.)
- Minigames (in the terminal/CLI)

We settled on minigames, since this allows us to all work on the project and integrate together at the end, as well as learning different structures that we can teach each other.
The other two projects we decided would be either too little or too much for the current scope.

The games we intend to create are Snake, Connect 4 and Blackjack. We are each going to work together and learn Rust, share resources etc. After we each create our individual games, this should give us enough of a basis in Rust to come together and create a bot for Connect 4 for players to play against.

## Blackjack planning 

Rules can be read here https://en.wikipedia.org/wiki/Blackjack

### Game flow

1. Uses the 52 card deck (shuffled)
2. Player makes a bet (optional)
3. Dealer draws 2 cards
4. Player draws 2 cards
5. If player gets Blackjack they win (1.5x payout), restart game
6. Player draws, multiple options<br>
    (a) Player goes bust <br>
    (b) Player gets 21<br>
    (c) Player stands<br>
    (d) Player draws another card<br>
7. Dealer plays, following same rules but doesn't draw higher than 17
8. Player either has a higher (wins), lower (loses) or equal ("push", bet is returned to player) score to the dealer
9. Game Restarts

### Programmable sections

- Game flow
    - Player input
    - Game logic
    - Dealer actions
- Visual (terminal) output

-----

# Project start
Began by compiling some resources, W3 Schools with a decent intro tutorial to get to grips with syntax and program flow, and looked around online for a decent CLI UI interface. One of my collaborators mentioned crossterm?

[11/03/26]

I started my Rust project using Cargo and Crates, but quickly found that Crates was deprecated and moved on to Dependi.

Also had to manually install some MSVC build tools.

Getting used to the stricter typing and how Rust formats module imports, as well as finding the modules that did what I needed, namely randomisation and inputs.

Someone made a module that allows you to use input!() like python's input(), instead of going through the std interface which I promptly installed.

Generating the initial deck was easy(ish), I was initially going to write out the whole deck by hand with suits. Then I realised in blackjack suits are irrelevant, then I realised I could write the array once for each denomination and "multiply" by 4 then randomise. However this left me then with a 2D array, since the enumeration doesn't concatenate.

Then I thought about it again and realised this was a lot of unneccessary steps instead of just writing the deck out once (a 52 card deck never changes, so generating like so is just more operations). It did however teach me some nice stuff about array and vector handling in Rust, alongside mapping and enumeration - a worthwhile early exercise. (If you look at my commit history you'll see 2 that are like 15 minutes apart after I realise I can just do it in a simpler way).





    