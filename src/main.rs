extern crate rsw;
extern crate regex;
#[macro_use]
extern crate clap;

use rsw::util::*;
use rsw::parse;
use rsw::template;
use std::fs;
use regex::Regex;
use clap::App;
use std::path::Path;

use log::{info, warn};
use log4rs;


// 编译后的静态文件
static BUILD_DIR: &str = "build";
// 资源文件，如css、js、图片
static PUBLIC_DIR: &str = "public";
// 源文件目录
static SRC_DIR: &str = "src";

fn copy_public(target: &str, src: &str) {
    let dir = Path::new(src);
    // 遍历目录
    for entry in fs::read_dir(dir).expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = child.to_str().unwrap();

            if child.is_file() {
                // 判断如果是模版文件就忽略
                let re_template_file = Regex::new(r".*___.*\.html$").unwrap();
                if re_template_file.is_match(file_name) {
                    continue;
                }

                // 拆分源文件名，方便以后组合成目标文件名
                let dirs: Vec<&str> = file_name.splitn(2, '/').collect();
                let new_file = format!("{}/{}", target, dirs[1]);
                // 将目标文件从右边按`/`拆分得到目录
                let dirs: Vec<&str> = new_file.splitn(2, '/').collect();
                // 如果要复制的目标目录不存在，则创建
                create_not_exists(dirs[0]);
                // 复制文件
                match fs::copy(file_name, &new_file) {
                    Err(why) => panic!("{} -> {}: {}", file_name, new_file, why.to_string()),
                    Ok(_) => println!("{} -> {}", file_name, new_file),
                }
            } else {
                // 如果是目录，则继续递归
                copy_public(target, file_name);
            }
        }
    }
}
fn loop_parse(build: &str, public: &str, src: &str) {
    let path = Path::new(src);
    // 递归方式列出所有的源文件
    for entry in fs::read_dir(path).expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let child = entry.path();
            let file_name = child.to_str().unwrap();
            if child.is_file() {
                let md_file = parse::parse_md_file(build, &child);
                template::render(public, md_file);
            } else {
                loop_parse(build, public, file_name);
            }
        }
    }
}

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let yaml = load_yaml!("cli.yml");
    info!("projectname:{:?}",yaml);
    let matches = App::from_yaml(yaml).get_matches();

    warn!("projectname:{:?}",matches);
    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("PROJECT").unwrap();
        println!("projectname:{}", project_name);
        // 创建项目饼初始化工作空间
        init_work_space(project_name, PUBLIC_DIR, SRC_DIR);
    }

    if let Some(_) = matches.subcommand_matches("build") {
        // copy public下的资源文件到build目录，但是会忽略模版文件
        copy_public(BUILD_DIR, PUBLIC_DIR);
        // 解析md文件
        loop_parse(BUILD_DIR, PUBLIC_DIR, SRC_DIR);
    }
    println!("Hello, world!");
}
