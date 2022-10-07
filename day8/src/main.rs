fn main() {
    a();
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
