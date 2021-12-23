use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use crate::AmphipodVariety::{A, B, C, D};
use crate::BurrowLocation::{Hallway, Room};

#[derive(Clone, Hash, Eq, PartialEq)]
enum AmphipodVariety {
    A,
    B,
    C,
    D,
}
impl AmphipodVariety {
    fn room_index(&self) -> usize {
        match self {
            AmphipodVariety::A => 0,
            AmphipodVariety::B => 1,
            AmphipodVariety::C => 2,
            AmphipodVariety::D => 3,
        }
    }

    fn movement_cost(&self) -> usize {
        match self {
            AmphipodVariety::A => 1,
            AmphipodVariety::B => 10,
            AmphipodVariety::C => 100,
            AmphipodVariety::D => 1000,
        }
    }

    fn name(&self) -> &str {
        match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Amphipod {
    variety: AmphipodVariety,
    is_home: bool,
}

fn debug_char(amph: &Option<Amphipod>) -> &str {
    match amph {
        None => ".",
        Some(amph) => amph.variety.name()
    }
}

enum BurrowLocation {
    Room(usize, usize), // (index of room, index of space within room, top to bottom)
    Hallway(usize), // (index of space in hallway, left to right)
}

type Move = (BurrowLocation, BurrowLocation, usize);

struct State<const DEPTH: usize> {
    burrow: Burrow<DEPTH>,
    inverse_movement_cost: isize,
}
impl<const DEPTH: usize> State<DEPTH> {
    fn new(burrow: Burrow<DEPTH>) -> State<DEPTH> {
        State { burrow, inverse_movement_cost: 0 }
    }

    fn neighbour_states(&self) -> Vec<State<DEPTH>> {
        self.burrow.available_moves()
            .iter()
            .map(|mv| self.new_state_from_move(mv))
            .collect()
    }

    fn new_state_from_move(&self, mv: &Move) -> State<DEPTH> {
        let mut new_burrow = self.burrow.clone();
        let move_cost = new_burrow.apply_move(mv);
        State {
            burrow: new_burrow,
            inverse_movement_cost: self.inverse_movement_cost - move_cost as isize,
        }
    }
}

impl<const DEPTH: usize> Ord for State<DEPTH> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.inverse_movement_cost.cmp(&other.inverse_movement_cost)
    }
}
impl<const DEPTH: usize> PartialOrd for State<DEPTH> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<const DEPTH: usize> PartialEq<State<DEPTH>> for State<DEPTH> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<const DEPTH: usize> Eq for State<DEPTH> {}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Burrow<const DEPTH: usize> {
    rooms: [[Option<Amphipod>; DEPTH]; 4],
    hallway: [Option<Amphipod>; 11],
}

impl<const DEPTH: usize> Burrow<DEPTH> {
    fn new(rooms: [[AmphipodVariety; DEPTH]; 4]) -> Burrow<DEPTH> {
        let mut rooms = rooms.map(|r| {
            r.map(|variety| {
                Some(Amphipod { variety, is_home: false })
            })
        });
        for ri in 0..rooms.len() {
            let room = &mut rooms[ri];
            let mut lower_are_all_home = true;
            for si in 0..room.len() {
                let si = room.len() - si - 1;
                if let Some(amph) = &mut room[si] {
                    amph.is_home = lower_are_all_home && ri == amph.variety.room_index();
                    lower_are_all_home = amph.is_home;
                }
            }
        }
        Burrow {
            rooms,
            hallway: [None, None, None, None, None, None, None, None, None, None, None],
        }
    }

    fn print_map(&self) {
        println!("#############");
        print!("#");
        for h in &self.hallway {
            print!("{}", debug_char(h))
        }
        println!("#");
        println!(
            "###{}#{}#{}#{}###",
            debug_char(&self.rooms[0][0]),
            debug_char(&self.rooms[1][0]),
            debug_char(&self.rooms[2][0]),
            debug_char(&self.rooms[3][0]),
        );
        for i in 1..DEPTH {
            println!(
                "  #{}#{}#{}#{}#",
                debug_char(&self.rooms[0][i]),
                debug_char(&self.rooms[1][i]),
                debug_char(&self.rooms[2][i]),
                debug_char(&self.rooms[3][i]),
            );
        }
        println!("  #########");
    }

    fn is_complete(&self) -> bool {
        const EXPECTED_VARIETY_BY_ROOM_ID: [AmphipodVariety; 4] = [AmphipodVariety::A, AmphipodVariety::B, AmphipodVariety::C, AmphipodVariety::D];
        for i in 0..self.rooms.len() {
            let room = &self.rooms[i];
            for j in 0..room.len() {
                let amph = &room[j];
                if let Some(amph) = amph {
                    if amph.variety != EXPECTED_VARIETY_BY_ROOM_ID[i] {
                        // Mismatched variety
                        return false;
                    }
                } else {
                    // Empty room
                    return false;
                }
            }
        }
        true
    }

    // List of valid (from_location, to_location, spaces_moved) tuples
    fn available_moves(&self) -> Vec<Move> {
        let mut result = vec![];

        // Find amphs in hallway that can move to their destination room
        let hallway_moves = self.hallway.iter()
            .enumerate()
            .filter_map(|(i, maybe_amph)| {
                if let Some(amph) = maybe_amph {
                    Some((i, amph))
                } else {
                    None
                }
            })
            .filter_map(|(i, amph)| {
                let ri = amph.variety.room_index();
                let maybe_si = self.deepest_available_slot_index(&ri);
                if let Some(si) = maybe_si {
                    // There's a slot free. Now check if hallway is free
                    let target_hi = self.hallway_index_of_room(&ri);
                    let (low, high) = if target_hi < i {
                        (target_hi, i - 1)
                    } else {
                        (i + 1, target_hi)
                    };
                    let mut range = low..=high;
                    let all_free = range.all(|hi| self.hallway[hi].is_none());
                    if all_free {
                        let from = Hallway(i);
                        let to = Room(ri, si);
                        let moves = high - low + 1 + si + 1;
                        Some((from, to, moves))
                    } else {
                        None
                    }
                } else {
                    None
                }
            });
        result.extend(hallway_moves);

        // find hallway spots amphs in rooms other than their own can move to
        for (ri, room) in self.rooms.iter().enumerate() {
            let first_amph = room.iter().enumerate()
                .find(|(_, slot)| slot.is_some());
            if let Some((si, amph)) = first_amph {
                if let Some(amph) = amph {
                    if !amph.is_home {
                        for target_hi in [0, 1, 3, 5, 7, 9, 10] {
                            let starting_hi = self.hallway_index_of_room(&ri);
                            let low = std::cmp::min(starting_hi, target_hi);
                            let high = std::cmp::max(starting_hi, target_hi);
                            let mut range = low..=high;
                            let is_clear = range.all(|hi| self.hallway[hi].is_none());
                            if is_clear {
                                let from = Room(ri, si);
                                let to = Hallway(target_hi);
                                let moves = high - low + si + 1;
                                result.push((from, to, moves));
                            }
                        }
                    }
                }
            }
        }

        result
    }

    // Returns cost of the move
    fn apply_move(&mut self, mv: &Move) -> usize {
        // Move out of from
        let amph = match mv.0 {
            Room(ri, si) => {
                std::mem::replace(&mut self.rooms[ri][si], None)
            },
            Hallway(hi) => {
                std::mem::replace(&mut self.hallway[hi], None)
            }
        };

        let mut amph = amph.expect("Move's from location is empty");

        let cost = amph.variety.movement_cost() * mv.2;

        // Move into to
        match mv.1 {
            Room(ri, si) => {
                if self.rooms[ri][si].is_some() {
                    panic!("Move's to location is not empty");
                }
                amph.is_home = true;
                self.rooms[ri][si] = Some(amph);
            },
            Hallway(hi) => {
                if self.hallway[hi].is_some() {
                    panic!("Move's to location is not empty");
                }
                self.hallway[hi] = Some(amph);
            }
        }

        cost
    }

    fn deepest_available_slot_index(&self, &room_index: &usize) -> Option<usize> {
        let room = &self.rooms[room_index];
        for si in 0..room.len() {
            let si = room.len() - si - 1;
            if let Some(amph) = &room[si] {
                if !amph.is_home {
                    // Even if there's a free slow higher up, this amph still needs to move, so
                    // this room cannot be a destination
                    return None;
                }
            } else {
                // This slot is empty
                return Some(si);
            }
        }
        // If we reached the top without finding a slot, this room isn't a destination
        None
    }

    fn hallway_index_of_room(&self, room_index: &usize) -> usize {
        match room_index {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 8,
            _ => unreachable!("Unknown room index")
        }
    }
}

struct Search<const DEPTH: usize> {
    priority_queue: BinaryHeap<State<DEPTH>>,
    visited: HashSet<Burrow<DEPTH>>,
}
impl<const DEPTH: usize> Search<DEPTH> {
    fn new(start: State<DEPTH>) -> Search<DEPTH> {
        let mut search = Search {
            priority_queue: BinaryHeap::new(),
            visited: HashSet::new(),
        };
        search.priority_queue.push(start);
        search
    }

    fn find_cheapest_complete_state(&mut self) -> Option<State<DEPTH>> {
        while let Some(state) = self.priority_queue.pop() {
            if state.burrow.is_complete() {
                return Some(state);
            }
            if self.visited.contains(&state.burrow) {
                continue;
            }
            self.priority_queue.extend(state.neighbour_states());
            self.visited.insert(state.burrow);
        }
        None
    }
}

fn main() {
    // Part 1 example:
    // let burrow = Burrow::new([
    //     [B, A],
    //     [C, D],
    //     [B, C],
    //     [D, A]
    // ]);

    // Part 1 with expected moves working backwards - uncommenting more requires solving more steps
    // let mut burrow = Burrow::new([
    //     [A, A],
    //     [B, B],
    //     [C, C],
    //     [D, D]
    // ]);
    // burrow.apply_move(&(Room(0, 0), Hallway(9), 0));
    // if let Some(amph) = &mut burrow.hallway[9] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Room(3, 0), Hallway(5), 0));
    // if let Some(amph) = &mut burrow.hallway[5] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Room(3, 1), Hallway(7), 0));
    // if let Some(amph) = &mut burrow.hallway[7] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Hallway(9), Room(3, 1), 0));
    // if let Some(amph) = &mut burrow.rooms[3][1] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Hallway(7), Room(3, 0), 0));
    // if let Some(amph) = &mut burrow.rooms[3][0] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Room(1, 0), Room(0, 0), 0));
    // if let Some(amph) = &mut burrow.rooms[0][0] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Room(1, 1), Hallway(3), 0));
    // if let Some(amph) = &mut burrow.hallway[3] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Hallway(5), Room(1, 1), 0));
    // if let Some(amph) = &mut burrow.rooms[1][1] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Room(2, 0), Room(1, 0), 0));
    // if let Some(amph) = &mut burrow.rooms[1][0] {
    //     amph.is_home = false;
    // }
    // burrow.apply_move(&(Hallway(3), Room(2, 0), 0));
    // if let Some(amph) = &mut burrow.rooms[2][0] {
    //     amph.is_home = false;
    // }

    let part_1_burrow = Burrow::new([
        [A, C],
        [D, D],
        [C, B],
        [A, B],
    ]);
    let mut search = Search::new(State::new(part_1_burrow));
    let soln = search.find_cheapest_complete_state();
    if let Some(soln) = soln {
        soln.burrow.print_map();
        println!("Part 1: {}", -soln.inverse_movement_cost);
    } else {
        panic!("Could not find solution to part 1");
    }

    let part_2_burrow = Burrow::new([
        [A, D, D, C],
        [D, C, B, D],
        [C, B, A, B],
        [A, A, C, B],
    ]);
    let mut search = Search::new(State::new(part_2_burrow));
    let soln = search.find_cheapest_complete_state();
    if let Some(soln) = soln {
        soln.burrow.print_map();
        println!("Part 2: {}", -soln.inverse_movement_cost);
    } else {
        panic!("Could not find solution to part 2");
    }
}

/*
#############
#...........#
###A#D#C#A###
  #C#D#B#B#
  #########

#############
#.A.........#
###.#D#C#A###
  #C#D#B#B#
  #########

#############
#.A........A#
###.#D#C#.###
  #C#D#B#B#
  #########

#############
#.A.......BA#
###.#D#C#.###
  #C#D#B#.#
  #########

#############
#.A.......BA#
###.#.#C#.###
  #C#D#B#D#
  #########

#############
#.A.......BA#
###.#.#C#D###
  #C#.#B#D#
  #########

#############
#.A........A#
###.#.#C#D###
  #C#B#B#D#
  #########

#############
#.A.C......A#
###.#.#.#D###
  #C#B#B#D#
  #########

#############
#.A.C......A#
###.#B#.#D###
  #C#B#.#D#
  #########

#############
#.A........A#
###.#B#.#D###
  #C#B#C#D#
  #########

#############
#.A........A#
###.#B#C#D###
  #.#B#C#D#
  #########

#############
#..........A#
###.#B#C#D###
  #A#B#C#D#
  #########

#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########
 */
