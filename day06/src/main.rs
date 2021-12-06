fn main() {
    let fish_list = vec![2u8,4,1,5,1,3,1,1,5,2,2,5,4,2,1,2,5,3,2,4,1,3,5,3,1,3,1,3,5,4,1,1,1,1,5,1,2,5,5,5,2,3,4,1,1,1,2,1,4,1,3,2,1,4,3,1,4,1,5,4,5,1,4,1,2,2,3,1,1,1,2,5,1,1,1,2,1,1,2,2,1,4,3,3,1,1,1,2,1,2,5,4,1,4,3,1,5,5,1,3,1,5,1,5,2,4,5,1,2,1,1,5,4,1,1,4,5,3,1,4,5,1,3,2,2,1,1,1,4,5,2,2,5,1,4,5,2,1,1,5,3,1,1,1,3,1,2,3,3,1,4,3,1,2,3,1,4,2,1,2,5,4,2,5,4,1,1,2,1,2,4,3,3,1,1,5,1,1,1,1,1,3,1,4,1,4,1,2,3,5,1,2,5,4,5,4,1,3,1,4,3,1,2,2,2,1,5,1,1,1,3,2,1,3,5,2,1,1,4,4,3,5,3,5,1,4,3,1,3,5,1,3,4,1,2,5,2,1,5,4,3,4,1,3,3,5,1,1,3,5,3,3,4,3,5,5,1,4,1,1,3,5,5,1,5,4,4,1,3,1,1,1,1,3,2,1,2,3,1,5,1,1,1,4,3,1,1,1,1,1,1,1,1,1,2,1,1,2,5,3];

    let mut fish_by_age = [0u64; 9];
    for fish in fish_list {
        fish_by_age[fish as usize] += 1;
    }

    // print!("Day 0 -- ");
    // println!("[{}, {}, {}, {}, {}, {}, {}, {}, {}]", fish_by_age[0],fish_by_age[1],fish_by_age[2],fish_by_age[3],fish_by_age[4],fish_by_age[5],fish_by_age[6],fish_by_age[7],fish_by_age[8]);

    for i in 1..257 {
        fish_by_age = [
            fish_by_age[1],
            fish_by_age[2],
            fish_by_age[3],
            fish_by_age[4],
            fish_by_age[5],
            fish_by_age[6],
            fish_by_age[7] + fish_by_age[0],
            fish_by_age[8],
            fish_by_age[0],
        ];
        // print!("Day {} -- ", i);
        // println!("[{}, {}, {}, {}, {}, {}, {}, {}, {}]", fish_by_age[0],fish_by_age[1],fish_by_age[2],fish_by_age[3],fish_by_age[4],fish_by_age[5],fish_by_age[6],fish_by_age[7],fish_by_age[8]);
        if i == 80 {
            println!("Part 1: {}", fish_by_age.iter().sum::<u64>());
        }
    }
    println!("Part 2: {}", fish_by_age.iter().sum::<u64>());

    // for i in 0..256 {
    //     println!("Day {}", i);
    //     let mut new_fish: Vec<u8> = vec![];
    //     for fish in fish_list.iter_mut() {
    //         match fish {
    //             0 => {
    //                 *fish = 6;
    //                 new_fish.push(8);
    //             },
    //             _ => {
    //                 *fish -= 1;
    //             }
    //         }
    //     }
    //     fish_list.append(&mut new_fish);
    // }
    //
    // println!("Part 1: {}", fish_list.len());
}
