fn main() {
    a();
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
