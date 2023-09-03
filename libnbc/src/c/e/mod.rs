//! 编译器错误定义

use std::error::Error;
use std::fmt::Display;

use super::t::{源代码位置, 源文件};

// 编译器错误代码定义

/// 编译错误 NE0001: 源代码文件不是 utf-8 编码
pub const NE0001: u32 = 1;
/// 编译错误 NE0002: 源代码文件含有禁止使用的 Unicode 字符
pub const NE0002: u32 = 2;
/// 编译错误 NE0003: 错误的缩进长度
pub const NE0003: u32 = 3;
/// 编译错误 NE0004: 禁止分词中不能使用字符
pub const NE0004: u32 = 4;
/// 编译错误 NE0005: 错误的数字格式
pub const NE0005: u32 = 5;
/// 编译错误 NE0006: 字符串不能换行
pub const NE0006: u32 = 6;
/// 编译错误 NE0007: 字符串中错误的转义
pub const NE0007: u32 = 7;
/// 编译错误 NE0008: 字符串插值不能使用复杂表达式
pub const NE0008: u32 = 8;

/// 编译器错误
#[derive(Debug, Clone)]
pub struct 编译器错误 {
    pub 代码: u32,
    pub 描述: String,
    pub 文件: 源文件,
    pub 位置: 源代码位置,
}

impl Error for 编译器错误 {}

impl Display for 编译器错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "编译错误 NE{:04}: {}\n  {}:({}, {}):({}, {})",
            self.代码,
            self.描述,
            self.文件.路径,
            self.位置.开始.0,
            self.位置.开始.1,
            self.位置.结束.0,
            self.位置.结束.1
        )
    }
}

/// 构造 NE0001 错误
pub fn ne0001(文件: &源文件) -> 编译器错误 {
    编译器错误 {
        代码: NE0001,
        描述: "源代码文件不是 utf-8 编码".into(),
        文件: 文件.clone(),
        位置: 源代码位置::default(),
    }
}

/// 构造 NE0002 错误
pub fn ne0002(文件: &源文件, 位置: &源代码位置) -> 编译器错误 {
    编译器错误 {
        代码: NE0002,
        描述: "源代码文件含有禁止使用的 Unicode 字符".into(),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

/// 构造 NE0003 错误
pub fn ne0003(文件: &源文件, 位置: &源代码位置, 长度: usize) -> 编译器错误 {
    编译器错误 {
        代码: NE0003,
        描述: format!("错误的缩进长度 {} (每级缩进应该使用 2 个空格)", 长度),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

fn 显示字符(字符: char) -> String {
    match 字符 {
        '\n' => "\n".into(),
        _ => 字符.into(),
    }
}

/// 构造 NE0004 错误
pub fn ne0004(文件: &源文件, 位置: &源代码位置, 字符: char) -> 编译器错误 {
    编译器错误 {
        代码: NE0004,
        描述: format!("禁止分词中不能使用字符 {}", 显示字符(字符)),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

/// 构造 NE0005 错误
pub fn ne0005(文件: &源文件, 位置: &源代码位置, 数字: String) -> 编译器错误 {
    编译器错误 {
        代码: NE0005,
        描述: format!("错误的数字格式 {}", 数字),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

/// 构造 NE0006 错误
pub fn ne0006(文件: &源文件, 位置: &源代码位置) -> 编译器错误 {
    编译器错误 {
        代码: NE0006,
        描述: "字符串不能换行".into(),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

/// 构造 NE0007 错误
pub fn ne0007(文件: &源文件, 位置: &源代码位置, 字符: char) -> 编译器错误 {
    编译器错误 {
        代码: NE0007,
        描述: format!("字符串中错误的转义 {}", 显示字符(字符)),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}

/// 构造 NE0008 错误
pub fn ne0008(文件: &源文件, 位置: &源代码位置, 字符: char) -> 编译器错误 {
    编译器错误 {
        代码: NE0008,
        描述: format!("字符串插值不能使用复杂表达式 {}", 显示字符(字符)),
        文件: 文件.clone(),
        位置: 位置.clone(),
    }
}
