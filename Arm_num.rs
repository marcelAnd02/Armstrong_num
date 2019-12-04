//Program determines if given integer(u32) is an Armstrong number or not
use std::io;

fn main()
{
    let mut in_num = String::new();
    io::stdin()
        .read_line(&mut in_num)
        .expect("Could not read from standard input");
    let trimmed = in_num.trim();
    match trimmed.parse::<u32>()
    {
        Ok(i) => determine(i),
        Err(..) => println!("not an integer: {}", trimmed),
    };
}

fn determine(a : u32)
{
    let mut n1 = a;
    let mut dig_count = 0;
    let mut prosp_num = 0;
    let mut vec = Vec::new();
    while n1 > 0
    {
        dig_count += 1;
        let curr_dig = n1 % 10;
        vec.push(curr_dig);
        n1 /= 10;
    }
    for nm in vec.iter()
    {
        prosp_num += u32::pow(*nm, dig_count);
    }
        
    if prosp_num == a
    {
        println!("This is an Armstrong number");
    }
        
    else
    {
        println!("Not an Armstrong number");
    }
        
}
