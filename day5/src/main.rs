struct Line {
    start: (usize, usize),
    current_point: (usize, usize),
    end: (usize, usize),
}

const SIZE: usize = 1000;

impl Line {
    fn new(line: &str) -> Self {
        let mut split = line.split(" -> ");
        let mut start = split.next().expect("No start found").split(",").map(|n| {
            let num = n.parse::<usize>().expect("Should be a number");
            if num >= SIZE {
                panic!("Input too big");
            }
            num
        });
        let mut end = split.next().expect("No end found").split(",").map(|n| {
            let num = n.parse::<usize>().expect("Should be a number");
            if num >= SIZE {
                panic!("Input too big");
            }
            num
        });

        Self {
            start: (
                start.next().expect("x1 not there"),
                start.next().expect("y1 not there"),
            ),
            current_point: (SIZE, SIZE),
            end: (
                end.next().expect("x2 not there"),
                end.next().expect("y2 not there"),
            ),
        }
    }

    fn is_straight(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
}

impl Iterator for Line {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point == (SIZE, SIZE) {
            self.current_point = self.start;
            return Some(self.start);
        }
        if self.current_point == self.end {
            return None;
        }
        if self.current_point.0 < self.end.0 {
            self.current_point.0 += 1;
        } else if self.current_point.0 > self.end.0 {
            self.current_point.0 -= 1;
        }
        if self.current_point.1 < self.end.1 {
            self.current_point.1 += 1;
        } else if self.current_point.1 > self.end.1 {
            self.current_point.1 -= 1;
        }
        Some(self.current_point)
    }
}
fn main() {
    a();
}

fn a() {
    let lines = include_str!("../inputa.txt")
        .lines()
        .map(|line| Line::new(line))
        .filter(|l| l.is_straight());
    let mut grid = [[0; SIZE]; SIZE];
    lines.for_each(|line| {
        line.for_each(|p| {
            grid[p.1][p.0] += 1;
        })
    });
    let count: usize = grid.iter().fold(0, |acc, row| {
        row.iter()
            .fold(0, |acc, c| if c > &1 { acc + 1 } else { acc })
            + acc
    });
    println!("The num of danger zones is {}", count);
}
