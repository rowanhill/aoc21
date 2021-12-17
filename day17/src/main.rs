fn main() {
    // target area: x=277..318, y=-92..-53
    let target = (277..=318, -92..=-53);

    let mut total_highest = 0;
    let mut hit_counts = 0;
    for initial_dx in 1..(target.0.end()+1) {
        for initial_dy in *(target.1.start())..(-1*target.1.start() + 1) {
            let mut pos = (0, 0);
            let mut velocity = (initial_dx, initial_dy);
            // println!("Trial {:?}", velocity);

            let mut trial_highest = 0;
            loop {
                // Step
                pos = (pos.0 + velocity.0, pos.1 + velocity.1);
                velocity = (
                    if velocity.0 > 0 { velocity.0 - 1 } else { 0 },
                    velocity.1 - 1
                );

                // println!("  > pos: {:?}, vel: {:?}", pos, velocity);

                // Track peak height
                if pos.1 > trial_highest {
                    trial_highest = pos.1;
                }

                // Check for a hit
                if target.0.contains(&pos.0) && target.1.contains(&pos.1) {
                    if trial_highest > total_highest {
                        total_highest = trial_highest;
                    }
                    // println!("  > HIT");
                    // println!("  > {} highest", trial_highest);
                    hit_counts += 1;
                    break;
                }

                if &pos.0 > target.0.end() || &pos.1 < target.1.start() {
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", total_highest);
    println!("Part 2: {}", hit_counts);
}
