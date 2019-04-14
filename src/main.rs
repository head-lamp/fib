use std::env;

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("usage: `fib $START $END` or `fib $END`");
        return;
    }

    let start:i32 = start(&args);
    let end:i32 = end(&args);
    
    let mut t1 = start;
    let mut t2 = start + 1;
    let mut next: i32;

    for _ in start..end {
        print!("{} ", t1);
        next = t1 + t2;
        t1 = t2;
        t2 = next;
    }
    print!("\n");
}

fn start(args:&std::vec::Vec<String>) -> i32 {
    if args.len() > 2 {
        get_arg(&args, 1)
    }
    else {
        0
    }
}

fn end(args:&Vec<String>) -> i32 {
    let index: usize;
    if args.len() > 2 {
        index = 2;
    }
    else {
        index = 1;
    }

    get_arg(&args, index)
}

fn get_arg(args:&Vec<String>, index:  usize) -> i32 {
    match &args[index].parse::<i32>(){
        Ok(num) => *num,
        Err(_) => 0,
    }
}
