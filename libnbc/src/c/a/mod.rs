//! 编译器前端

use std::error::Error;

use super::super::io::读取文件;
use super::t::源文件;

mod pp;
mod s1;
mod t;

pub use pp::预处理;
pub use t::{编译前端设置, 语词, 语词类型};

pub mod d;
pub mod s2;
pub mod s3;
pub mod tc;

use s1::词法分析器;

/// 编译器前端第 1 阶段: 源代码预处理, 词法分析 (token)
pub fn 阶段1(
    设置: 编译前端设置, 文件: 源文件
) -> Result<Vec<语词>, Box<dyn Error>> {
    let 源代码 = 读取文件(文件.路径.clone())?;

    let 代码 = 预处理(源代码, 文件.clone())?;

    let mut 分析器 = 词法分析器::new(设置, 文件);
    for c in 代码.chars() {
        分析器.吃(c)?;
    }
    // 最后必须追加一个 `\n` 字符 (换行)
    分析器.吃('\n')?;

    Ok(分析器.结果()?)
}
