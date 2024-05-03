use std::env;


use std::process;

use minigrep::run;
use minigrep::Config;

/**
 * 
 * 
 * 
 * use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // 当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程，
        // 其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    //   如果结果等于符合条件 则赋值给e
    if let Err(e) =  run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


 */
fn main() {
    let args: Vec<String> = env::args().collect();

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // 当 Result 包含错误时，我们不再调用 panic 让程序崩溃，而是通过 process::exit(1) 来终结进程，
        // 其中 1 是一个信号值(事实上非 0 值都可以)，通知调用我们程序的进程，程序是因为错误而退出的。
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    //   如果结果等于符合条件 则赋值给e
    if let Err(e) =  run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


