extern crate regex;
extern crate yaml_rust;
extern crate comrak;

use crate::util::*;
use self::regex::Regex;
//use std::io::prelude::*
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use crate::parse::MdFile;
use self::yaml_rust::{YamlLoader, Yaml};
use self::comrak::{markdown_to_html, ComrakOptions};

pub fn render(public: &str, md_file: MdFile) {
    let yaml_docs = YamlLoader::load_from_str(md_file.yaml_str.as_str()).unwrap();
    let html_str = markdown_to_html(md_file.md_str.as_str(), &ComrakOptions::default());

    // 渲染模版
    let html_content = render_template(public, yaml_docs, html_str.as_str());

    // 生成目标文件
    generate_html(md_file.target_file_name.as_str(), html_content.as_str());
}

fn generate_html(html_path: &str, html_content: &str) {
    // 拆分文件名，如`build/2017/01/01/happy.html`得到的是`["happy.html", "build/2017/01/01"]`
    let dirs: Vec<&str> = html_path.rsplitn(2, '/').collect();
    create_not_exists(dirs[1]);

    let path = Path::new(html_path);
    let display = path.display();

    // 以只写模式打开文件，返回`io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("create {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    match file.write_all(html_content.as_bytes()) {
        Err(why) => panic!("write {}: {}", display, why.to_string()),
        Ok(_) => println!("write {}", display),
    };
}

fn render_template(public: &str, yaml_docs: Vec<Yaml>, html_str: &str) -> String {
    // 从yaml数据中取出md文件的元数据
    let yaml_doc = &yaml_docs[0];

    let template = yaml_doc["template"].as_str().unwrap();
    let template_names: Vec<&str> = template.rsplitn(2, '/').collect();
    let mut file_name = String::new();
    if template_names.len() == 1 {
        file_name.push_str("___");
        file_name.push_str(template);
    } else {
        file_name.push_str(template_names[1]);
        file_name.push_str("/___");
        file_name.push_str(template_names[0]);
    }

    let template_file = format!("{}/{}.html", public, file_name);

    // 打开模板文件
    let path = Path::new(template_file.as_str());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file
    };

    let mut template_content = String::new();
    if let Err(err) = file.read_to_string(&mut template_content) {
        panic!("couldn't read {}: {}", display, err.to_string());
    }
    //if let Err(err) = file.read_to_string(&mut template_content) {
    //    panic!("couldn't read {}: {}", display, err.to_string());
    //}

    // 将author渲染到模板中
    let re_author = Regex::new(r"\{\{\s*author\s*\}\}").unwrap();
    template_content = String::from(re_author.replace_all(template_content.as_str(), yaml_doc["author"].as_str().unwrap_or("RustWriter")));

    // 将content渲染到模板中
    let re_content = Regex::new(r"\{\{\s*content\s*\}\}").unwrap();
    template_content = String::from(re_content.replace_all(template_content.as_str(), html_str));

    return template_content;
}