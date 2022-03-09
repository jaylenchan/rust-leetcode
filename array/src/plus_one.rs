/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] Plus One
 *
 * https://leetcode-cn.com/problems/plus-one/description/
 *
 * algorithms
 * Easy (46.10%)
 * Likes:    936
 * Dislikes: 0
 * Total Accepted:    449.1K
 * Total Submissions: 974.2K
 * Testcase Example:  '[1,2,3]'
 *
 * You are given a large integer represented as an integer array digits, where
 * each digits[i] is the i^th digit of the integer. The digits are ordered from
 * most significant to least significant in left-to-right order. The large
 * integer does not contain any leading 0's.
 *
 * Increment the large integer by one and return the resulting array of
 * digits.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 * Incrementing by one gives 123 + 1 = 124.
 * Thus, the result should be [1,2,4].
 *
 *
 * Example 2:
 *
 *
 * Input: digits = [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 * Incrementing by one gives 4321 + 1 = 4322.
 * Thus, the result should be [4,3,2,2].
 *
 *
 * Example 3:
 *
 *
 * Input: digits = [9]
 * Output: [1,0]
 * Explanation: The array represents the integer 9.
 * Incrementing by one gives 9 + 1 = 10.
 * Thus, the result should be [1,0].
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= digits.length <= 100
 * 0 <= digits[i] <= 9
 * digits does not contain any leading 0's.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut i = digits.len() as i32 - 1;
        while i >= 0 {
            if digits[i as usize] != 9 {
                digits[i as usize] = digits[i as usize] + carry;
                carry = 0;
                break;
            } else {
                digits[i as usize] = 0;
                carry = 1;
            }
            i -= 1;
        }
        if carry == 1 {
            digits.insert(0, carry);
        }
        digits
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        output: Vec<i32>,
    }

    #[test]
    fn test_plus_one() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 2, 3],
                output: vec![1, 2, 4],
            },
            TestCase {
                input: vec![4, 3, 2, 1],
                output: vec![4, 3, 2, 2],
            },
            TestCase {
                input: vec![9],
                output: vec![1, 0],
            },
        ];

        for test_case in test_cases {
            assert_eq!(Solution::plus_one(test_case.input), test_case.output);
        }
    }
}
