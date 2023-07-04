mod print; //告诉 Rust 从另一个和模块 print_str 同名的文件中加载该模块的内容
pub use crate::print::print_str; // 使用绝对路径加载mod PrintStr
fn main() {
    // use PrintStr;
    println!("-----------");
    for  i in 91..97{
        println!("{}",char::from(i));
    }
    println!("-----------");
    for  i in 65..122{
        print_str::print(i);
    }
    println!("------------");
}
