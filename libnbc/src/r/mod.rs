//! 编译器运行入口

use std::error::Error;

use log::debug;

use super::c::a::{编译前端设置, 阶段1};
use super::c::t::源文件;

/// 运行参数
#[derive(Debug, Clone)]
pub struct 运行参数 {
    /// 要编译的源代码文件的路径 (入口文件)
    pub 源文件: String,
}

impl Default for 运行参数 {
    fn default() -> Self {
        Self {
            源文件: "".into()
        }
    }
}

/// 运行入口
pub fn 运行(参数: 运行参数) -> Result<(), Box<dyn Error>> {
    debug!("{:?}", 参数);

    // 测试: 运行编译器前端阶段 1
    let 文件 = 源文件 {
        编号: 0,
        路径: 参数.源文件,
        哈希: "".into(),
    };
    let r = 阶段1(编译前端设置::default(), 文件)?;
    println!("{:#?}", r);

    // TODO
    Ok(())
}
