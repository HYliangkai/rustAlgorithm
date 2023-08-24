/**
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。  
有效字符串需满足：
1.左括号必须用相同类型的右括号闭合。
2.左括号必须以正确的顺序闭合。
3.注意空字符串可被认为是有效字符串。
 */
/**
示例 1:
输入: "()"
输出: true
 */

fn do_one(inp: String) -> bool {
    /* 
    1. 如何使用栈来判断有效性
    使用`出栈`和`入栈`两个行为来表示 : '进入匹配' -- '匹配成功' 两个行为,因为刚好匹配符号的顺序和进出栈的顺序是一致的
    2  如何用rust来表示栈 
    使用vec
    */
    let mut stack: Vec<char> = Vec::new();
    let mut flag = true;
    inp.chars()
        .into_iter()
        .for_each(|item: char| {
            match item {
                '{' | '(' | '[' | '<' => { stack.push(item) }
                '}' | ')' | ']' | '>' => {
                    let val = stack.pop().unwrap();
                    match item {
                        '}' => {
                            if val != '{' {
                                flag = false;
                            }
                        }
                        ')' => {
                            if val != '(' {
                                flag = false;
                            }
                        }
                        ']' => {
                            if val != '[' {
                                flag = false;
                            }
                        }
                        '>' => {
                            if val != '<' {
                                flag = false;
                            }
                        }
                        _ => {
                            flag = false;
                        }
                    }
                }
                _ => (),
            }
        });
    return flag;
}

#[test]
fn testing() {
    assert!(do_one("()".to_string()));
    assert!(!do_one("([)]".to_string()));
}
