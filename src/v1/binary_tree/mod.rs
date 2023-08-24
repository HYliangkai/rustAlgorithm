use std::{ rc::Rc, cell::RefCell };

/** Rust写二叉树
rust 写二叉树的难度和list是差不多的,由于树节点最有两个共享,而树节点又需要有改变的能力,所以需要用
Rc(多共享) + RefCell(内部可变性) 来包裹
 */

pub struct TreeNode<T> {
    pub val: T, //树值
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    #[inline] //内联展开,如果一个函数调用次序高可以使用对高频函数进行优化,缺点是会增加内存占用
    pub fn new(val: T) -> TreeNode<T> {
        return TreeNode {
            val,
            left: None,
            right: None,
        };
    }
}
