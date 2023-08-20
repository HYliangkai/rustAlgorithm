/** # 移除元素
给你一个数组 nums 和一个值 val，你需要 原地 移除所有数值等于 val 的元素，并返回移除后数组的新长度。  
不要使用额外的数组空间，你必须仅使用 O(1) 额外空间并原地修改输入数组。  
元素的顺序可以改变。你不需要考虑数组中超出新长度后面的元素。  

## 解决思路 -- 双指针法
双指针法在解决 **有序数据的删除** 问题上是最有效果的,其算法思想是使用两个指针,其中一个为快指针,用于探测情做判断;另一个为慢指针.用于保存当前数据的实际长度,作为保守指针 
 */

/**
### 示例 1
给定 nums = [3,2,2,3], val = 3, 函数应该返回新的长度 2, 并且 nums 中的前两个元素均为 2。 
你不需要考虑数组中超出新长度后面的元素。
### 示例 2: 
给定 nums = [0,1,2,2,3,0,4,2], val = 2, 函数应该返回新的长度 5, 并且 nums 中的前五个元素为 0, 1, 3, 0, 4

 */
pub fn one(nums: &mut Vec<i32>, val: i32) -> usize {
    /* 如何使用rust表示双指针 : 直接遍历nums取下标即可 */
    let mut fast = 0;
    let mut target = 0;
    for index in 0..nums.len() {
        if fast == nums.len() - 1 {
            target = index;
            break;
        }
        if nums[index].eq(&val) {
            fast += 1;
        }
        nums[index] = nums[fast];
        fast += 1;
    }
    let ret = &nums[..target];
    return target;
}

#[test]
fn test_one() {
    assert_eq!(one(&mut vec![3, 2, 2, 3], 3), 2);
    assert_eq!(one(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
}

/** ## 优化版 */
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    /* 优化版的逻辑是将index作为fast-point 因为fast-point 必须要遍历,所以用index做是最好的 */
    let mut slow_idx = 0;
    for pos in 0..nums.len() {
        if nums[pos] != val {
            nums[slow_idx] = nums[pos];
            slow_idx += 1;
        }
    }
    return slow_idx as i32;
}
