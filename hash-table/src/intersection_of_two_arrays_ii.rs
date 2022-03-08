/*
 * @lc app=leetcode.cn id=350 lang=rust
 *
 * [350] Intersection of Two Arrays II
 *
 * https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/description/
 *
 * algorithms
 * Easy (55.64%)
 * Likes:    673
 * Dislikes: 0
 * Total Accepted:    314.4K
 * Total Submissions: 564.8K
 * Testcase Example:  '[1,2,2,1]\n[2,2]'
 *
 * Given two integer arrays nums1 and nums2, return an array of their
 * intersection. Each element in the result must appear as many times as it
 * shows in both arrays and you may return the result in any order.
 *
 *
 * Example 1:
 *
 *
 * Input: nums1 = [1,2,2,1], nums2 = [2,2]
 * Output: [2,2]
 *
 *
 * Example 2:
 *
 *
 * Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
 * Output: [4,9]
 * Explanation: [9,4] is also accepted.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums1.length, nums2.length <= 1000
 * 0 <= nums1[i], nums2[i] <= 1000
 *
 *
 *
 * Follow up:
 *
 *
 * What if the given array is already sorted? How would you optimize your
 * algorithm?
 * What if nums1's size is small compared to nums2's size? Which algorithm is
 * better?
 * What if elements of nums2 are stored on disk, and the memory is limited such
 * that you cannot load all elements into the memory at once?
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash_table: HashMap<i32, u32> = HashMap::new();

        let mut ans_vec: Vec<i32> = vec![];

        for num in nums1 {
            let count = match hash_table.get(&num) {
                Some(value) => value + 1,
                None => 1,
            };
            hash_table.insert(num, count);
        }

        for num in nums2 {
            let count = match hash_table.get(&num) {
                Some(value) => *value,
                None => 0,
            };
            if count > 0 {
                ans_vec.push(num);
                hash_table.insert(num, count - 1);
            }
        }
        ans_vec
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    struct Input {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    }

    struct TestCase {
        input: Input,
        output: Vec<i32>,
    }

    #[test]
    fn test_intersect() {
        let test_cases = vec![
            TestCase {
                input: Input {
                    nums1: vec![1, 2, 2, 1],
                    nums2: vec![2, 2],
                },
                output: vec![2, 2],
            },
            TestCase {
                input: Input {
                    nums1: vec![4, 9, 5],
                    nums2: vec![9, 4, 9, 8, 4],
                },
                output: vec![4, 9],
            },
            TestCase {
                input: Input {
                    nums1: vec![3, 3, 2],
                    nums2: vec![2, 1, 1, 3, 4],
                },
                output: vec![2, 3],
            },
        ];

        for test_case in test_cases {
            let mut output = Solution::intersect(test_case.input.nums1, test_case.input.nums2);
            output.sort();
            assert_eq!(output, test_case.output)
        }
    }
}
