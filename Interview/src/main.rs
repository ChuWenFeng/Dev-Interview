use rand::Rng;

/*
    测试gen_range()函数生成的随机数的均匀性
    gen_range rand num:10000 
    expect:55000
    sum:55243
*/
fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();

    let num = 10000;
    let mut  sum = 0;
    for _ in 0..num{
        sum += rng.gen_range(1..11);
    }

    let expect = (55*num)/10;

    println!("gen_range rand num:{} \nexpect:{} \nsum:{}",num,expect,sum);

}
