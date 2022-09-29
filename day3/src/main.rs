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

const INPUT_SIZE: usize = 12;
fn b() {
    let inputb: Vec<[u32; INPUT_SIZE]> = include_str!("../inputb.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(2).expect("Input contains a non digit"))
                .collect::<Vec<u32>>()
                .try_into()
                .expect("input wrong size")
        })
        .collect();

    let ox_gen_rating = {
        let mut indexes: Vec<usize> = (0..inputb.len()).collect();
        let mut bit_pos = 0;
        while indexes.len() > 1 {
            // Get most common value at bit_pos
            let num_of_one = indexes
                .iter()
                .filter(|index| inputb[**index][bit_pos] == 1)
                .count();
            let value_to_filter = if num_of_one as f32 >= (indexes.len() as f32 / 2.0) {
                1
            } else {
                0
            };
            indexes = indexes
                .into_iter()
                .filter(|i| inputb[*i][bit_pos] == value_to_filter)
                .collect();
            bit_pos += 1;
        }
        // Convert last input to number
        inputb[indexes[0]].iter().fold(0, |a, b| 2 * a + b)
    };

    let co2_scrub_rating = {
        let mut indexes: Vec<usize> = (0..inputb.len()).collect();
        let mut bit_pos = 0;
        while indexes.len() > 1 {
            // Get most common value at bit_pos
            let num_of_one = indexes
                .iter()
                .filter(|index| inputb[**index][bit_pos] == 1)
                .count();
            let value_to_filter = if num_of_one as f32 >= (indexes.len() as f32 / 2.0) {
                0
            } else {
                1
            };
            indexes = indexes
                .into_iter()
                .filter(|i| inputb[*i][bit_pos] == value_to_filter)
                .collect();
            bit_pos += 1;
        }
        // Convert last input to number
        inputb[indexes[0]].iter().fold(0, |a, b| 2 * a + b)
    };

    println!(
        "ox rating is {:?}, co2 srub rating is {:?}",
        ox_gen_rating, co2_scrub_rating
    );
    println!(
        "Life support rating is {:?}",
        ox_gen_rating * co2_scrub_rating
    );
}
