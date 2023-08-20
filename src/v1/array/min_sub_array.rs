/**
给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的 连续 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。
 */

/**
示例：
输入：s = 7, nums = [2,3,1,2,4,3]
输出：2
解释：子数组 [4,3] 是该条件下的长度最小的子数组。
 */

fn do_one(max: i32, nums: Vec<i32>) -> usize {
    /* 解决思路还是双指针 */
    let [mut low, mut fast, mut length] = [0, 0, 0];
    while low < nums.len() - 1 {
        let val = nums[low..fast].iter().fold(0, |item, all| item + all);
        if val >= max {
            if fast - low < length || length == 0 {
                length = fast - low;
            }
            low += 1;
        } else {
            fast += 1;
        }
    }
    return length;
}

#[test]
fn testing() {
    assert_eq!(do_one(7, vec![2, 3, 1, 2, 4, 3]), 2);
}

fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    /* 优化版
  1. 每次循环不重新计算sum值,而是通过累加/累减的方式获得sum值,使得时间复杂度为O(n)
   */

    let (mut result, mut subLength): (i32, i32) = (i32::MAX, 0);
    let (mut sum, mut i) = (0, 0);

    for (pos, val) in nums.iter().enumerate() {
        sum += val;
        while sum >= target {
            subLength = (pos - i + 1) as i32;
            if result > subLength {
                result = subLength;
            }
            sum -= nums[i];
            i += 1;
        }
    }
    if result == i32::MAX {
        return 0;
    }
    result
}
