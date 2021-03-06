extern crate regex;

use self::regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
#[derive(Clone)]
pub struct MdFile {
    pub file_name: String,
    pub target_file_name: String,
    pub yaml_str: String,
    pub md_str: String,
}

impl MdFile {
    fn new<T: Into<String>>(file_name: T, target_file_name: T, yaml_str: T, md_str: T) -> Self {
        MdFile {
            file_name: file_name.into(),
            target_file_name: target_file_name.into(),
            yaml_str: yaml_str.into(),
            md_str: md_str.into(),
        }
    }
}

pub fn parse_md_file(build: &str, path: &Path) -> MdFile {
    let display = path.display();

    // 以只读方式打开路径，返回`io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回`io::Result<usize>`
    let mut content = String::new();
    if let Err(err) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, err);
    }

    let re_md = Regex::new(r"^---([\s\S]*?)---([\s\S]*)").unwrap();
    let caps = re_md.captures(content.as_str()).unwrap();
    let yaml_str = caps.get(1).unwrap().as_str();

    // 提取正文markdown内容
    let mut md_str  = String::from(caps.get(2).unwrap().as_str());

    // 提取rsw:://格式转成超链接
    let re_rsw = Regex::new(r"(\(rsw://(?P<url_str>\S+)\.md(?P<query_str>[\S]*)\))").unwrap();
    md_str = String::from(re_rsw.replace_all(&md_str, "($url_str.html$query_str)"));

    let file_name = path.to_str().unwrap();
    // 将src路径转成build路径
    let file_names: Vec<&str> = file_name.splitn(2, '/').collect();
    let target_file = format!("{}/{}", build, file_names[1]);

    // 将md扩展转换成html
    let target_files: Vec<&str> = target_file.rsplitn(2, '.').collect();
    let target_file_name = format!("{}{}", target_files[1], ".html");
    return MdFile::new(file_name, target_file_name.as_str(), yaml_str, &md_str);
}