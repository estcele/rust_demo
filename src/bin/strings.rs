fn main() {
    //创建String的三种方式
    let mut s1 = String::new();
    let s2 = String::from("String from");
    let s3 = "String from slice".to_string();

    //更新String
    s1.push('a');
    s1.push_str("bc");
    println!("s1: {}", s1);

    //连接String
    let s4 = s2 + &s3;
    println!("s4: {}", s4);
    // println!("s2: {}", s2); // error: value borrowed here after move
    println!("s3: {}", s3);

    //格式化String
    let s5 = format!("{}-{}-{}", s1, s4, s3);
    println!("s5: {}", s5);

    //索引String
    let s6 = "hello".to_string();
    // let s7 = s6[0]; // error: the trait `Index<_>` is not implemented for `String`
    let s7 = &s6[0..1];//返回的是字节，英文字符占一个字节，中文字符占三个字节
    println!("{} bytes={} chars={} s7: {}",s6,s6.bytes().len(),s6.chars().count(), s7);

    let s8 = "你好".to_string();
    let s9 = &s8[0..3];//返回的是字节，英文字符占一个字节，中文字符占三个字节
    println!("{} bytes={} chars={} s9: {}",s8,s8.bytes().len(),s8.chars().count(), s9);

    //遍历String
    for b in s6.bytes() {
        println!("byte: {}", b);
        
    }
    for c in s6.chars() {
        println!("char: {}", c);
    }

    for b in s8.bytes() {
        println!("byte: {}", b);
    }
    for c in s8.chars() {
        println!("char: {}", c);
    }

    //字符串切片
    let s10 = "hello world";
    let s11 = &s10[0..5];
    let s12 = &s10[6..11];
    println!("s11: {}, s12: {}", s11, s12);

    //遍历字符串切片
    for c in s10.chars() {
        println!("char: {}", c);
    }

    //转换String为&str
    let s13 = "hello".to_string();
    let s14: &str = &s13;
    println!("s14: {}", s14);

    //转换&str为String
    let s15 = "hello";
    let s16 = s15.to_string();
    println!("s16: {}", s16);
}