pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {

    let mut result = Vec::new();

    // it took a bit of thinking to understand the following
    for (row_num, row) in input.iter().enumerate() {
        for (col_num, item) in row.iter().enumerate() {
            if row.iter().all(|x| x <= item) && (0..(input.len())).all(|x| input[x][col_num] >= *item) {
                result.push((row_num, col_num));
            }
        }
    }

    result
}
