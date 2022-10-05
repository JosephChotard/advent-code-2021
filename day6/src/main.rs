fn main() {
    a();
}

const NUM_DAYS: usize = 80;

fn a() {
    // State holds the number of lantern fish per timer position (index is time left)
    let mut state: [usize; 9] = include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse::<usize>().expect("Not a number"))
        .fold([0; 9], |mut acc, n| {
            acc[n] += 1;
            acc
        });

    for _ in 0..NUM_DAYS {
        let num_of_0 = state[0];
        for i in 1..9 {
            state[i - 1] = state[i]
        }
        state[6] += num_of_0;
        state[8] = num_of_0;
    }

    let num_fish: usize = state.iter().sum();
    println!("P1 num of fish after {} days is {}", NUM_DAYS, num_fish);
}
