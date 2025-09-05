use std::collections::HashMap;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Utc, NaiveDateTime};
use crate::{RtoolsResult, RtoolsError};

/// 日志级别
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Unknown(String),
}

impl LogLevel {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug" => Self::Debug,
            "info" => Self::Info,
            "warning" | "warn" => Self::Warning,
            "error" | "err" => Self::Error,
            "critical" | "fatal" => Self::Critical,
            _ => Self::Unknown(s.to_string()),
        }
    }
    
    pub fn severity(&self) -> u8 {
        match self {
            Self::Debug => 0,
            Self::Info => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Critical => 4,
            Self::Unknown(_) => 0,
        }
    }
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: Option<DateTime<Utc>>,
    pub level: LogLevel,
    pub message: String,
    pub source: Option<String>,
    pub line_number: Option<usize>,
}

/// 日志分析结果
#[derive(Debug, Default)]
pub struct LogAnalysis {
    pub total_entries: usize,
    pub level_distribution: HashMap<LogLevel, usize>,
    pub time_distribution: HashMap<String, usize>,
    pub error_patterns: HashMap<String, usize>,
    pub top_messages: Vec<(String, usize)>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl LogAnalysis {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn print_analysis(&self) {
        println!("日志分析结果:");
        println!("- 总条目数: {}", self.total_entries);
        
        if let Some((start, end)) = self.time_range {
            let duration = end - start;
            println!("- 时间范围: {} 到 {}", start.format("%Y-%m-%d %H:%M:%S"), end.format("%Y-%m-%d %H:%M:%S"));
            println!("- 持续时间: {} 秒", duration.num_seconds());
        }
        
        println!("\n日志级别分布:");
        let mut sorted_levels: Vec<(&LogLevel, &usize)> = self.level_distribution.iter().collect();
        sorted_levels.sort_by(|a, b| b.1.cmp(a.1));
        
        for (level, count) in sorted_levels {
            let percentage = (*count as f64 / self.total_entries as f64) * 100.0;
            println!("  {:?}: {} ({:.1}%)", level, count, percentage);
        }
        
        if !self.error_patterns.is_empty() {
            println!("\n错误模式统计:");
            let mut sorted_patterns: Vec<(&String, &usize)> = self.error_patterns.iter().collect();
            sorted_patterns.sort_by(|a, b| b.1.cmp(a.1));
            
            for (pattern, count) in sorted_patterns.iter().take(10) {
                println!("  {}: {}次", pattern, count);
            }
        }
        
        if !self.top_messages.is_empty() {
            println!("\n最常见的消息:");
            for (i, (message, count)) in self.top_messages.iter().enumerate().take(5) {
                println!("  {}. {}: {}次", i + 1, message, count);
            }
        }
    }
}

/// 分析日志文件
pub fn analyze_log_file(file_path: &str) -> RtoolsResult<LogAnalysis> {
    let path = Path::new(file_path);
    
    if !path.exists() {
        return Err(RtoolsError::FileNotFound(file_path.to_string()));
    }
    
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut analysis = LogAnalysis::new();
    let mut entries = Vec::new();
    
    for line in lines.iter() {
        if let Some(entry) = parse_log_line(line) {
            entries.push(entry);
        }
    }
    
    analysis.total_entries = entries.len();
    
    if entries.is_empty() {
        return Ok(analysis);
    }
    
    // 分析日志级别分布
    for entry in &entries {
        *analysis.level_distribution.entry(entry.level.clone()).or_insert(0) += 1;
    }
    
    // 分析时间分布
    let mut timestamps: Vec<DateTime<Utc>> = Vec::new();
    for entry in &entries {
        if let Some(ts) = entry.timestamp {
            timestamps.push(ts);
            let hour_key = ts.format("%Y-%m-%d %H").to_string();
            *analysis.time_distribution.entry(hour_key).or_insert(0) += 1;
        }
    }
    
    // 计算时间范围
    if !timestamps.is_empty() {
        timestamps.sort();
        analysis.time_range = Some((timestamps[0], timestamps[timestamps.len() - 1]));
    }
    
    // 分析错误模式
    for entry in &entries {
        if entry.level.severity() >= LogLevel::Error.severity() {
            let words: Vec<&str> = entry.message.split_whitespace().collect();
            for word in words {
                if word.len() > 3 && word.chars().any(|c| c.is_uppercase()) {
                    *analysis.error_patterns.entry(word.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    // 分析常见消息
    let mut message_counts: HashMap<String, usize> = HashMap::new();
    for entry in &entries {
        let key = entry.message.clone();
        *message_counts.entry(key).or_insert(0) += 1;
    }
    
    let mut sorted_messages: Vec<(String, usize)> = message_counts.into_iter().collect();
    sorted_messages.sort_by(|a, b| b.1.cmp(&a.1));
    analysis.top_messages = sorted_messages;
    
    Ok(analysis)
}

/// 解析单行日志
fn parse_log_line(line: &str) -> Option<LogEntry> {
    // 尝试多种常见的日志格式
    if let Some(entry) = parse_standard_format(line) {
        return Some(entry);
    }
    
    if let Some(entry) = parse_simple_format(line) {
        return Some(entry);
    }
    
    // 如果无法解析，创建一个默认条目
    Some(LogEntry {
        timestamp: None,
        level: LogLevel::Unknown("unknown".to_string()),
        message: line.to_string(),
        source: None,
        line_number: None,
    })
}

/// 解析标准格式: [2023-01-01 12:00:00] [INFO] message
fn parse_standard_format(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if !line.starts_with('[') {
        return None;
    }
    
    let parts: Vec<&str> = line.split(']').collect();
    if parts.len() < 3 {
        return None;
    }
    
    // 解析时间戳
    let timestamp_str = parts[0].trim_start_matches('[');
    let timestamp = parse_timestamp(timestamp_str);
    
    // 解析日志级别
    let level_str = parts[1].trim_start_matches('[').trim();
    let level = LogLevel::parse(level_str);
    
    // 解析消息
    let message = parts[2..].join("]").trim().to_string();
    
    Some(LogEntry {
        timestamp,
        level,
        message,
        source: None,
        line_number: None,
    })
}

/// 解析简单格式: 2023-01-01 12:00:00 INFO message
fn parse_simple_format(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    
    // 尝试解析时间戳
    let timestamp_str = format!("{} {}", parts[0], parts[1]);
    let timestamp = parse_timestamp(&timestamp_str);
    
    // 解析日志级别
    let level = LogLevel::parse(parts[2]);
    
    // 解析消息
    let message = parts[3..].join(" ");
    
    Some(LogEntry {
        timestamp,
        level,
        message,
        source: None,
        line_number: None,
    })
}

/// 解析时间戳
fn parse_timestamp(timestamp_str: &str) -> Option<DateTime<Utc>> {
    // 尝试多种时间格式
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M:%S%.3f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M:%SZ",
    ];
    
    for format in &formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(timestamp_str, format) {
            return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
        }
    }
    
    None
} 