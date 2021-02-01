struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut seconds = 0;
        let mut current = vec![points[0][0], points[0][1]];

        let mut index = 0;
        loop {
            if points.get(index + 1) == None {
                break;
            }

            loop {
                if current == points[index + 1] {
                    break;
                }

                current[0] += (points[index + 1][0] - current[0]).signum();
                current[1] += (points[index + 1][1] - current[1]).signum();

                seconds += 1;
            }

            index += 1;
        }

        seconds
    }
}

fn main() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}
