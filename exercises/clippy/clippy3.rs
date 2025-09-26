// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        //避免 unwrap(None) panic，改为打印提示信息
        println!("Option is none, nothing to unwrap.");
    }

    let my_arr = &[
        -1, -2, -3, //补上逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.resize(0, 5); //resize是就地修改方法，不可用于赋值
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); //变量交换
    println!("value a: {}; value b: {}", value_a, value_b);
}
