fn main() {
    a();
    b();
}

fn a() {
    let num_val: usize = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let mut s = l.split(" | ").skip(1);
            s.next().expect("No outputs")
        })
        .map(|output_values| {
            output_values
                .split(" ")
                .filter(|o| match o.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    println!("1, 4, 7, or 8 appear {} times", num_val);
}

fn b() {
    let total: u32 = include_bytes!("../input.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut part = line.splitn(2, |&b| b == b'|');
            let mut input = part.next().unwrap().split(|&b| b == b' ');
            let one = input.clone().find(|d| d.len() == 2).unwrap();
            let four = input.find(|d| d.len() == 4).unwrap();
            part.next()
                .unwrap()
                .split(|&b| b == b' ')
                .skip(1)
                .map(|d| match d.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    len => match (
                        len,
                        d.iter().filter(|&b| one.contains(b)).count(),
                        d.iter().filter(|&b| four.contains(b)).count(),
                    ) {
                        (5, 1, 3) => 5,
                        (5, 2, 3) => 3,
                        (5, _, 2) => 2,
                        (6, 1, _) => 6,
                        (6, _, 3) => 0,
                        (6, _, 4) => 9,
                        _ => unreachable!(),
                    },
                })
                .enumerate()
                .fold(0, |acc, (i, n)| acc + n * 10_u32.pow(3 - i as u32))
        })
        .sum();
    println!("The total sum is {}", total);
}
