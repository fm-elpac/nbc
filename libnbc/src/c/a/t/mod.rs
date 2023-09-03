//! 编译器前端的主要数据结构

mod tk;

pub use tk::{语词, 语词类型};

pub mod at;

/// 编译器前端的编译设置
#[derive(Debug, Clone)]
pub struct 编译前端设置 {
    // 标准库的路径
    pub 标准库: String,
}

impl Default for 编译前端设置 {
    fn default() -> Self {
        Self {
            标准库: "".into()
        }
    }
}

// TODO
