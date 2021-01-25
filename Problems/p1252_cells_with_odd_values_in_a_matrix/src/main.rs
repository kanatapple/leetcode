struct Solution {}

impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut cells = vec![vec![0; m as usize]; n as usize];

        for index in indices.iter() {
            let (row, column) = (index[0] as usize, index[1] as usize);

            for i in 0..m as usize {
                cells[row][i] += 1;
            }

            for j in 0..n as usize {
                cells[j][column] += 1;
            }
        }

        let mut odds = 0;
        for cell in cells {
            for i in 0..m as usize {
                odds += cell[i] % 2;
            }
        }
        odds
    }
}

fn main() {
    assert_eq!(Solution::odd_cells(2, 3, vec![vec![0,1],vec![1,1]]), 6);
    assert_eq!(Solution::odd_cells(2, 2, vec![vec![1,1],vec![0,0]]), 0);
}
