use std::env;

fn main() { 
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("usage: `fib $START $END` or `fib $END`");
        return;
    }

    let start:u64 = start(&args);
    let end:u64 = end(&args);
    
    let mut t1 = start;
    let mut t2 = start + 1;
    let mut next: u64;

    for _ in start..end {
        println!("{} ", t1);
        next = t1 + t2;
        t1 = t2;
        t2 = next;
    }
}

fn start(args:&std::vec::Vec<String>) -> u64 {
    if args.len() > 2 {
        get_arg(&args, 1)
    }
    else {
        0
    }
}

fn end(args:&Vec<String>) -> u64 {
    let index: usize;
    if args.len() > 2 {
        index = 2;
    }
    else {
        index = 1;
    }

    get_arg(&args, index)
}

fn get_arg(args:&Vec<String>, index:  usize) -> u64 {
    match &args[index].parse::<u64>(){
        Ok(num) => *num,
        Err(_) => 0,
    }
}
