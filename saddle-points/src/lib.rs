pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let rows = input.len();
    let cols = input[0].len();

    let mut candidates: Vec<(usize, usize)> = vec![];

    for i in 0..rows {
        for j in 0..cols {
            if candidates.len() == 0 {
                candidates.push((i, j));
            } else {
                let last_candidate = *candidates.last().unwrap();
                if last_candidate.0 == i {
                    if input[i][j] >= input[i][last_candidate.1] {
                        if input[i][j] > input[i][last_candidate.1] {
                            candidates.pop();
                        }
                        candidates.push((i, j));
                    }
                } else {
                    candidates.push((i, j));
                }
            }
        }
    }

    candidates
        .into_iter()
        .filter(|c| input.iter().all(|row| input[c.0][c.1] <= row[c.1]))
        .collect()
}
