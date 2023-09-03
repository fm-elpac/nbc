//! 命令行参数和环境变量处理

use std::env;
//use std::path::PathBuf;

use log::debug;

use libnbc::r::运行参数;

// 编译信息
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

/// 显示版本信息
pub fn 版本() {
    let name = env!("CARGO_PKG_NAME");
    let v = env!("CARGO_PKG_VERSION");
    let target = built_info::TARGET;
    //let features = built_info::FEATURES_LOWERCASE_STR;
    println!("{} 版本 {} ({})", name, v, target);

    // debug
    let git = env!("VERGEN_GIT_DESCRIBE");
    let profile = built_info::PROFILE;
    let time = env!("VERGEN_BUILD_TIMESTAMP");
    let rustc = built_info::RUSTC_VERSION;
    debug!("{} {} {}, {}", git, profile, time, rustc);
}

/// 显示帮助信息
pub fn 帮助() {
    let url = env!("CARGO_PKG_REPOSITORY");

    println!("命令行帮助信息:");
    // TODO
    println!("<{}>", url);
}

/// 命令行参数和环境变量的处理结果
#[derive(Debug)]
pub enum Cea {
    /// 错误的命令行参数
    错误(String),

    /// 显示版本信息
    版本,

    /// 显示帮助信息
    帮助,

    /// 运行 nbc
    运行(运行参数),
}

impl Cea {
    pub fn 处理() -> Self {
        let mut 版本 = false;
        let mut 帮助 = false;

        let mut a = 运行参数::default();

        // 处理每个命令行参数
        for i in env::args().skip(1) {
            match i.as_str() {
                "--version" | "--版本" => {
                    版本 = true;
                }
                "--help" | "--帮助" => {
                    帮助 = true;
                }

                // 输入的源代码文件路径
                _ => {
                    a.源文件 = i;
                }
            }
        }

        // TODO 处理环境变量
        // if let Ok(i) = env::var("FMLSD") {
        //     let p = i.trim();
        //     if p.len() > 0 {
        //         debug!("FMLSD={}", p);
        //         a.fmlsd = PathBuf::from(p);
        //     }
        // }
        // TODO 处理配置文件

        if 帮助 {
            Self::帮助
        } else if 版本 {
            Self::版本
        } else {
            if a.源文件.len() < 1 {
                return Self::错误("命令行格式错误, 查看帮助使用 --help".into());
            }

            Self::运行(a)
        }
    }
}
