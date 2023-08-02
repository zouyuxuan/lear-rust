
//#[macro_export] 注解表明只要导入了定义这个宏的 crate，该宏就应该是可用的。
#[macro_export]
macro_rules! macro_vec{
    // 我们使用美元符号（$）在宏系统中声明一个变量来包含匹配该模式的 Rust 代码。
    //美元符号明确表明这是一个宏变量而不是普通 Rust 变量。之后是一对括号，其捕
    //获了符合括号内模式的值用以在替代代码中使用

    //$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式命名为 $x

    // $() 之后的逗号说明一个可有可无的;分隔符可以出现在 $() 所匹配的代码之后,
    // 使用；描述宏参数分隔符

    // 紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式
    /*
    ?：表示最多一次重复，所以此时不能前跟分隔标记。
    *：表示零次或多次重复。
    +：表示一次或多次重复。
    */
    ($($x:expr);*)=>{
        // 在这个块内用大括号括起来，然后在里面写多条语句
        {
            let mut temp_vec = Vec::new();
            // 开始反复捕获
            $(
                // 每个反复会展开成下面表达式，其中 $x 被换成相应被捕获的表达式
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/*

expr  捕获方式
捕获方式又被称作“片段分类符” (fragment-specifier)，必须是以下一种：

block：一个块（比如一块语句或者由大括号包围的一个表达式）
expr：一个表达式 (expression)
ident：一个标识符 (identifier)，包括关键字 (keywords)
item：一个条目（比如函数、结构体、模块、impl 块）
lifetime：一个生命周期注解（比如 'foo、'static）
literal：一个字面值（比如 "Hello World!"、3.14、'🦀'）
meta：一个元信息（比如 #[...] 和 #![...] 属性内部的东西）
pat：一个模式 (pattern)
path：一条路径（比如 foo、::std::mem::replace、transmute::<_, int>）
stmt：一条语句 (statement)
tt：单棵标记树
ty：一个类型
vis：一个可能为空的可视标识符（比如 pub、pub(in crate)）
*/

fn main() {
    let n = macro_vec!(1;2;3);
    println!("{}",n[0]);
}
