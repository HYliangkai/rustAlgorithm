/** 完成逆波兰式
* 逆波兰式的原理
输入: ["2", "1", "+", "3", " * "]
输出: 9
解释: 该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9
求值过程 2 ... 1..   + (2+1)==3 ... 3 ... * (3*3)==9
 */

pub fn do_one(inp: Vec<String>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    inp.into_iter().for_each(|item| {
        if let Ok(now) = item.parse::<f64>() {
            stack.push(now);
        } else {
            let one = stack.pop().expect("格式错误");
            let two = stack.pop().expect("格式错误");
            match item.as_str() {
                "+" => { stack.push(one + two) }
                "-" => { stack.push(two - one) }
                "*" => { stack.push(two * one) }
                "/" => { stack.push(two / one) }
                _ => panic!("输入内容不合法"),
            }
        }
    });
    return stack.pop().unwrap();
}

#[test]
fn testing() {
    let res = do_one(
        vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]
    );
    println!("{:?}", res);
}
