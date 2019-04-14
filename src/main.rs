use std::env;

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("usage: `fib $START $END` or `fib $END`");
        return;
    }

    let start:u32 = if args.len() > 2 {
        match &args[1].parse::<u32>(){
            Ok(num) => *num,
            Err(_) => 0,
        }
    }
    else {
        0
    };

    let end:u32 = {
        let index: usize;
        if args.len() > 2 {
            index = 2;
        }
        else {
            index = 1;
        }
        match &args[index].parse::<u32>(){
            Ok(num) => *num,
            Err(_) => 0,
        }

    };
    
    let mut t1 = 0;
    let mut t2 = 1;
    let mut next: i32;

    for _ in start..end {
        print!("{} ", t1);
        next = t1 + t2;
        t1 = t2;
        t2 = next;
    }
    print!("\n");
}
