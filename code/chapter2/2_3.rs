# fn main(){
    // 进制的字面量
    let a1 = -124;
    let a2 = 0xFF;
    let a3 = 0o34;
    let a4 = 0b10;

    println!("a1: {a1}");
    println!("a2: {a2}");
    println!("a3: {a3}");
    println!("a4: {a4}");

    // MAX MIN
    println!("u32 MAX: {}",u32::MAX);
    println!("u32 MIN: {}",u32::MIN);
    println!("i32 MAX: {}",i32::MAX);
    println!("i32 MIN: {}",i32::MIN);
    println!("usize MAX: {}",usize::MAX);
    println!("usize MIN: {}",usize::MIN);
    // byte
    println!("isize is {} byte", std::mem::size_of::<isize>());
    println!("diff way isize is {} byte", isize::BITS / 8);
    println!("usize is {} byte", std::mem::size_of::<usize>());
    println!("u64 is {} byte", std::mem::size_of::<u64>());
    println!("i32 is {} byte", std::mem::size_of::<i32>());
    println!("f64 is {} byte", std::mem::size_of::<f64>());

    // float
    let f1: f32 = 1.23434;
    let f2: f64 = 45.23234;
    println!("Float are: {:.2} {:.3}", f1, f2);

    // bool
    let is_ok = true;
    let can_ok = false;
    println!("is_ok? {is_ok}; can_ok? {can_ok}");
    println!(
        "is_ok or can_ok ? {}; is_ok and can_ok? {}",
        is_ok || can_ok,
        is_ok && can_ok
    );
    
    // char
    let char_c = 'C';
    let emo_char = '😊';
    println!("You get {char_c} feel {emo_char}");
# }
