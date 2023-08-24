use std::ops::Range;

/**
给定一个正整数 n，生成一个包含 1 到 n^2 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。
 */

/**
输入: 3 输出: [ [ 1, 2, 3 ], [ 8, 9, 4 ], [ 7, 6, 5 ] ]
 */

fn do_one(target: usize) -> Vec<Vec<i32>> {
    //先创建矩阵 使用层叠的创建语法  (注意 `vec![0..N] `和 `vec![0 ; N]`的区别)
    let mut matrix = vec![vec![0;target];target];
    //往矩阵内填充数据即可
    let mut now = 1;

    //旋转一圈,每次都到中位值停止向前,换一个方向向前
    let (mut loop_num, mid) = (target / 2, target / 2); // loop_num为旋转的圈数 , mid为中位index
    let (mut start_x, mut start_y) = (0, 0); //开始时的x,y轴
    let mut offset = 1; // 控制每一层填充元素个数
    while loop_num > 0 {
        //方向为顺时针,旋转4次,分别为上-右-下-左
        let (mut x, mut y) = (start_x, start_y);
        //上,只增加 y
        while y < start_y + target - offset {
            matrix[x][y] = now;
            now += 1;
            y += 1;
        }
        //右,此时 x == target - start_y - offset,为最大index值,开始增加x
        while x < start_x + target - offset {
            matrix[x][y] = now;
            now += 1;
            x += 1;
        }
        //下 ,开始递减y到start_y(不能递减到0,因为有第n层的存在)
        while y > start_y {
            matrix[x][y] = now;
            now += 1;
            y -= 1;
        }
        //左,开始递减x到start_x
        while x > start_x {
            matrix[x][y] = now;
            now += 1;
            x -= 1;
        }
        //转完一圈开始结算
        start_x += 1;
        start_y += 1;
        offset += 2; //每转一圈增加两个padding,so add 2
        loop_num -= 1;
    }
    //边界处理,target为奇数最中点需要手动加上
    if target % 2 == 1 {
        matrix[mid][mid] = target * target;
    }
    println!("{:?}", matrix);
    return vec![Vec::new()];
}

#[test]
fn testing() {
    /*
  [1,2,3,4]
  [18,13,14,5]
  [17,16,15,6]
  [10,9,8,7]
  */
    do_one(4);
}
