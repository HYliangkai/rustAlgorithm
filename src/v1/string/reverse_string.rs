/**
编写一个函数，其作用是将输入的字符串反转过来。输入字符串以字符数组 char[] 的形式给出。  
不要给另外的数组分配额外的空间，你必须原地修改输入数组、使用 O(1) 的额外空间解决这一问题。  
你可以假设数组中的所有字符都是 ASCII 码表中的可打印字符。  
 */

/**示例 1：
输入：["h","e","l","l","o"]
输出：["o","l","l","e","h"] */
fn do_one(mut inp: Vec<char>) -> Vec<char> {
    //直接双指针覆盖即可
    let mut local = '0';
    let (mut st, mut et) = (0, inp.len() - 1); //注意max-index == len()-1
    while st <= et {
        local = inp[st];
        inp[st] = inp[et];
        inp[et] = local;
        st += 1;
        et -= 1;
    }
    return inp;
}

#[test]
fn testing() {
    println!("{:?}", do_one(vec!['h', 'e', 'l', 'l', 'o']))
}
