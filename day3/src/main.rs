fn main() {
    a();
    b();
}

fn a() {
    // Last number in count is the number of lines
    let count = include_str!("../inputa.txt").lines().fold(
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        |mut count, line| {
            if line.len() + 1 != count.len() {
                panic!("Line is too long");
            }
            line.chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    count[i] += 1;
                }
            });
            count[count.len() - 1] += 1;
            count
        },
    );
    // Truncating to int is fine
    let threshold = count[count.len() - 1] / 2;
    // Remove the last digit from count before map (it's the number of rows)
    let gamma_rate_bin = count[..count.len() - 1]
        .iter()
        .map(|n| if n > &threshold { 1 } else { 0 })
        .collect::<Vec<i32>>();
    let epsilon_rate_bin = gamma_rate_bin.iter().map(|n| -n + 1);
    let gamma_rate = gamma_rate_bin.iter().fold(0, |a, b| {
        return 2 * a + b;
    });
    let epsilon_rate = epsilon_rate_bin.fold(0, |a, b| 2 * a + b);
    println!("Answer to day 3 part 1 is {:?}", gamma_rate * epsilon_rate);
}

fn b() {}
