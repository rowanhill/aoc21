use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct GameState {
    players: [Player; 2],
    cur_player_index: usize,
}
impl GameState {
    fn new(p1_pos: u32, p2_pos: u32) -> GameState {
        GameState {
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

    fn next_states<const N: usize>(&self, die: &mut dyn Die<N>) -> [(GameState, u128); N] {
        let freq_distribution = die.roll_thrice();
        freq_distribution.map(|(rolls_sum, freq)| {
            let new_state = GameState {
                players: [
                    if self.cur_player_index == 0 {
                        self.players[0].move_and_score(&rolls_sum)
                    } else {
                        self.players[0].clone()
                    },
                    if self.cur_player_index == 1 {
                        self.players[1].move_and_score(&rolls_sum)
                    } else {
                        self.players[1].clone()
                    }
                ],
                cur_player_index: (self.cur_player_index + 1) % 2,
            };
            (new_state, freq)
        })
    }

    fn winning_player_index(&self, win_threshold: &u32) -> Option<usize> {
        if &self.players[0].score >= win_threshold {
            Some(0)
        } else if &self.players[1].score >= win_threshold {
            Some(1)
        } else {
            None
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Player {
    position_index: u32,
    score: u32,
}
impl Player {
    fn move_and_score(&self, spaces: &u32) -> Player {
        let new_position_index = (self.position_index + spaces) % 10;
        Player {
            position_index: new_position_index,
            score: self.score + new_position_index + 1,
        }
    }
}

trait Die<const N: usize> {
    fn roll_thrice(&mut self) -> [(u32, u128); N];
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
impl Die<1> for DeterministicDie {
    fn roll_thrice(&mut self) -> [(u32, u128); 1] {
        let result = (self.num_rolls % self.size) + 1 +
            ((self.num_rolls + 1) % self.size) + 1 +
            ((self.num_rolls + 2) % self.size) + 1;
        self.num_rolls += 3;
        [(result, 1)]
    }
}

struct DiracDie {}
impl Die<7> for DiracDie {
    fn roll_thrice(&mut self) -> [(u32, u128); 7] {
        [
            (3, 1),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1),
        ]
    }
}

struct DeterministicGameRunner {
    game_state: GameState,
    die: DeterministicDie,
}
impl DeterministicGameRunner {
    fn new(p1_pos: u32, p2_pos: u32) -> DeterministicGameRunner {
        DeterministicGameRunner {
            die: DeterministicDie::new(100),
            game_state: GameState::new(p1_pos, p2_pos),
        }
    }

    fn play_to_completion(&mut self, win_threshold: &u32) -> GameResult {
        loop {
            match self.game_state.winning_player_index(win_threshold) {
                None => {
                    self.play_turn();
                },
                Some(winning_player_index) => {
                    return GameResult {
                        loser_score: self.game_state.players[(winning_player_index + 1) % 2].score,
                        num_rolls: self.die.num_rolls,
                    }
                },
            }
        }
    }

    fn play_turn(&mut self) {
        let [(new_state, _)] = self.game_state.next_states(&mut self.die);
        self.game_state = new_state;
    }
}

struct GameResult {
    loser_score: u32,
    num_rolls: u32,
}
impl GameResult {
    fn part1_score(&self) -> u32 {
        self.loser_score * self.num_rolls
    }
}

struct NondeterministicGameRunner {
    die: DiracDie,
    unfinished_games: HashMap<GameState, u128>,
    win_counts: [u128; 2],
}
impl NondeterministicGameRunner {
    fn new(p1_pos: u32, p2_pos: u32) -> NondeterministicGameRunner {
        NondeterministicGameRunner {
            die: DiracDie {},
            unfinished_games: HashMap::from([
                (GameState::new(p1_pos, p2_pos), 1),
            ]),
            win_counts: [0, 0],
        }
    }

    fn play_to_completion(&mut self, win_threshold: &u32) {
        while !self.unfinished_games.is_empty() {
            let mut new_unfinished_games = HashMap::new();
            for (state, num_universes) in &self.unfinished_games {
                for (new_state, count) in state.next_states(&mut self.die) {
                    if let Some(winner_index) = new_state.winning_player_index(win_threshold) {
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
    let mut game = DeterministicGameRunner::new(4, 6);
    let game_result = game.play_to_completion(&1000);
    let p1 = game_result.part1_score();
    println!("Part 1: {}", p1);

    let mut multiverse = NondeterministicGameRunner::new(4, 6);
    multiverse.play_to_completion(&21);
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
        let mut runner = DeterministicGameRunner::new(4, 8);
        let game_result = runner.play_to_completion(&1000);
        assert_eq!(game_result.num_rolls, 993);
        assert_eq!(game_result.loser_score, 745);
        let p1 = game_result.part1_score();
        assert_eq!(p1, 739785);
    }

    #[test]
    fn test_example_two_moves() {
        let mut runner = DeterministicGameRunner::new(4, 8);
        runner.play_turn();
        runner.play_turn();
        assert_eq!(runner.game_state.players[0].score, 10);
        assert_eq!(runner.game_state.players[1].score, 3);
    }

    #[test]
    fn test_example_gives_correct_part2_result() {
        let mut runner = NondeterministicGameRunner::new(4, 8);
        runner.play_to_completion(&21);
        assert_eq!(runner.win_counts[0], 444356092776315);
        assert_eq!(runner.win_counts[1], 341960390180808);
    }
}