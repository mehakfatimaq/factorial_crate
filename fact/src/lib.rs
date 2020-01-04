pub fn factorial_func (mut num:i32) {
    if num == 0 {
    println!("1!")
    }
    else {
        for i in (1..num).rev() {
            num = num*i;
            
        }
        println!("{}",num);
             }
}