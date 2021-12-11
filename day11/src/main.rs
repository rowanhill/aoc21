type Octo = (u32, bool);
struct OctoMap {
    octos: [[Octo; 10]; 10]
}

impl OctoMap {
    fn parse(string: &str) -> OctoMap {
        assert_eq!(string.lines().count(), 10, "There must be 10 lines in the input string");
        let lines = string.lines();
        let mut octos = [[(0u32, false); 10]; 10];
        for (line_index, line) in lines.into_iter().enumerate() {
            assert_eq!(line.chars().count(), 10, "There must be 10 digits in each line of input");
            let digits = line.chars();
            for (digit_index, digit) in digits.into_iter().enumerate() {
                let digit = digit.to_digit(10).expect("Could not parse digit");
                octos[line_index][digit_index] = (digit, false);
            }
        }
        OctoMap { octos }
    }

    fn step(&mut self) -> u32 {
        // Reset octos to not having flashed
        for row in self.octos.iter_mut() {
            for mut octo in row.iter_mut() {
                (*octo).1 = false
            }
        }

        let mut flash_count = 0;

        // Increment all octos
        let mut to_flash = vec![];
        for (row_index, row) in self.octos.iter_mut().enumerate() {
            for (octo_index, mut octo) in row.iter_mut().enumerate() {
                (*octo).0 += 1;
                if octo.0 > 9 {
                    (*octo).1 = true;
                    (*octo).0 = 0;
                    to_flash.push((row_index, octo_index));
                    flash_count += 1;
                }
            }
        }

        // Process flashes
        while let Some((row_index, octo_index)) = to_flash.pop() {
            for y in (row_index as i32 - 1).clamp(0, 9)..(row_index as i32 + 2).clamp(0, 10) {
                for x in (octo_index as i32 - 1).clamp(0, 9)..(octo_index as i32 + 2).clamp(0, 10) {
                    let mut octo = self.octos
                        .get_mut(y as usize).expect("Could not get row")
                        .get_mut(x as usize).expect("Could not get octo");
                    if octo.1 == false {
                        (*octo).0 += 1;
                        if octo.0 > 9 {
                            (*octo).1 = true;
                            (*octo).0 = 0;
                            to_flash.push((y as usize, x as usize));
                            flash_count += 1;
                        }
                    }
                }
            }
        }
        
        flash_count
    }
}

fn main() {
    let input = "2566885432
3857414357
6761543247
5477332114
3731585385
1716783173
1277321612
3371176148
1162578285
6144726367";
    let mut map = OctoMap::parse(input);

    let mut total_flashes = 0;
    for _ in 1..101 {
        total_flashes += map.step();
    }
    println!("Part 1: {}", total_flashes);

    for i in 101..10000 {
        let flashes = map.step();
        if flashes >= 100 {
            println!("Part 2: {} ({} flashes)", i, flashes);
            break;
        }
    }
}
