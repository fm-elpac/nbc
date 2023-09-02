//! 歧语言语法常量定义

// 词法解析 (token) 的特殊字符

/// `\n` 特殊字符: 换行
pub const 特殊字符换行: char = '\n';
/// ` ` 特殊字符: 空格
pub const 特殊字符空格: char = ' ';
/// `"` 特殊字符: 双引号
pub const 特殊字符引号: char = '"';
/// `#` 特殊字符: 注释
pub const 特殊字符注释: char = '#';
/// `(` 特殊字符: 小括号
pub const 特殊字符小括号左: char = '(';
/// `)` 特殊字符: 小括号
pub const 特殊字符小括号右: char = ')';
/// `,` 特殊字符: 逗号
pub const 特殊字符逗号: char = ',';
/// `:` 特殊字符: 冒号
pub const 特殊字符冒号: char = ':';
/// `=` 特殊字符: 等号
pub const 特殊字符等号: char = '=';
/// `[` 特殊字符: 中括号
pub const 特殊字符中括号左: char = '[';
/// `\` 特殊字符: 反斜杠
pub const 特殊字符转义: char = '\\';
/// `]` 特殊字符: 中括号
pub const 特殊字符中括号右: char = ']';
/// ` 特殊字符: 反引号
pub const 特殊字符反引号: char = '`';
/// `{` 特殊字符: 大括号
pub const 特殊字符大括号左: char = '{';
/// `}` 特殊字符: 大括号
pub const 特殊字符大括号右: char = '}';
/// `、` 特殊字符: 顿号
pub const 特殊字符顿号: char = '、';
/// `。` 特殊字符: 句号
pub const 特殊字符句号: char = '。';
/// `0` 特殊字符: 用于数字
pub const 特殊字符0: char = '0';
/// `9` 特殊字符: 用于数字
pub const 特殊字符9: char = '9';
/// `a` 特殊字符: 用于数字
pub const 特殊字符A: char = 'a';
/// `f` 特殊字符: 用于数字
pub const 特殊字符F: char = 'f';
/// `.` 特殊字符: 用于数字
pub const 特殊字符点: char = '.';
/// `_` 特殊字符: 用于数字
pub const 特殊字符下划线: char = '_';
/// `e` 特殊字符: 用于数字
pub const 特殊字符E: char = 'e';
/// `-` 特殊字符: 用于数字
pub const 特殊字符减: char = '-';

// 关键词

/// `的` 语句关键词
pub const 关键词的: &'static str = "的";
/// `是` 语句关键词
pub const 关键词是: &'static str = "是";

/// `定义` 行首关键词
pub const 关键词定义: &'static str = "定义";
/// `类型` 行首关键词: 定义类型
pub const 关键词类型: &'static str = "类型";
/// `枚举` 行首关键词: 定义枚举
pub const 关键词枚举: &'static str = "枚举";
/// `变量` 行首关键词: 定义变量
pub const 关键词变量: &'static str = "变量";
/// `常量` 行首关键词: 定义常量
pub const 关键词常量: &'static str = "常量";
/// `设` 行首关键词
pub const 关键词设: &'static str = "设";
/// `导入` 行首关键词
pub const 关键词导入: &'static str = "导入";
/// `导出` 行首关键词
pub const 关键词导出: &'static str = "导出";
/// `返回` 行首关键词
pub const 关键词返回: &'static str = "返回";
/// `循环` 行首关键词
pub const 关键词循环: &'static str = "循环";
/// `破` 行首关键词
pub const 关键词破: &'static str = "破";
/// `若` 行首关键词
pub const 关键词若: &'static str = "若";
/// `另` 行首关键词
pub const 关键词另: &'static str = "另";
/// `匹配` 行首关键词
pub const 关键词匹配: &'static str = "匹配";

/// `参数` 条件关键词
pub const 关键词参数: &'static str = "参数";
/// `标记` 条件关键词
pub const 关键词标记: &'static str = "标记";
/// `泛型` 条件关键词
pub const 关键词泛型: &'static str = "泛型";
/// `次` 条件关键词
pub const 关键词次: &'static str = "次";
/// `至` 条件关键词
pub const 关键词至: &'static str = "至";
/// `到` 条件关键词
pub const 关键词到: &'static str = "到";
/// `步长` 条件关键词
pub const 关键词步长: &'static str = "步长";
/// `全部` 条件关键词
pub const 关键词全部: &'static str = "全部";

// 标记语法

/// `内部` 标记语法
pub const 标记内部: &'static str = "内部";
/// `自动类型转换` 标记语法
pub const 标记自动类型转换: &'static str = "自动类型转换";
/// `运算符` 标记语法
pub const 标记运算符: &'static str = "运算符";
/// `前运算符` 标记语法
pub const 标记前运算符: &'static str = "前运算符";
/// `后运算符` 标记语法
pub const 标记后运算符: &'static str = "后运算符";
/// `严格数学` 标记语法: 严格数学模式
pub const 标记严格数学: &'static str = "严格数学";

// 内置数据类型

/// `u1` 内置数据类型 (简单类型): 无符号 1 位二进制整数
pub const 内置类型U1: &'static str = "u1";
/// `二` 类型别名 (`u1`)
pub const 内置类型U1_2: &'static str = "二";
/// `u8` 内置数据类型 (简单类型): 无符号 8 位二进制整数
pub const 内置类型U8: &'static str = "u8";
/// `u16` 内置数据类型 (简单类型): 无符号 16 位二进制整数
pub const 内置类型U16: &'static str = "u16";
/// `u32` 内置数据类型 (简单类型): 无符号 32 位二进制整数
pub const 内置类型U32: &'static str = "u32";
/// `u64` 内置数据类型 (简单类型): 无符号 64 位二进制整数
pub const 内置类型U64: &'static str = "u64";
/// `u128` 内置数据类型 (简单类型): 无符号 128 位二进制整数
pub const 内置类型U128: &'static str = "u128";
/// `i8` 内置数据类型 (简单类型): 有符号 8 位二进制整数
pub const 内置类型I8: &'static str = "i8";
/// `i16` 内置数据类型 (简单类型): 有符号 16 位二进制整数
pub const 内置类型I16: &'static str = "i16";
/// `i32` 内置数据类型 (简单类型): 有符号 32 位二进制整数
pub const 内置类型I32: &'static str = "i32";
/// `i64` 内置数据类型 (简单类型): 有符号 64 位二进制整数
pub const 内置类型I64: &'static str = "i64";
/// `i128` 内置数据类型 (简单类型): 有符号 128 位二进制整数
pub const 内置类型I128: &'static str = "i128";
/// `f32` 内置数据类型 (简单类型): 32 位浮点数
pub const 内置类型F32: &'static str = "f32";
/// `f64` 内置数据类型 (简单类型): 64 位浮点数
pub const 内置类型F64: &'static str = "f64";

/// `整数` 内置数据类型 (简单类型): 自动类型整数
pub const 内置类型整数: &'static str = "整数";
/// `实数` 内置数据类型 (简单类型): 自动类型浮点数
pub const 内置类型实数: &'static str = "实数";
/// `数字` 内置数据类型 (简单类型): 自动类型数字
pub const 内置类型数字: &'static str = "数字";
/// `文本` 内置数据类型 (简单类型): utf-8 字符串
pub const 内置类型文本: &'static str = "文本";

/// `函数` 内置数据类型 (复合类型)
pub const 内置类型函数: &'static str = "函数";
/// `数组` 内置数据类型 (复合类型)
pub const 内置类型数组: &'static str = "数组";
/// `集合` 内置数据类型 (复合类型)
pub const 内置类型集合: &'static str = "集合";
/// `字典` 内置数据类型 (复合类型)
pub const 内置类型字典: &'static str = "字典";

// 内置函数

/// `输出` 内置函数: 标准输出
pub const 内置函数输出: &'static str = "输出";
/// `写` 内置函数: 标准输出
pub const 内置函数写: &'static str = "写";
/// `换行` 内置函数: 标准输出
pub const 内置函数换行: &'static str = "换行";

// 内置模块

/// `标准库` 内置模块
pub const 内置标准库: &'static str = "标准库";
/// `序章` 标准库的序章
pub const 内置标准库序章: &'static str = "序章";

// 内置特殊函数 (运算符): 赋值

/// `+=` 赋值运算符
pub const 内置运算符加等: &'static str = "+=";
/// `-=` 赋值运算符
pub const 内置运算符减等: &'static str = "-=";
/// `*=` 赋值运算符
pub const 内置运算符乘等: &'static str = "*=";
/// `/=` 赋值运算符
pub const 内置运算符除等: &'static str = "/=";
/// `%=` 赋值运算符
pub const 内置运算符余等: &'static str = "%=";
/// `&=` 赋值运算符
pub const 内置运算符与等: &'static str = "&=";
/// `|=` 赋值运算符
pub const 内置运算符或等: &'static str = "|=";
/// `^=` 赋值运算符
pub const 内置运算符异或等: &'static str = "^=";
/// `<<=` 赋值运算符
pub const 内置运算符左移等: &'static str = "<<=";
/// `>>=` 赋值运算符
pub const 内置运算符右移等: &'static str = ">>=";
/// `>>>=` 赋值运算符
pub const 内置运算符右移等3: &'static str = ">>>=";

// 内置运算符

/// `+` 中间运算符
pub const 内置运算符加: &'static str = "+";
/// `-` 中间运算符
pub const 内置运算符减: &'static str = "-";
/// `*` 中间运算符
pub const 内置运算符乘: &'static str = "*";
/// `/` 中间运算符
pub const 内置运算符除: &'static str = "/";
/// `%` 中间运算符
pub const 内置运算符余: &'static str = "%";
/// `&` 中间运算符
pub const 内置运算符与: &'static str = "&";
/// `|` 中间运算符
pub const 内置运算符或: &'static str = "|";
/// `^` 中间运算符
pub const 内置运算符异或: &'static str = "^";
/// `<<` 中间运算符
pub const 内置运算符左移: &'static str = "<<";
/// `>>` 中间运算符
pub const 内置运算符右移: &'static str = ">>";
/// `>>>` 中间运算符
pub const 内置运算符右移3: &'static str = ">>>";

/// `+` 前运算符
pub const 内置运算符正: &'static str = "+";
/// `-` 前运算符
pub const 内置运算符负: &'static str = "-";
/// `~` 前运算符
pub const 内置运算符非: &'static str = "~";

/// `==` 比较运算符
pub const 内置运算符等于: &'static str = "==";
/// `!=` 比较运算符
pub const 内置运算符不等: &'static str = "!=";
/// `>` 比较运算符
pub const 内置运算符大于: &'static str = ">";
/// `<` 比较运算符
pub const 内置运算符小于: &'static str = "<";
/// `>=` 比较运算符
pub const 内置运算符大于等于: &'static str = ">=";
/// `<=` 比较运算符
pub const 内置运算符小于等于: &'static str = "<=";

// TODO
