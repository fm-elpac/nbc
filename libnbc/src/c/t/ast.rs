//! 抽象语法树 (AST)
//!
//! 编译器前端和后端共享的数据结构

use super::{数据类型, 数据类型定义, 标记, 源代码位置};

/// 语法树节点包装: 提供源代码位置信息
#[derive(Debug, Clone)]
pub struct 语法树包装(语法树, 源代码位置);

/// 抽象语法树的一个节点
#[derive(Debug, Clone)]
pub enum 语法树 {
    /// 模块: 一个源文件对应一个模块
    ///
    /// 一个模块 (顶级) 可以包含的内容:
    /// + `语句`: 普通语句, 赋值语句
    /// + `函数`: 定义函数
    /// + `类型`: 定义类型, 定义枚举
    /// + `变量`: 定义变量, 定义常量
    /// + `导入`
    /// + `导出`
    /// + `返回`
    /// + `循环`
    /// + `若`
    /// + `另`
    /// + `匹配`
    模块(语法树代码块),
    /// 语句
    语句(语法树语句),
    /// 表达式
    表达式(语法树表达式),
    /// 定义函数
    函数(语法树函数),
    /// 定义类型, 定义枚举
    类型(语法树类型),
    /// 定义变量, 定义常量
    变量(语法树变量),
    /// `导入`
    导入(语法树导入),
    /// `导出`
    导出(语法树导出),
    /// `返回`: 函数提前返回
    返回(语法树返回),
    /// `循环`: 循环语句
    循环(语法树循环),
    /// `破`: 跳出循环
    破(语法树破),
    /// `若`: 分支语句
    若(语法树分支),
    /// `匹配`: 模式匹配分支
    匹配(语法树匹配),
}

/// 代码块: 一个代码块由若干序列组成
#[derive(Debug, Clone)]
pub struct 语法树代码块 {
    /// 代码块的内容
    pub 内容: Vec<Box<语法树包装>>,
}

/// 语句
#[derive(Debug, Clone)]
pub enum 语法树语句 {
    /// 普通语句: 表达式 + 句号
    普通语句(语法树表达式),
    /// 赋值语句: `=` 或 `+=` 等, 不能以句号结尾
    赋值语句(语法树赋值语句),
}

/// 赋值语句
#[derive(Debug, Clone)]
pub struct 语法树赋值语句 {
    /// 赋值运算符: `=`, `+=` 等
    pub 赋值: &'static str,
    /// 左侧: 被赋值的东西 (表达式)
    pub 目标: Box<语法树包装>,
    /// 右侧: 值 (表达式)
    pub 值: Box<语法树包装>,
}

/// 表达式
#[derive(Debug, Clone)]
pub enum 语法树表达式 {
    /// 字面量
    字面量(语法树字面量),
    /// 访问成员变量, 或访问变量
    访问成员(语法树访问成员),
    /// 函数调用
    函数调用(语法树函数调用),
}

/// 字面量
#[derive(Debug, Clone)]
pub enum 语法树字面量 {
    /// 数字
    数字(语法树数字),
    /// 文本
    文本(语法树文本),
}

/// 字面量: 数字
#[derive(Debug, Clone)]
pub struct 语法树数字 {
    /// 数字的格式
    pub 格式: 语法树数字格式,
    /// 值: 以字符串 (原始形式) 表示
    pub 值: String,
}

/// 字面量数字的格式
#[derive(Debug, Clone, PartialEq)]
pub enum 语法树数字格式 {
    /// 10 进制,
    进制10,
    /// 2 进制 `0b`
    进制2,
    /// 8 进制 `0o`
    进制8,
    /// 16 进制 `0x`
    进制16,
    /// 小数 (浮点数, 不是整数)
    小数,
}

/// 字面量: 文本
#[derive(Debug, Clone)]
pub struct 语法树文本 {
    /// 若干文本片段
    pub 片段: Vec<String>,
    /// 表达式: 用于文本插值
    pub 插值: Vec<Box<语法树包装>>,
}

/// 访问成员变量
#[derive(Debug, Clone)]
pub struct 语法树访问成员 {
    /// 被访问的东西 (表达式), 如果为空表示直接访问变量
    pub 基础: Option<Box<语法树包装>>,
    /// 访问的成员 (变量) 名称
    pub 名称: String,
}

/// 函数调用
#[derive(Debug, Clone)]
pub struct 语法树函数调用 {
    /// 被调用的函数
    pub 函数: 数据类型定义,
    /// 传递的参数 (表达式)
    pub 参数: Vec<Box<语法树包装>>,
}

/// 定义函数
#[derive(Debug, Clone)]
pub struct 语法树函数 {
    /// 函数名称
    pub 名称: String,
    /// 函数返回值的类型
    pub 返回类型: 数据类型,
    /// 函数参数列表: (名称, 类型)
    pub 参数: Vec<(String, 数据类型)>,
    /// 标记语法
    pub 标记: Vec<标记>,
    /// 泛型
    pub 泛型: Vec<String>,
    /// 函数的内容 (代码块)
    pub 内容: 语法树代码块,
}

/// 可以定义的类型
#[derive(Debug, Clone)]
pub enum 语法树类型类型 {
    /// 结构体 (struct)
    结构体,
    /// 枚举
    枚举,
}

/// 定义类型
#[derive(Debug, Clone)]
pub struct 语法树类型 {
    /// 类型名称
    pub 名称: String,
    /// 具体的定义类型
    pub 类型: 语法树类型类型,
    /// 标记语法
    pub 标记: Vec<标记>,
    /// 泛型
    pub 泛型: Vec<String>,
    /// 内容: (名称, 类型)
    pub 内容: Vec<(String, 数据类型)>,
}

/// 定义变量
#[derive(Debug, Clone)]
pub struct 语法树变量 {
    /// 变量名称
    pub 名称: String,
    /// 不可变变量 (`定义常量`)
    pub 常量: bool,
    /// 变量类型
    pub 类型: 数据类型,
    /// 初始值 (表达式, `None` 表示默认值)
    pub 初值: Option<Box<语法树包装>>,
}

/// 导入
#[derive(Debug, Clone)]
pub struct 语法树导入 {
    /// 导入的名称
    pub 名称: Option<String>,
    /// 导入模块的路径
    pub 路径: Option<String>,
    /// 导入方式
    pub 方式: Option<语法树导入方式>,
}

/// 导入方式
#[derive(Debug, Clone)]
pub enum 语法树导入方式 {
    /// 全部导入: 比如 `导入标准库的全部。`
    全部,
    /// 指定导入名称列表, 比如 `导入标准库的序章。`
    ///
    /// (原导出名称, 导入重命名 (导入后的名称, 默认与原名称一致))
    名称(Vec<(String, Option<String>)>),
}

/// 导出
#[derive(Debug, Clone)]
pub struct 语法树导出 {
    /// 导出的名称
    pub 名称: String,
}

/// 返回
#[derive(Debug, Clone)]
pub struct 语法树返回 {
    /// 返回的东西 (表达式)
    pub 值: Option<语法树表达式>,
}

/// 循环
#[derive(Debug, Clone)]
pub struct 语法树循环 {
    /// 循环模式: `None` 表示无限循环
    pub 模式: Option<语法树循环模式>,
    /// 循环变量 (名称)
    pub 变量: Option<String>,
    /// 循环内容 (代码块)
    pub 内容: 语法树代码块,
}

/// 循环模式
#[derive(Debug, Clone)]
pub enum 语法树循环模式 {
    /// 循环 N 次 (表达式): 比如 `循环 3 次:`
    次(Box<语法树包装>),
    /// 范围循环
    范围(语法树循环范围),
    /// 条件循环 (表达式): 比如 `循环, 若 x > 1:`
    条件(Box<语法树包装>),
}

/// 范围循环: 比如 `循环 3 至 9, 变量 x, 步长 3:`
#[derive(Debug, Clone)]
pub struct 语法树循环范围 {
    /// 开始值 (表达式)
    pub 开始: Box<语法树包装>,
    /// 结束值 (表达式)
    pub 结束: Box<语法树包装>,
    /// 步长 (表达式)
    pub 步长: Box<语法树包装>,
    /// 包括结束值
    pub 到: bool,
}

/// 跳出循环
#[derive(Debug, Clone)]
pub struct 语法树破 {
    /// 跳出层级: `0` 类似 `continue`, `1` 类似 `break`
    pub 层级: u32,
}

/// 分支语句
#[derive(Debug, Clone)]
pub struct 语法树分支 {
    /// 判断条件 (表达式)
    pub 条件: Box<语法树包装>,
    /// `若:` 代码块
    pub 若: 语法树代码块,
    /// `另:` 分支 (或 `另若`)
    pub 另: Option<Box<语法树包装>>,
}

/// 模式匹配分支
#[derive(Debug, Clone)]
pub struct 语法树匹配 {
    // TODO
}
