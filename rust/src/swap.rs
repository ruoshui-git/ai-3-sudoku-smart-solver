use std::collections::HashMap;

use crate::board::positions::{HOUSES, House};

use super::filledboard::FilledBoard;

impl FilledBoard {
    pub fn check_swap(&self) -> (usize, usize) {
        let bad_rules: Vec<_> = HOUSES
            .iter()
            .filter(|rule| !self.is_valid_on_rule(rule))
            .collect();
        match bad_rules.len() {
            6 | 4 => {
                // find the most common indeces in the list of invalid rules, which are the culprit
                let indicies: Vec<_> = bad_rules
                    .iter()
                    .map(|rule| self.get_conflict(rule))
                    .flatten()
                    .collect();

                let mut freq = HashMap::new();
                for index in indicies {
                    *freq.entry(index).or_insert(0) += 1;
                }

                let mut freq_vec: Vec<_> = freq.iter().collect();
                freq_vec.sort_unstable_by_key(|x| x.1);

                // pop the last two and return indices (most common two)
                let mut ans = [*freq_vec.pop().unwrap().0, *freq_vec.pop().unwrap().0];
                ans.sort();
                return (ans[0], ans[1]);
            }
            2 => {
                let indicies: Vec<_> = bad_rules
                    .iter()
                    .map(|rule| self.get_conflict(rule))
                    .collect();

                assert_eq!(2, indicies.len());

                let pair0 = &indicies[0];
                let pair1 = &indicies[1];

                for i0 in pair0 {
                    for i1 in pair1 {
                        // swap indices in board and check validity
                        if self.swap_pair(i0, i1).is_solved() {
                            let mut ans = [*i0, *i1];
                            ans.sort();
                            return (ans[0], ans[1]);
                        }
                    }
                }

                panic!("No valid solution!");
            }
            _ => {
                panic!("Swapped board should only have 2, 4, or 6 conflicting values");
            }
        }
    }

    fn get_conflict(&self, rule: &&House) -> Vec<usize> {
        let mut val_pos = vec![];
        for pos in rule.iter().cloned() {
            let val = self.data[pos];
            for (stored_val, stored_pos) in &val_pos {
                if stored_val == &val {
                    return vec![*stored_pos, pos];
                }
            }
            val_pos.push((val, pos));
        }
        panic!("Must be a conflict!");
    }

    fn swap_pair(&self, i0: &usize, i1: &usize) -> FilledBoard {
        let mut board = self.clone();
        let (i0, i1) = (i0.clone(), i1.clone());
        let (v0, v1) = (board.data[i0], board.data[i1]);
        board.data[i0] = v1;
        board.data[i1] = v0;
        board
    }
}

#[cfg(test)]
mod tests {
    use crate::filledboard::FilledBoard;

    const SWAPPED_BOARD0: FilledBoard = FilledBoard {
        #[rustfmt::skip]
        data: [
            6,9,1,4,8,3,5,2,7,
            2,1,3,7,9,5,4,6,8,
            5,8,7,6,2,4,9,3,1,
            3,5,8,2,7,1,6,9,4,
            1,2,6,4,3,9,8,7,5,
            7,4,9,8,5,6,2,1,3,
            4,7,2,9,1,8,3,5,6,
            8,3,1,5,6,2,7,4,9,
            9,6,5,3,4,7,1,8,2,
        ],
    };

    const SWAPPED_BOARD1: FilledBoard = FilledBoard {
        #[rustfmt::skip]
        data: [
            3,5,6,2,9,4,8,7,1,
            1,7,8,6,5,3,4,9,2,
            4,2,9,7,8,1,3,6,5,
            8,1,2,5,4,9,6,3,8,
            6,9,5,1,3,7,2,4,7,
            7,3,4,8,6,2,1,5,9,
            2,6,3,9,7,8,5,1,4,
            9,4,1,3,2,5,7,8,6,
            5,8,7,4,1,6,9,2,3,
        ],
    };

    #[test]
    fn test_swap_pair() {
        let swapped_b0_0_17 = FilledBoard {
            #[rustfmt::skip]
            data: [
                8,9,1,4,8,3,5,2,7,
                2,1,3,7,9,5,4,6,6,
                5,8,7,6,2,4,9,3,1,
                3,5,8,2,7,1,6,9,4,
                1,2,6,4,3,9,8,7,5,
                7,4,9,8,5,6,2,1,3,
                4,7,2,9,1,8,3,5,6,
                8,3,1,5,6,2,7,4,9,
                9,6,5,3,4,7,1,8,2,
            ],
        };
        assert_eq!(swapped_b0_0_17, SWAPPED_BOARD0.swap_pair(&0, &17));
    }

    #[test]
    fn test_solution() {
        assert_eq!((35, 44), SWAPPED_BOARD1.check_swap());
        assert_eq!((2, 3), SWAPPED_BOARD0.check_swap());
    }
}
