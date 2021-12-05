use std::fs;

#[derive(Copy, Clone)]
struct BoardNum {
    num: u32,
    is_marked: bool
}
struct Board {
    nums: [BoardNum; 25]
}
impl Board {
    fn new(nums: &Vec<u32>) -> Board {
        if nums.len() != 25 {
            panic!("Wrong number of numbers for a board");
        }
        let mut nums_arr: [BoardNum; 25] = [BoardNum { num: 0, is_marked: false }; 25];
        for (index, num) in nums.iter().enumerate() {
            nums_arr[index] = BoardNum { num: *num, is_marked: false };
        }
        Board {
            nums: nums_arr
        }
    }

    fn mark_if_present(&mut self, number: u32) {
        let indexes: Vec<usize> = self.nums.iter().enumerate()
            .filter(|&(_, board_num)| board_num.num == number)
            .map(|(index, _)| index)
            .collect();
        for index in indexes {
            self.nums[index].is_marked = true;
        }
    }

    fn is_bingo(&self) -> bool {
        // Rows
        self.are_marked([ 0,  1,  2,  3,  4]) ||
        self.are_marked([ 5,  6,  7,  8,  9]) ||
        self.are_marked([10, 11, 12, 13, 14]) ||
        self.are_marked([15, 16, 17, 18, 19]) ||
        self.are_marked([20, 21, 22, 23, 24]) ||
        // Cols
        self.are_marked([ 0,  5, 10, 15, 20]) ||
        self.are_marked([ 1,  6, 11, 16, 21]) ||
        self.are_marked([ 2,  7, 12, 17, 22]) ||
        self.are_marked([ 3,  8, 13, 18, 23]) ||
        self.are_marked([ 4,  9, 14, 19, 24])
    }

    fn is_marked(&self, index: usize) -> bool {
        self.nums[index].is_marked
    }

    fn are_marked(&self, indexes: [usize; 5]) -> bool {
        self.is_marked(indexes[0]) &&
        self.is_marked(indexes[1]) &&
        self.is_marked(indexes[2]) &&
        self.is_marked(indexes[3]) &&
        self.is_marked(indexes[4])
    }

    fn sum_of_unmarked(&self) -> u32 {
        self.nums.iter()
            .filter(|bn| !bn.is_marked)
            .map(|bn| bn.num)
            .sum()
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let mut chunks = contents.split("\n\n");

    let draw_numbers: Vec<u32> = chunks.next().expect("Could not find first chunk")
        .split(",")
        .map(|s| s.parse().expect("Could not parse number"))
        .collect();

    let num_re = regex::Regex::new(r"\d+").unwrap();

    let mut boards: Vec<Board> = chunks.map(|board_chunk| {
        let nums: Vec<u32> = num_re.find_iter(board_chunk)
            .filter_map(|m| m.as_str().parse().ok())
            .collect();
        Board::new(&nums)
    }).collect();

    let mut winning_board_count = 0;
    let num_boards = boards.len();
    'outer: for drawn_num in draw_numbers {
        for board in boards.iter_mut() {
            if !board.is_bingo() {
                board.mark_if_present(drawn_num);
                if board.is_bingo() {
                    winning_board_count += 1;
                    if winning_board_count == 1 {
                        println!("Part 1: {}", board.sum_of_unmarked() * drawn_num);
                    } else if winning_board_count == num_boards {
                        println!("Part 2: {}", board.sum_of_unmarked() * drawn_num);
                        break 'outer;
                    }
                }
            }
        }
    }
}
