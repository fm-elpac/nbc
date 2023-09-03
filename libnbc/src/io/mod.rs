//! 输入输出封装

use std::error::Error;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/// 清理文件路径
pub fn 路径标准化(路径: String) -> String {
    Path::new(&路径)
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/// 拼接文件路径
pub fn 拼接路径(a: String, b: String) -> String {
    let p: PathBuf = [a, b].iter().collect();
    p.to_str().unwrap().to_string()
}

/// 读取一个文本文件的内容 (utf-8 编码)
pub fn 读取文件(路径: String) -> Result<String, Box<dyn Error>> {
    Ok(read_to_string(路径)?)
}

// TODO
