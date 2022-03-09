/*
 * @lc app=leetcode.cn id=35 lang=rust
 *
 * [35] Search Insert Position
 *
 * https://leetcode-cn.com/problems/search-insert-position/description/
 *
 * algorithms
 * Easy (45.58%)
 * Likes:    1384
 * Dislikes: 0
 * Total Accepted:    686.4K
 * Total Submissions: 1.5M
 * Testcase Example:  '[1,3,5,6]\n5'
 *
 * Given a sorted array of distinct integers and a target value, return the
 * index if the target is found. If not, return the index where it would be if
 * it were inserted in order.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,3,5,6], target = 5
 * Output: 2
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [1,3,5,6], target = 2
 * Output: 1
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [1,3,5,6], target = 7
 * Output: 4
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -10^4 <= nums[i] <= 10^4
 * nums contains distinct values sorted in ascending order.
 * -10^4 <= target <= 10^4
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;
        let mut mid: i32 = 0;

        while left <= right {
            mid = left + ((right - left) >> 1);
            if nums[mid as usize] == target {
                return mid as i32;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        if right < mid {
            return mid;
        }
        mid + 1
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input: Vec<i32>,
        target: i32,
        output: i32,
    }

    #[test]
    fn test_search_insert() {
        let test_cases = vec![
            TestCase {
                input: vec![1, 3, 5, 6],
                target: 5,
                output: 2,
            },
            TestCase {
                input: vec![1, 3, 5, 6],
                target: 2,
                output: 1,
            },
            TestCase {
                input: vec![1, 3, 5, 6],
                target: 7,
                output: 4,
            },
            TestCase {
                input: vec![1, 3, 5, 6],
                target: 0,
                output: 0,
            },
            TestCase {
                input: vec![2, 5],
                target: 1,
                output: 0,
            },
            TestCase {
                input: vec![1],
                target: 0,
                output: 0,
            },
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::search_insert(test_case.input, test_case.target),
                test_case.output
            );
        }
    }
}
