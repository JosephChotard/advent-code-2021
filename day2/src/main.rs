fn main() {
    a();
}

fn a() {
    let new_pos = include_str!("../inputa.txt")
        .lines()
        .fold([0, 0], |[h, d], line| {
            let direction = &line[..line.len() - 2];
            let distance = line[line.len() - 1..]
                .parse::<i32>()
                .expect("This should be a number");
            match direction {
                "forward" => [h + distance, d],
                "down" => [h, d + distance],
                "up" => [h, d - distance],
                _ => panic!("Not a valid direction"),
            }
        });
    println!("Final answer {:?}", new_pos[0] * new_pos[1]);
}
