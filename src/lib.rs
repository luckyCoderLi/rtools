//! Rust工具集库
//! 
//! 这个库提供了多个实用的文件系统工具，包括文件信息查看、文本统计、目录扫描、
//! 文件搜索、日志分析和配置管理功能。
//! 
//! ## 主要功能
//! 
//! - **文件信息查看**: 获取文件的详细信息，包括大小、类型、修改时间等
//! - **文本统计**: 分析文本文件的统计信息，包括字符数、单词数、词频等
//! - **目录扫描**: 递归扫描目录，统计文件信息、大小分布、文件类型等
//! - **文件搜索**: 在目录中搜索文件，支持按名称、扩展名、大小等条件过滤
//! - **日志分析**: 分析日志文件，统计日志级别、时间分布、错误模式等
//! - **配置管理**: 管理各种格式的配置文件，支持JSON、TOML、INI等格式
//! 
//! ## 使用示例
//! 
//! ```rust
//! use rtools::{get_file_info, analyze_text_file, scan_directory};
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 获取文件信息
//!     let info = get_file_info("src/main.rs")?;
//!     println!("{}", info);
//! 
//!     // 分析文本文件
//!     let stats = analyze_text_file("README.md")?;
//!     stats.print_stats();
//! 
//!     // 扫描目录
//!     let dir_stats = scan_directory("src", Some(2))?;
//!     dir_stats.print_stats();
//!     
//!     Ok(())
//! }
//! ```

pub mod fileinfo;
pub mod textstats;
pub mod dirscan;
pub mod filesearch;
pub mod loganalyzer;
pub mod config;
pub mod httpclient;
pub mod network;

// 重新导出主要功能，方便用户使用
pub use fileinfo::get_file_info;
pub use textstats::{TextStats, analyze_file as analyze_text_file};
pub use dirscan::{DirectoryStats, scan_directory};
pub use filesearch::{SearchCriteria, SearchResult, search_files};
pub use loganalyzer::{LogAnalysis, analyze_log_file};
pub use config::{ConfigManager, ConfigValue};
pub use httpclient::{HttpRequest, HttpResponse, HttpMethod, send_request, get, post, check_url};
pub use network::{ConnectivityResult, PortScanResult, DnsResult, test_tcp_connection, scan_ports, dns_lookup, ping_host};

/// 工具集的主要错误类型
#[derive(Debug, thiserror::Error)]
pub enum RtoolsError {
    #[error("文件不存在: {0}")]
    FileNotFound(String),
    
    #[error("目录不存在: {0}")]
    DirectoryNotFound(String),
    
    #[error("路径不是目录: {0}")]
    NotADirectory(String),
    
    #[error("文件不是文件: {0}")]
    NotAFile(String),
    
    #[error("权限不足: {0}")]
    PermissionDenied(String),
    
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("时间错误: {0}")]
    TimeError(#[from] std::time::SystemTimeError),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("无效参数: {0}")]
    InvalidArgument(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("网络错误: {0}")]
    NetworkError(String),
}

/// 工具集的结果类型
pub type RtoolsResult<T> = Result<T, RtoolsError>;

/// 工具类型枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolType {
    FileInfo,
    TextStats,
    DirScan,
    FileSearch,
    LogAnalyzer,
    Config,
    HttpClient,
    Network,
}

impl ToolType {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "fileinfo" => Some(Self::FileInfo),
            "textstats" => Some(Self::TextStats),
            "dirscan" => Some(Self::DirScan),
            "filesearch" => Some(Self::FileSearch),
            "loganalyzer" => Some(Self::LogAnalyzer),
            "config" => Some(Self::Config),
            "httpclient" => Some(Self::HttpClient),
            "network" => Some(Self::Network),
            _ => None,
        }
    }
    
    pub fn help_text(&self) -> &'static str {
        match self {
            Self::FileInfo => "fileinfo <文件路径>   - 显示文件信息",
            Self::TextStats => "textstats <文件路径>  - 分析文本文件统计信息",
            Self::DirScan => "dirscan <目录路径> [深度] - 扫描目录统计信息",
            Self::FileSearch => "filesearch <目录路径> [选项] - 搜索文件",
            Self::LogAnalyzer => "loganalyzer <日志文件> - 分析日志文件",
            Self::Config => "config <配置文件> - 管理配置文件",
            Self::HttpClient => "httpclient <URL> [选项] - HTTP客户端工具",
            Self::Network => "network <主机> [选项] - 网络连接测试工具",
        }
    }
    
    pub fn usage_example(&self) -> &'static str {
        match self {
            Self::FileInfo => "rtools fileinfo src/main.rs",
            Self::TextStats => "rtools textstats src/main.rs",
            Self::DirScan => "rtools dirscan src/ 2",
            Self::FileSearch => "rtools filesearch src/ --ext rs",
            Self::LogAnalyzer => "rtools loganalyzer app.log",
            Self::Config => "rtools config config.json",
            Self::HttpClient => "rtools httpclient https://api.github.com",
            Self::Network => "rtools network google.com --ping",
        }
    }
}

/// 获取所有可用工具的帮助信息
pub fn get_help_text() -> String {
    let tools = [
        ToolType::FileInfo,
        ToolType::TextStats,
        ToolType::DirScan,
        ToolType::FileSearch,
        ToolType::LogAnalyzer,
        ToolType::Config,
        ToolType::HttpClient,
        ToolType::Network,
    ];
    
    let mut help = String::from("Rust工具集 (rtools)\n\n可用命令:\n");
    
    for tool in &tools {
        help.push_str(&format!("  {}\n", tool.help_text()));
    }
    
    help.push_str("  help                 - 显示此帮助信息\n\n");
    help.push_str("示例:\n");
    
    for tool in &tools {
        help.push_str(&format!("  {}\n", tool.usage_example()));
    }
    
    help
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_type_parse() {
        assert_eq!(ToolType::parse("fileinfo"), Some(ToolType::FileInfo));
        assert_eq!(ToolType::parse("textstats"), Some(ToolType::TextStats));
        assert_eq!(ToolType::parse("dirscan"), Some(ToolType::DirScan));
        assert_eq!(ToolType::parse("filesearch"), Some(ToolType::FileSearch));
        assert_eq!(ToolType::parse("loganalyzer"), Some(ToolType::LogAnalyzer));
        assert_eq!(ToolType::parse("config"), Some(ToolType::Config));
        assert_eq!(ToolType::parse("unknown"), None);
    }

    #[test]
    fn test_tool_type_help_text() {
        assert!(ToolType::FileInfo.help_text().contains("fileinfo"));
        assert!(ToolType::TextStats.help_text().contains("textstats"));
        assert!(ToolType::DirScan.help_text().contains("dirscan"));
    }

    #[test]
    fn test_tool_type_usage_example() {
        assert!(ToolType::FileInfo.usage_example().contains("rtools"));
        assert!(ToolType::TextStats.usage_example().contains("rtools"));
        assert!(ToolType::DirScan.usage_example().contains("rtools"));
    }

    #[test]
    fn test_get_help_text() {
        let help = get_help_text();
        assert!(help.contains("Rust工具集"));
        assert!(help.contains("fileinfo"));
        assert!(help.contains("textstats"));
        assert!(help.contains("dirscan"));
        assert!(help.contains("filesearch"));
        assert!(help.contains("loganalyzer"));
        assert!(help.contains("config"));
    }

    #[test]
    fn test_error_display() {
        let error = RtoolsError::FileNotFound("test.txt".to_string());
        assert!(error.to_string().contains("文件不存在"));
        assert!(error.to_string().contains("test.txt"));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let rtools_error: RtoolsError = io_error.into();
        assert!(matches!(rtools_error, RtoolsError::IoError(_)));
    }
} 