// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    #[macro_export] //将宏导出到当前 crate 根作用域
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); //这里不需要加 macros:: 前缀，因#[macro_export]导入到了全局作用域
}
