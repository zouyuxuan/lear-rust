use std::vec;



fn main() {
//     #[derive(Debug)]
//     enum Book{
//         Price(f32),
//         Name(String),
//         UseAge(u32),
//     }
//    let price = Book::Price(10.5);
//    let name = Book::Name(String::from("test"));
//    let use_age = Book::UseAge(15);

   
//    let mut books : Vec<Book> = Vec::new();
//    books.push(price);
//    books.push(name);
//    books.push(use_age);
   
//    for book in &books{
//     println!("{:?}",book)
//    }

    pub trait Book {
        fn info(&self)->String;
    }

    struct Rust{
        name :String,
        price:f32,
    }

    struct Python{
        name :String,
    }
    impl Book for Python {
        fn info(&self)->String{
            return self.name.clone();
        } 
    }
    impl Book for   Rust  {
        fn info(&self)->String{
            return self.name.clone();
        }
        
    }
    impl  Rust {
        fn get_price(&self)->f32{
            return self.price;
        }
    }
    let mut books:Vec<Box<dyn Book>> = Vec::new(); 
    

    let rust:Rust = Rust { name: (String::from("rust")), price: (10.1) };
    let python:Python = Python { name: (String::from("python")) };
    books.push(Box::new(rust));
    books.push(Box::new(python));

    // let info = rust.info();
    // let price :f32 = rust.get_price();
    // println!("info  = {:?}",info);
    println!("info  = {:?}",books[0].info());
    println!("info  = {:?}",books[1].info());

    
   
}
