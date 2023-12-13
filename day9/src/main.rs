fn main() {
    let parts = include_str!("input.txt").lines().fold([0, 0], |mut sum, line| {
        let nums: Vec<_> = line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect();
        let mut diffs: Vec<_> = vec![nums];
        loop {
            let new_diffs: Vec<_> = diffs[0]
                .iter()
                .zip(diffs[0].iter().skip(1))
                .map(|(n1, n2)| n2 - n1)
                .collect();
            diffs.insert(0, new_diffs);
            let all_zeroes = diffs[0].iter().all(|n| *n == 0);
            if all_zeroes {
                break;
            }
        }
        let last_number: i64 = diffs.iter().map(|v| v.last().unwrap()).sum();
        let first_numbers: Vec<_> = diffs.iter().map(|v| v.first().unwrap()).collect();

        let mut sign = 1;
        let signed_first_numbers: Vec<_> = first_numbers.iter().map(|n|{
            let nn = sign * **n;
            sign = -sign;
            nn
        }).collect();
        dbg!(&signed_first_numbers);

        sum[0] += last_number;
        sum[1] += signed_first_numbers.iter().sum::<i64>();
        sum

    });

    dbg!(parts);
}
