fn main() {
    a();
    b();
}

fn a() {
    let new_pos = include_str!("../inputa.txt")
        .lines()
        .fold([0, 0], |[h, d], line| {
            let direction = &line[..line.len() - 2];
            let distance = line[line.len() - 1..]
                .parse::<u32>()
                .expect("This should be a number");
            match direction {
                "forward" => [h + distance, d],
                "down" => [h, d + distance],
                "up" => [h, d - distance],
                _ => panic!("Not a valid direction"),
            }
        });
    println!("Final answer of day 2 part 1 {:?}", new_pos[0] * new_pos[1]);
}

fn b() {
    let new_pos = include_str!("../inputb.txt")
        .lines()
        .fold([0, 0, 0], |[h, d, a], line| {
            let direction = &line[..line.len() - 2];
            let distance = line[line.len() - 1..]
                .parse::<u32>()
                .expect("This should be a number");
            match direction {
                "forward" => [h + distance, d + a * distance, a],
                "down" => [h, d, a + distance],
                "up" => [h, d, a - distance],
                _ => panic!("Not a valid direction"),
            }
        });
    println!("Final answer of day 2 part 2 {:?}", new_pos[0] * new_pos[1]);
}
