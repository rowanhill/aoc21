use crate::AmphipodVariety::{A, B, C, D};
use crate::BurrowLocation::{Hallway, Room};

#[derive(Eq, PartialEq, Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
struct Burrow<const DEPTH: usize> {
    rooms: [[Option<Amphipod>; DEPTH]; 4],
    hallway: [Option<Amphipod>; 11],
    movement_cost: usize,
    solution_parent: Option<Box<Burrow<DEPTH>>>
}

impl<const DEPTH: usize> Burrow<DEPTH> {
    fn new(rooms: [[AmphipodVariety; DEPTH]; 4]) -> Burrow<DEPTH> {
        let mut rooms = rooms.map(|r| {
            r.map(|variety| {
                Some( Amphipod { variety, is_home: false })
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
            movement_cost: 0,
            solution_parent: None,
        }
    }

    fn find_solution(self) -> Option<Burrow<DEPTH>> {
        if self.is_complete() {
            return Some(self);
        }

        // println!("Looking from {}", self.movement_cost);

        let mut cheapest_solution = self.available_moves()
            .iter()
            .filter_map(|mv| self.new_state_from_move(&mv).find_solution())
            .min_by_key(|b| b.movement_cost);

        if let Some(soln) = &mut cheapest_solution {
            // println!("{} parent of {}", self.movement_cost, soln.movement_cost);
            soln.append_solution_parent(self);
        }

        cheapest_solution
    }

    fn append_solution_parent(&mut self, new_parent: Burrow<DEPTH>) {
        if let Some(parent) = &mut self.solution_parent {
            parent.append_solution_parent(new_parent);
        } else {
            self.solution_parent = Some(Box::new(new_parent))
        }
    }

    fn print_map_sequence(&self) {
        if let Some(parent) = &self.solution_parent {
            parent.print_map_sequence();
            println!();
        }
        self.print_map();
    }

    fn print_map(&self) {
        println!("{}:", self.movement_cost);
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
                "  #{}#{}#{}#{}###",
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

    fn new_state_from_move(&self, mv: &Move) -> Burrow<DEPTH> {
        let mut new_state = self.clone();
        new_state.apply_move(mv);
        new_state
    }

    fn apply_move(&mut self, mv: &Move) {
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

        // Update total cost
        self.movement_cost += amph.variety.movement_cost() * mv.2;

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

fn main() {
    // let burrow = Burrow::new([
    //     [A, C],
    //     [D, D],
    //     [C, B],
    //     [A, B]
    // ]);
    let burrow = Burrow::new([
        [B, A],
        [C, D],
        [B, C],
        [D, A]
    ]);
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
    // // burrow.apply_move(&(Room(3, 1), Hallway(7), 0));
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
    let soln = burrow.find_solution();
    if let Some(soln) = soln {
        soln.print_map_sequence();
        println!("Part 1: {}", soln.movement_cost);
    } else {
        panic!("Could not find solution to part 1");
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
