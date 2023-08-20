/** # 数组平方
给你一个按 非递减顺序 排序的整数数组 nums，返回 每个数字的平方 组成的新数组，要求也按 非递减顺序 排序。
 */

/** ## 示例 1：
输入：nums = [-4,-1,0,3,10]
输出：[0,1,9,16,100]
解释：平方后，数组变为 [16,1,0,9,100]，排序后，数组变为 [0,1,9,16,100]
 */

pub fn do_one(nums: &Vec<i32>) -> Vec<i32> {
    /* 还是考察双指针的用处  
数组其实是有序的， 只不过负数平方之后可能成为最大数了。
那么数组平方的最大值就在数组的两端，不是最左边就是最右边，不可能是中间。  
此时可以考虑双指针法了，i指向起始位置，j指向终止位置。 */
    let mut res: Vec<i32> = vec![0;nums.len()]; //给vec填加东西即可
    let mut start = 0;
    let mut end = nums.len() - 1;
    for index in 0..nums.len() {
        if start == end {
            res[0] = nums[end];
            break;
        } else if nums[end] * nums[end] >= nums[start] * nums[start] {
            //进行比较.谁大谁就push
            res[nums.len() - 1 - index] = nums[end] * nums[end];
            end -= 1;
        } else {
            res[nums.len() - 1 - index] = nums[start] * nums[start];
            start += 1;
        }
    }
    return res;
}

#[test]
fn testing() {
    println!("{:?}", do_one(&vec![-4, -1, 0, 3, 10]))
}

/** 优化版
1. 把多个固定表示数组index的变量使用 解构+元组 的方式合并在一起,即能将逻辑合并在一处又能省内存,!!妙啊
2. 把for循环改成while循环,优点是代码内的index1==index2判断可以直接作为中断依据
 */
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let (mut i, mut j, mut k) = (0, n - 1, n);
    let mut ans = vec![0;n];
    while i <= j {
        if nums[i] * nums[i] < nums[j] * nums[j] {
            ans[k - 1] = nums[j] * nums[j];
            j -= 1;
        } else {
            ans[k - 1] = nums[i] * nums[i];
            i += 1;
        }
        k -= 1;
    }
    ans
}
