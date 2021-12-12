use std::collections::{HashMap, HashSet};

struct Network<'a> {
    neighbours: HashMap<&'a str, Vec<&'a str>>,
}

impl Network<'_> {
    fn parse(input: &str) -> Network {
        let mut neighbours = HashMap::new();

        for line in input.lines() {
            let (left, right) = line.split_once("-").expect("Could not split line");

            if !neighbours.contains_key(left) {
                neighbours.insert(left, vec![]);
            }
            neighbours.get_mut(left).expect("Could not find vec").push(right);

            if !neighbours.contains_key(right) {
                neighbours.insert(right, vec![]);
            }
            neighbours.get_mut(right).expect("Could not find vec").push(left);
        }

        Network { neighbours }
    }

    fn count_paths(&self, can_revisit_one: bool) -> u32 {
        Pathfinder::new(&self, can_revisit_one).count_paths("start")
    }
}

struct Pathfinder<'a> {
    network: &'a Network<'a>,
    visited: HashSet<&'a str>,
    can_revisit_one: bool,
    has_revisited: bool
}

trait CaveString {
    fn is_small(&self) -> bool;
}
impl CaveString for str {
    fn is_small(&self) -> bool {
        self.chars().all(|c| c.is_lowercase())
    }
}

impl <'a> Pathfinder<'a> {
    fn new(network: &'a Network, can_revisit_one: bool) -> Pathfinder<'a> {
        Pathfinder {
            network,
            visited: HashSet::new(),
            can_revisit_one,
            has_revisited: false
        }
    }

    fn count_paths(&mut self, from: &'a str) -> u32 {
        let mut paths = 0;

        let should_remove_from_visited = from.is_small() && self.visited.insert(from);

        for &neighbour in self.network.neighbours.get(from).unwrap() {
            if neighbour == "end" {
                paths += 1;
            } else {
                if !neighbour.is_small() || !self.visited.contains(neighbour) {
                    paths += self.count_paths(neighbour);
                } else if neighbour != "start" && self.can_revisit_one && !self.has_revisited {
                    self.has_revisited = true;
                    paths += self.count_paths(neighbour);
                    self.has_revisited = false;
                }
            }
        }

        if should_remove_from_visited {
            self.visited.remove(from);
        }

        paths
    }
}

fn main() {
    let input = "vp-BY
ui-oo
kk-IY
ij-vp
oo-start
SP-ij
kg-uj
ij-UH
SP-end
oo-IY
SP-kk
SP-vp
ui-ij
UH-ui
ij-IY
start-ui
IY-ui
uj-ui
kk-oo
IY-start
end-vp
uj-UH
ij-kk
UH-end
UH-kk";

    let network = Network::parse(input);
    let num_paths = network.count_paths(false);
    println!("Part 1: {}", num_paths);
    let num_paths = network.count_paths(true);
    println!("Part 2: {}", num_paths);
}
