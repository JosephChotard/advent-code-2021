fn main() {
    a();
}

fn a() {
    let mut nums = include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse::<f64>().expect("input is not num"))
        .collect::<Vec<f64>>();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = {
        let half = nums.len() / 2;
        if nums.len() % 2 == 0 {
            (nums[half - 1] + nums[half]) / 2.0
        } else {
            nums[half]
        }
    };
    let total_fuel: f64 = nums
        .iter()
        .fold(0.0, |accum, num| accum + f64::abs(num - median));
    println!("Total fuel for crabs 1 is {}", total_fuel);
}

fn b() {}
