use std::env;
use std::process;
use regex::Regex;
use colored::*;
use std::collections::HashSet;
use tracing;

pub mod find;
pub mod tracing_init;

fn main() {
    tracing_init::tracing_init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("使用方式： {} <目标目录><要搜索的正则表达式>", args[0]);
        process::exit(1);
    }

    let pattern = &args[2];
    let regex = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("无效的正则表达式 '{}': {}", pattern, err);
            process::exit(1);
        }
    };

    match find::find(&args[1], &regex) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("{}","未找到匹配项。".yellow());//输出黄色
            } else {
                println!("{}","找到以下匹配项:".yellow());
                let mut matches: Vec<_> = matches.into_iter().collect::<HashSet<_>>().into_iter().collect();//去重
                matches.sort(); //排序
                for file in matches {
                    println!("{}", file.bright_magenta());//输出品红色
                }
            }
        }
        Err(error) => {
            eprintln!("发生错误: {}", error);
            process::exit(1);
        }
    }
    tracing::event!(tracing::Level::INFO, "");//输出日志
}