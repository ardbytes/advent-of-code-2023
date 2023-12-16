fn mismatch_count(l: &Vec<u8>, r: &Vec<u8>) -> Vec<usize> {
    l.iter()
        .zip(r.iter())
        .map(|(a, b)| if a != b { 1 } else { 0 })
        .collect::<Vec<usize>>()
}

fn find_vertical_line_of_reflection1(block: &Vec<&[u8]>) -> Option<usize> {
    let row_count = block.len();
    let c = block[0].len();
    let line_of_reflections: Vec<_> = (0..=c - 2).map(|i| (i, i + 1)).collect();
    let result: Vec<_> = line_of_reflections
        .iter()
        .filter(|(l, r)| {
            let binding = [(0..=*l).count(), (*r..=c - 1).count()];
            let count = binding.iter().min().unwrap();
            (0..*count).all(|i| {
                let left_column: Vec<_> = (0..row_count).map(|j| block[j][l - i]).collect();
                let right_column: Vec<_> = (0..row_count).map(|j| block[j][r + i]).collect();
                left_column == right_column
            })
        })
        .collect();
    if result.len() > 0 {
        Some(result[0].0 + 1)
    } else {
        None
    }
}

fn find_horizontal_line_of_reflection1(block: &Vec<&[u8]>) -> Option<usize> {
    let r = block.len();
    let c = block[0].len();
    let line_of_reflections: Vec<_> = (0..=r - 2).map(|i| (i, i + 1)).collect();
    let result: Vec<_> = line_of_reflections
        .iter()
        .filter(|(u, d)| {
            let binding = [(0..=*u).count(), (*d..=r - 1).count()];
            let count = binding.iter().min().unwrap();
            (0..*count).all(|i| {
                let top_row: Vec<_> = (0..c).map(|j| block[u - i][j]).collect();
                let bottom_row: Vec<_> = (0..c).map(|j| block[d + i][j]).collect();
                top_row == bottom_row
            })
        })
        .collect();
    if result.len() > 0 {
        Some(result[0].0 + 1)
    } else {
        None
    }
}

fn find_vertical_line_of_reflection2(block: &Vec<&[u8]>) -> Option<usize> {
    let row_count = block.len();
    let c = block[0].len();
    let line_of_reflections: Vec<_> = (0..=c - 2).map(|i| (i, i + 1)).collect();
    let result: Vec<_> = line_of_reflections
        .iter()
        .filter(|(l, r)| {
            let binding = [(0..=*l).count(), (*r..=c - 1).count()];
            let count = binding.iter().min().unwrap();
            let matches: Vec<_> = (0..*count)
                .map(|i| {
                    let left_column: Vec<_> = (0..row_count).map(|j| block[j][l - i]).collect();
                    let right_column: Vec<_> = (0..row_count).map(|j| block[j][r + i]).collect();
                    mismatch_count(&left_column, &right_column)
                })
                .flatten()
                .collect();
            let zeroes = matches.iter().filter(|&&n| n == 0).count();
            let ones = matches.iter().filter(|&&n| n == 1).count();
            zeroes == count * row_count - 1 && ones == 1
        })
        .collect();
    if result.len() > 0 {
        Some(result[0].0 + 1)
    } else {
        None
    }
}

fn find_horizontal_line_of_reflection2(block: &Vec<&[u8]>) -> Option<usize> {
    let r = block.len();
    let c = block[0].len();
    let line_of_reflections: Vec<_> = (0..=r - 2).map(|i| (i, i + 1)).collect();
    let result: Vec<_> = line_of_reflections
        .iter()
        .filter(|(u, d)| {
            let binding = [(0..=*u).count(), (*d..=r - 1).count()];
            let count = binding.iter().min().unwrap();
            let matches: Vec<_> = (0..*count)
                .map(|i| {
                    let top_row: Vec<_> = (0..c).map(|j| block[u - i][j]).collect();
                    let bottom_row: Vec<_> = (0..c).map(|j| block[d + i][j]).collect();
                    mismatch_count(&top_row, &bottom_row)
                })
                .flatten()
                .collect();
            let zeroes = matches.iter().filter(|&&n| n == 0).count();
            let ones = matches.iter().filter(|&&n| n == 1).count();
            zeroes == count * c - 1 && ones == 1
        })
        .collect();
    if result.len() > 0 {
        Some(result[0].0 + 1)
    } else {
        None
    }
}

fn main() {
    let valleys = include_str!("input.txt").split("\n\n").collect::<Vec<_>>();

    let part1 = valleys.iter().fold(0, |mut sum, block_of_mirrors| {
        let block: Vec<_> = block_of_mirrors
            .split("\n")
            .map(|line| line.as_bytes())
            .filter(|line| line.len() > 0)
            .collect();
        let mut result = None;
        if let Some(line_of_vert_reflection) = find_vertical_line_of_reflection1(&block) {
            result = Some(line_of_vert_reflection);
        }

        if result.is_none() {
            if let Some(line_of_hori_reflection) = find_horizontal_line_of_reflection1(&block) {
                result = Some(100 * line_of_hori_reflection);
            }
        }

        sum = sum + result.unwrap();
        sum
    });
    dbg!(part1);

    let part2 = valleys.iter().fold(0, |mut sum, block_of_mirrors| {
        let block: Vec<_> = block_of_mirrors
            .split("\n")
            .map(|line| line.as_bytes())
            .filter(|line| line.len() > 0)
            .collect();
        let mut result = None;
        if let Some(line_of_hori_reflection) = find_horizontal_line_of_reflection2(&block) {
            result = Some(100 * line_of_hori_reflection);
        }

        if result.is_none() {
            if let Some(line_of_vert_reflection) = find_vertical_line_of_reflection2(&block) {
                result = Some(line_of_vert_reflection);
            }
        }

        sum = sum + result.unwrap();
        sum
    });
    dbg!(part2);
}
