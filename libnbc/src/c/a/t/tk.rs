//! 词法分析 (token) 的一个语词 (token) 的数据结构

use super::super::super::t::{源代码位置, 语法树数字格式};

/// 一个语词 (token)
#[derive(Debug, Clone)]
pub struct 语词 {
    /// 语词类型
    pub 类型: 语词类型,
    /// 在源代码中的位置
    pub 位置: 源代码位置,
    /// 对应的文本 (可能为 `""`)
    pub 文本: String,
}

/// 语词 (token) 的类型
#[derive(Debug, Clone, PartialEq)]
pub enum 语词类型 {
    /// 换行: 表示一行的结束
    换行,
    /// 注释: (后续处理会被忽略) 行尾的注释内容
    注释,
    /// 缩进 (缩进层级): 首行缩进内容, 强制使用 2 个空格作为一级缩进
    缩进(u8),
    /// 空白: (后续处理会被忽略) 代码中的空格, 连续的多个空格相当于一个空格
    空白,
    /// 数字字面量 (语法树数字格式)
    数字(语法树数字格式),
    /// 整个字符串的开始 (处理插值)
    文本开始,
    /// 字符串片段: 不是插值的部分
    文本片段,
    /// 整个字符串的结束 (处理插值)
    文本结束,
    /// `(` 小括号: 用于多种语法
    小括号左,
    /// `)` 小括号: 用于多种语法
    小括号右,
    /// `[` 中括号
    中括号左,
    /// `]` 中括号
    中括号右,
    /// `{` 大括号
    大括号左,
    /// `}` 大括号
    大括号右,
    /// `,` 逗号: 用于多种语法
    逗号,
    /// `:` 冒号
    冒号,
    /// `、` 顿号
    顿号,
    /// `。` 句号
    句号,
    /// 没有分词 (以后需要分词) 的短语
    待分词,
    /// 已经分词 (后续禁止分词) 的短语
    已分词,
}
