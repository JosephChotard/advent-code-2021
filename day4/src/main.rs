fn main() {
    a();
    b();
}

const SIZE: usize = 5;

struct BingoBoard {
    grid: Vec<[usize; SIZE]>,
    marked_positions: Vec<[bool; SIZE]>,
}

impl BingoBoard {
    fn new_row(&mut self, row: [usize; SIZE]) {
        self.grid.push(row);
        self.marked_positions.push([false; SIZE])
    }

    fn mark_number(&mut self, num: usize) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if &num == col {
                    self.marked_positions[y][x] = true
                }
            }
        }
    }

    fn check_for_bingo(&self) -> bool {
        for pos1 in 0..SIZE {
            let mut complete1_2 = true;
            let mut complete2_1 = true;
            for pos2 in 0..SIZE {
                if self.marked_positions[pos1][pos2] == false {
                    complete1_2 = false;
                }
                if self.marked_positions[pos2][pos1] == false {
                    complete2_1 = false;
                }
                if complete1_2 == false && complete2_1 == false {
                    break;
                }
            }
            if complete1_2 == true || complete2_1 == true {
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.marked_positions[y][x] == false {
                    sum += self.grid[y][x];
                }
            }
        }
        sum
    }
}

fn a() {
    let mut lines = include_str!("../inputa.txt").lines();
    let inputs = lines
        .next()
        .expect("Initial line isn't there")
        .split(',')
        .map(|n| n.parse::<usize>().expect("initial line not a num"));

    let mut boards = lines.fold(Vec::<BingoBoard>::new(), |mut boards, line| {
        if line.len() == 0 {
            boards.push(BingoBoard {
                grid: Vec::new(),
                marked_positions: Vec::new(),
            });
            return boards;
        }
        let board = boards.last_mut().unwrap();
        board.new_row(
            line.split(' ')
                .filter(|i| i.trim().len() > 0)
                .map(|n| n.trim().parse::<usize>().expect("Not a number"))
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap(),
        );
        boards
    });

    for number in inputs {
        boards.iter_mut().for_each(|b| b.mark_number(number));
        let mut bingo = boards.iter().find(|b| b.check_for_bingo());
        if bingo.is_some() {
            let board = bingo.take().expect("Board should be some");
            let sum = board.sum_unmarked();
            println!("Result for bingo 1 is {:?}", sum * number);
            return;
        }
    }
}

fn b() {
    let mut lines = include_str!("../inputb.txt").lines();
    let inputs = lines
        .next()
        .expect("Initial line isn't there")
        .split(',')
        .map(|n| n.parse::<usize>().expect("initial line not a num"));

    let mut boards = lines.fold(Vec::<BingoBoard>::new(), |mut boards, line| {
        if line.len() == 0 {
            boards.push(BingoBoard {
                grid: Vec::new(),
                marked_positions: Vec::new(),
            });
            return boards;
        }
        let board = boards.last_mut().unwrap();
        board.new_row(
            line.split(' ')
                .filter(|i| i.trim().len() > 0)
                .map(|n| n.trim().parse::<usize>().expect("Not a number"))
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap(),
        );
        boards
    });

    for number in inputs {
        let mut non_bingo_boards: Vec<&mut BingoBoard> =
            boards.iter_mut().filter(|b| !b.check_for_bingo()).collect();

        if non_bingo_boards.len() == 1 {
            let only_board_left = non_bingo_boards.get_mut(0).expect("len is 1");
            only_board_left.mark_number(number);

            if only_board_left.check_for_bingo() {
                let sum = only_board_left.sum_unmarked();
                println!("Result for bingo 2 is {:?}", sum * number);
                return;
            }
        }
        non_bingo_boards
            .iter_mut()
            .for_each(|b| b.mark_number(number));
    }
}
