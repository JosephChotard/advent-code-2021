fn main() {
    a();
    b();
}

fn a() {
    let mut depths = include_str!("../inputa.txt")
        .lines()
        .map(|n| n.parse::<usize>().expect("Not a number"));

    let mut num_increases: usize = 0;
    let mut previous_depth = depths.next().expect("No initial value");

    for depth in depths {
        if depth > previous_depth {
            num_increases += 1;
        }
        previous_depth = depth;
    }

    println!("Num of depth increases {:?}", num_increases);
}

fn b() {
    let count = include_str!("../inputb.txt")
        .lines()
        .map(|n| n.parse::<usize>().expect("Not a number"))
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    println!("Num of depth increases for 2 {:?}", count);
}
