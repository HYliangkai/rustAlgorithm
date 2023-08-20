# 我在Rust写算法

## Version One 《代码随想录》
算法最难的部分在于如何想出来,而不是如何实现.因此,我在学习算法的过程中,会尽量多的去看别人的代码,然后自己去实现.这样的好处是,可以让我更快的学习到算法的思想,而不是去纠结于如何实现.(这和数学很像,但是可以通过大量的练习获得基本的技能,再自己提炼出组合技)

### 双指针法

1. 双指针法在解决 **有序数据的删除** 问题上是最有效果的,其算法思想是使用两个指针,其中一个为快指针,用于探测情做判断;另一个为慢指针.用于保存当前数据的实际长度,作为保守指针

[移除元素的经典做法] 

![27.移除元素-双指针法](https://code-thinking.cdn.bcebos.com/gifs/27.%E7%A7%BB%E9%99%A4%E5%85%83%E7%B4%A0-%E5%8F%8C%E6%8C%87%E9%92%88%E6%B3%95.gif)

2. **有序数据的排序**问题上使用双指针也是有效果的

# Rust妙用

## 元组可以将冗余的一组变量声明合并在一起

使用一个元组能将多个逻辑相同的数据合并在一起使用,rust的元组可以结构重命名(js也可以),所以当出现*固定逻辑共用(或分组)的数据可以使用元组*,这样使得维护多个数据的行为变成维护单个数据里面的多个数据

#### 示例1,逻辑共用

```rust
//保存vec的下标,正常方式
let mut start_index=0;
let mut end_index=10;
let mut now_index=0;
//使用三个变量来表是不同的下标,代码冗余

//使用元组的方式
let (mut start,mut end,mut now)=(0,10,0);
//一行代码吊打前面3行代码,无敌
```

#### 示例2,分组

```rust
//表示table的信息
let table_data=Vec::new();
let table_cloumn=Vec::new();
let table_size=10;
let table_page=1;
||
||
V
let (table_data,table_cloumn,table_siz0e,table_page)=(Vec::new(),Vec::new(),10,1)
```

#### 优点

+ 仅仅只是优化变量声明的过程,并没有带来多一层的数据包装
+ 想到再说

## 没有用到break和for循环的数据可以改写成while循环

+ 改写: `break判断`-->`while判断`

特别是如果for循环只是用到index,要考虑index是否可以做替代

例子

```rust
for index in (0..num.len()){
  if xxx{
    todo();//todo()必然会到达才可使用
    break
  }
}

||
||
V

whlie xxx{
 
}
todo();
```

