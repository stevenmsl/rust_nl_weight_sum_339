#[derive(Debug, PartialEq)]

pub enum NestedInteger {
    List(Vec<NestedInteger>),
    Integer(i32),
}

pub struct Solution {}

impl Solution {
    pub fn depth_sum(list: Vec<NestedInteger>) -> i32 {
        Self::depth_sum_internal(&list, 1)
    }

    fn depth_sum_internal(list: &Vec<NestedInteger>, level: usize) -> i32 {
        let mut sum: i32 = 0;

        for elment in list.into_iter() {
            match elment {
                NestedInteger::List(some_list) => {
                    sum += Self::depth_sum_internal(some_list, level + 1)
                }
                NestedInteger::Integer(some_integer) => {
                    sum += *some_integer * level as i32;
                }
            }
        }

        sum
    }

    pub fn text_fixture_1() -> Vec<NestedInteger> {
        //[[1,1],2,[1,1]]
        vec![
            NestedInteger::List(vec![NestedInteger::Integer(1), NestedInteger::Integer(1)]),
            NestedInteger::Integer(2),
            NestedInteger::List(vec![NestedInteger::Integer(1), NestedInteger::Integer(1)]),
        ]
    }

    pub fn text_fixture_2() -> Vec<NestedInteger> {
        //[1,[4,[6]]]
        vec![
            NestedInteger::Integer(1),
            NestedInteger::List(vec![
                NestedInteger::Integer(4),
                NestedInteger::List(vec![NestedInteger::Integer(6)]),
            ]),
        ]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let result = Solution::depth_sum(Solution::text_fixture_1());
        assert_eq!(result, 10);
    }

    #[test]
    fn sample_2() {
        let result = Solution::depth_sum(Solution::text_fixture_2());
        assert_eq!(result, 27);
    }
}
