// static
static MY_STATIC:i32 = 12;
static mut MY_MUT_STATIC: i32 = 95;
# 
# fn main(){
    // const

    const SECOND_HOUR: usize = 3_600;
    const SECOND_DAY: usize = SECOND_HOUR * 24;

    println!("SECONDE_DAY: {SECOND_DAY}");

    {
        const _SE: usize = 1_000;
    }
    // println!("SE: {SE}"); not found in this scope

    println!("MY_STATIC {MY_STATIC}");
    // println!("MY_MUT_STATIC {MY_MUT_STATIC}");   error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    unsafe{
        MY_MUT_STATIC = 33;
        println!("unsafe MY_MUT_STATIC {MY_MUT_STATIC}");
    }
# }