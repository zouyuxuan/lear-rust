
#[macro_export]
macro_rules! macro_vec{
    ($($x:expr),*)=>{
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let n = macro_vec!(1,2,3);
    println!("{}",n[0]);
}
