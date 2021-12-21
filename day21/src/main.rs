use std::collections::HashMap;

trait Die {
    fn next(&mut self) -> u32;
}

struct DeterministicDie {
    size: u32,
    num_rolls: u32,
}
impl DeterministicDie {
    fn new(size: u32) -> Self {
        DeterministicDie { num_rolls: 0, size }
    }
}
impl Die for DeterministicDie {
    fn next(&mut self) -> u32 {
        let result = (self.num_rolls % self.size) + 1;
        self.num_rolls += 1;
        result
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Player {
    position_index: u32,
    score: u32,
}
impl Player {
    fn move_and_score(&mut self, spaces: &u32) {
        self.position_index = (self.position_index + spaces) % 10;
        self.score += self.position_index + 1;
    }
}

struct DeterministicGame {
    players: [Player; 2],
    cur_player_index: usize,
    die: DeterministicDie,
}
impl DeterministicGame {
    fn new(p1_pos: u32, p2_pos: u32) -> DeterministicGame {
        DeterministicGame {
            players: [
                Player {
                    position_index: p1_pos - 1,
                    score: 0,
                },
                Player {
                    position_index: p2_pos - 1,
                    score: 0,
                }
            ],
            cur_player_index: 0,
            die: DeterministicDie::new(100),
        }
    }

    fn play_until_victory(&mut self, win_threshold: &u32) -> GameResult {
        loop {
            match self.game_is_won(win_threshold) {
                None => self.play(),
                Some(game_result) => return game_result,
            }
        }
    }

    fn play(&mut self) {
        let spaces_to_move = self.roll_n(3);
        self.players[self.cur_player_index].move_and_score(&spaces_to_move);

        self.cur_player_index = (self.cur_player_index + 1) % 2;
    }

    fn game_is_won(&self, win_threshold: &u32) -> Option<GameResult> {
        if &self.players[0].score >= win_threshold {
            Some(GameResult {
                loser: self.players[1].clone(),
                num_rolls: self.die.num_rolls,
            })
        } else if &self.players[1].score >= win_threshold {
            Some(GameResult {
                loser: self.players[0].clone(),
                num_rolls: self.die.num_rolls,
            })
        } else {
            None
        }
    }

    fn roll_n(&mut self, n: u32) -> u32 {
        (0..n).map(|_| self.die.next()).sum()
    }
}

struct GameResult {
    loser: Player,
    num_rolls: u32,
}
impl GameResult {
    fn part1_score(&self) -> u32 {
        self.loser.score * self.num_rolls
    }
}

#[derive(Hash, Eq, PartialEq)]
struct DiracGameState {
    players: [Player; 2],
    cur_player_index: usize,
}
impl DiracGameState {
    fn new(p1_pos: u32, p2_pos: u32) -> DiracGameState {
        DiracGameState {
            players: [
                Player {
                    position_index: p1_pos - 1,
                    score: 0,
                },
                Player {
                    position_index: p2_pos - 1,
                    score: 0,
                }
            ],
            cur_player_index: 0,
        }
    }

    fn play(&self) -> [(DiracGameState, u128); 7] {
        [
            self.play_roll(3, 1),
            self.play_roll(4, 3),
            self.play_roll(5, 6),
            self.play_roll(6, 7),
            self.play_roll(7, 6),
            self.play_roll(8, 3),
            self.play_roll(9, 1),
        ]
    }

    fn play_roll(&self, rolls_sum: u32, num_universes: u128) -> (DiracGameState, u128) {
        let mut new_players = self.players.map(|p| p.clone());
        new_players[self.cur_player_index].move_and_score(&rolls_sum);

        let new_player_index = (self.cur_player_index + 1) % 2;

        (DiracGameState {
            players: new_players,
            cur_player_index: new_player_index,
        }, num_universes)
    }

    fn winner(&self, win_threshold: &u32) -> Option<usize> {
        if &self.players[0].score >= win_threshold {
            Some(0)
        } else if &self.players[1].score >= win_threshold {
            Some(1)
        } else {
            None
        }
    }
}

struct DiracGameMultiverse {
    unfinished_games: HashMap<DiracGameState, u128>,
    win_counts: [u128; 2],
}
impl DiracGameMultiverse {
    fn new(p1_pos: u32, p2_pos: u32) -> DiracGameMultiverse {
        DiracGameMultiverse {
            unfinished_games: HashMap::from([
                (DiracGameState::new(p1_pos, p2_pos), 1),
            ]),
            win_counts: [0, 0],
        }
    }

    fn play_to_victories(&mut self, win_threshold: &u32) {
        while !self.unfinished_games.is_empty() {
            let mut new_unfinished_games = HashMap::new();
            for (state, num_universes) in &self.unfinished_games {
                for (new_state, count) in state.play() {
                    if let Some(winner_index) = new_state.winner(win_threshold) {
                        self.win_counts[winner_index] += num_universes * count;
                    } else {
                        *new_unfinished_games.entry(new_state).or_default() += num_universes * count;
                    }
                }
            }
            self.unfinished_games = new_unfinished_games;
        }
    }
}

fn main() {
    // Player 1 starting position: 4
    // Player 2 starting position: 6
    let mut game = DeterministicGame::new(4, 6);
    let game_result = game.play_until_victory(&1000);
    let p1 = game_result.part1_score();
    println!("Part 1: {}", p1);

    let mut multiverse = DiracGameMultiverse::new(4, 6);
    multiverse.play_to_victories(&21);
    let p2 = std::cmp::max(multiverse.win_counts[0], multiverse.win_counts[1]);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_gives_correct_part1_result() {
        // Player 1 starting position: 4
        // Player 2 starting position: 8
        let mut game = DeterministicGame::new(4, 8);
        let game_result = game.play_until_victory(&1000);
        assert_eq!(game_result.num_rolls, 993);
        assert_eq!(game_result.loser.score, 745);
        let p1 = game_result.part1_score();
        assert_eq!(p1, 739785);
    }

    #[test]
    fn test_example_two_moves() {
        let mut game = DeterministicGame::new(4, 8);
        game.play();
        game.play();
        assert_eq!(game.players[0].score, 10);
        assert_eq!(game.players[1].score, 3);
    }

    #[test]
    fn test_example_gives_correct_part2_result() {
        let mut multiverse = DiracGameMultiverse::new(4, 8);
        multiverse.play_to_victories(&21);
        assert_eq!(multiverse.win_counts[0], 444356092776315);
        assert_eq!(multiverse.win_counts[1], 341960390180808);
    }
}