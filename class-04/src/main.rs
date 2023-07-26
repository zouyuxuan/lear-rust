use std::vec;
mod mession_05;
use crate::mession_05::mession_05 as mession;

trait MyTrait {
    fn my_method(&self);
}

struct Type1(u32);
struct Type2(String);
struct Type3(bool);

// impl 实现trait
impl MyTrait for Type1 {
    fn my_method(&self) {
        println!("Type1: {}", self.0);
    }
}

impl MyTrait for Type2 {
    fn my_method(&self) {
        println!("Type2: {}", self.0);
    }
}

impl MyTrait for Type3 {
    fn my_method(&self) {
        println!("Type3: {}", self.0);
    }
}

enum MyEnum {
    Type1(u32),
    Type2(String),
    Type3(bool),
}

// 使用if let 来处理只匹配一个模式的值而忽略其他模式的情况
impl MyEnum {
    fn method_type1(&self) {
        if let MyEnum::Type1(value) = self {
            println!("Type1: {}", value);
        }
    }

    fn method_type2(&self) {
        if let MyEnum::Type2(value) = self {
            println!("Type2: {}", value);
        }
    }

    fn method_type3(&self) {
        if let MyEnum::Type3(value) = self {
            println!("Type3: {}", value);
        }
    }
}


fn main() {
    println!("-------04-------");
    //Trait Object的大小在编译时是不确定的,需要使用Box来进行堆分配
    let my_vec: Vec<Box<dyn MyTrait>> = vec![
        Box::new(Type1(123)),
        Box::new(Type2(String::from("Hello"))),
        Box::new(Type3(true)),
    ];
    for item in my_vec {
        item.my_method();
    }
    // 使用枚举不需要动态分配内存
    let my_vec: Vec<MyEnum> = vec![
        MyEnum::Type1(123),
        MyEnum::Type3(true),
        MyEnum::Type2(String::from("Hello")),
    ];

    for item in my_vec {
        item.method_type1();
        item.method_type2();
        item.method_type3();
    }
    println!("-------05-------");
    assert_eq!(
        mession::Point { x: 1, y: 0 } + mession::Point { x: 2, y: 3 },
        mession::Point { x: 3, y: 3 }
    );
    
}
