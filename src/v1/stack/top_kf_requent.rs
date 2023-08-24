use std::{ collections::{ HashMap, BinaryHeap }, cmp::Reverse };

/** 
给定一个非空的整数数组，返回其中出现频率前 k 高的元素。  
你可以假设给定的 k 总是合理的，且 1 ≤ k ≤ 数组中不相同的元素的个数。  
你的算法的时间复杂度必须优于 $O(n \log n)$ , n 是数组的大小。  
题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的。  
你可以按任意顺序返回答案。  
 */

/**
示例 1:  
输入: nums = [1,1,1,2,2,3], k = 2  
输出: [1,2]  
 */

fn do_one(inp: Vec<i32>, k: usize) -> Vec<i32> {
    /* 第一步,采用hashmap记录各个元素的个数情况 */

    /* 第二步是排序问题,如何根据hashmap来进行排序 */
    /* 答案是采用小顶堆  : 
    堆是一棵完全二叉树，树中每个结点的值都不小于（或不大于）其左右孩子的值。
    1. 如果父亲结点是大于等于左右孩子就是大顶堆
    2. 小于等于左右孩子就是小顶堆。 
    大顶堆（堆头是最大元素），小顶堆（堆头是最小元素） */
    /* 具体实现: 
    定义一个从小(头)到大(尾)的队列,长度为k,每次比较的数据都和最小的数据进行比较,如果比最小大就抛弃最小然后进行重新排序 */

    let mut map: HashMap<i32, u32> = HashMap::new();
    inp.iter().for_each(|item| {
        //简短一句话解决 : 获取值并且赋值
        //HashMap::entry()函数的目的是允许您以一种更高效的方式操作哈希映射。它允许您在不重复计算哈希值的情况下同时访问和修改哈希映射的条目。
        *map.entry(*item).or_insert(0) += 1;
    });
    //使用rust自带的小顶堆来实现
    let mut heap = BinaryHeap::with_capacity(k);
    map.iter().for_each(|(key, val)| {
        //满容量
        if heap.len() == heap.capacity() {
            //peek返回的是最值(只是取得,没有出栈操作)
            if *heap.peek().unwrap() <= (Reverse(*key), *val) {
                return;
            } else {
                //出最小值,进当前值
                heap.pop();
            }
        }
        heap.push((Reverse(*key), *val));
    });

    let res: Vec<i32> = heap
        .iter()
        .map(|(Reverse(le), _val)| {
            return *le;
        })
        .collect();
    return res;
}

#[test]
fn testing() {
    // ? 有问题
    println!("{:?}", do_one(vec![1, 1, 1, 2, 2, 3], 2))
}
