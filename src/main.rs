extern crate rand;

use rand::Rng;

fn genchars(len: usize) -> String
{
    let mut rstr = String::new();
    for _ in (0..len) 
    {
        rstr.push((0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char);
    }

    rstr
}

fn genstr(len: usize) -> String 
{
    let rstr = rand::thread_rng()
    .gen_ascii_chars()
    .take(len)
    .collect::<String>();

    rstr
}

fn gennum(len: usize) -> String
{
    let mut rstr = String::new();
    for _ in (0..len) 
    {
        rstr.push_str(&rand::thread_rng().gen_range(0, 9).to_string());
    }

    rstr
}

fn main()
{
    let lenght = 12;
    println!("Generate random passwords!");
    
    println!("Generate random password with strings: {}", genstr(lenght));
    println!("Generate random password with chars: {}", genchars(lenght));
    println!("Generate random password with numbers: {}", gennum(lenght));
}