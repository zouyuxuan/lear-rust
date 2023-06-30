// 模块使用pub修饰表示公共模块
// 方法使用pub修饰表示公共方法
pub mod print_str{
    pub fn print(asic:u8){
        println!("{}",char::from(asic));
    }
}