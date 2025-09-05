use std::time::{Duration, Instant};
use tokio::net::{TcpStream, lookup_host};
use crate::{RtoolsResult, RtoolsError};

/// 网络连接测试结果
#[derive(Debug)]
pub struct ConnectivityResult {
    pub host: String,
    pub port: Option<u16>,
    pub is_reachable: bool,
    pub response_time_ms: Option<u128>,
    pub error_message: Option<String>,
    pub ip_addresses: Vec<String>,
}

impl ConnectivityResult {
    pub fn new(host: String, port: Option<u16>) -> Self {
        Self {
            host,
            port,
            is_reachable: false,
            response_time_ms: None,
            error_message: None,
            ip_addresses: Vec::new(),
        }
    }
    
    pub fn print_result(&self) {
        println!("网络连接测试结果:");
        println!("- 主机: {}", self.host);
        
        if let Some(port) = self.port {
            println!("- 端口: {}", port);
        }
        
        println!("- 可达性: {}", if self.is_reachable { "✅ 可达" } else { "❌ 不可达" });
        
        if let Some(response_time) = self.response_time_ms {
            println!("- 响应时间: {} ms", response_time);
        }
        
        if !self.ip_addresses.is_empty() {
            println!("- IP地址:");
            for ip in &self.ip_addresses {
                println!("  {}", ip);
            }
        }
        
        if let Some(error) = &self.error_message {
            println!("- 错误信息: {}", error);
        }
    }
}

/// 端口扫描结果
#[derive(Debug)]
pub struct PortScanResult {
    pub host: String,
    pub open_ports: Vec<u16>,
    pub closed_ports: Vec<u16>,
    pub scan_time_ms: u128,
    pub total_ports: usize,
}

impl PortScanResult {
    pub fn new(host: String) -> Self {
        Self {
            host,
            open_ports: Vec::new(),
            closed_ports: Vec::new(),
            scan_time_ms: 0,
            total_ports: 0,
        }
    }
    
    pub fn print_result(&self) {
        println!("端口扫描结果:");
        println!("- 主机: {}", self.host);
        println!("- 扫描端口数: {}", self.total_ports);
        println!("- 开放端口数: {}", self.open_ports.len());
        println!("- 关闭端口数: {}", self.closed_ports.len());
        println!("- 扫描耗时: {} ms", self.scan_time_ms);
        
        if !self.open_ports.is_empty() {
            println!("\n开放端口:");
            for port in &self.open_ports {
                let service = get_service_name(*port);
                println!("  {} ({})", port, service);
            }
        }
    }
}

/// DNS查询结果
#[derive(Debug)]
pub struct DnsResult {
    pub domain: String,
    pub ip_addresses: Vec<String>,
    pub query_time_ms: u128,
    pub record_type: String,
}

impl DnsResult {
    pub fn new(domain: String, record_type: String) -> Self {
        Self {
            domain,
            ip_addresses: Vec::new(),
            query_time_ms: 0,
            record_type,
        }
    }
    
    pub fn print_result(&self) {
        println!("DNS查询结果:");
        println!("- 域名: {}", self.domain);
        println!("- 记录类型: {}", self.record_type);
        println!("- 查询耗时: {} ms", self.query_time_ms);
        
        if !self.ip_addresses.is_empty() {
            println!("- IP地址:");
            for ip in &self.ip_addresses {
                println!("  {}", ip);
            }
        } else {
            println!("- 未找到记录");
        }
    }
}

/// 测试TCP连接
pub async fn test_tcp_connection(host: &str, port: u16, timeout: Duration) -> RtoolsResult<ConnectivityResult> {
    let start_time = Instant::now();
    let mut result = ConnectivityResult::new(host.to_string(), Some(port));
    
    // 解析主机名
    let addr = format!("{}:{}", host, port);
    match lookup_host(&addr).await {
        Ok(addresses) => {
            for addr in addresses {
                result.ip_addresses.push(addr.ip().to_string());
                
                match tokio::time::timeout(timeout, TcpStream::connect(addr)).await {
                    Ok(Ok(_)) => {
                        result.is_reachable = true;
                        result.response_time_ms = Some(start_time.elapsed().as_millis());
                        break;
                    }
                    Ok(Err(e)) => {
                        result.error_message = Some(format!("连接失败: {}", e));
                    }
                    Err(_) => {
                        result.error_message = Some("连接超时".to_string());
                    }
                }
            }
        }
        Err(e) => {
            result.error_message = Some(format!("DNS解析失败: {}", e));
        }
    }
    
    Ok(result)
}

/// 扫描端口范围
pub async fn scan_ports(host: &str, start_port: u16, end_port: u16, timeout: Duration) -> RtoolsResult<PortScanResult> {
    let start_time = Instant::now();
    let mut result = PortScanResult::new(host.to_string());
    result.total_ports = (end_port - start_port + 1) as usize;
    
    let mut tasks = Vec::new();
    
    // 创建并发任务
    for port in start_port..=end_port {
        let host = host.to_string();
        let task = tokio::spawn(async move {
            test_tcp_connection(&host, port, timeout).await
        });
        tasks.push((port, task));
    }
    
    // 等待所有任务完成
    for (port, task) in tasks {
        match task.await {
            Ok(Ok(conn_result)) => {
                if conn_result.is_reachable {
                    result.open_ports.push(port);
                } else {
                    result.closed_ports.push(port);
                }
            }
            _ => {
                result.closed_ports.push(port);
            }
        }
    }
    
    result.scan_time_ms = start_time.elapsed().as_millis();
    Ok(result)
}

/// DNS查询
pub async fn dns_lookup(domain: &str) -> RtoolsResult<DnsResult> {
    let start_time = Instant::now();
    let mut result = DnsResult::new(domain.to_string(), "A".to_string());
    
    // 尝试不同的端口来解析域名
    let ports = vec![80, 443, 8080];
    
    for port in ports {
        let addr = format!("{}:{}", domain, port);
        match lookup_host(&addr).await {
            Ok(addresses) => {
                for addr in addresses {
                    let ip = addr.ip().to_string();
                    if !result.ip_addresses.contains(&ip) {
                        result.ip_addresses.push(ip);
                    }
                }
                if !result.ip_addresses.is_empty() {
                    break;
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
    
    if result.ip_addresses.is_empty() {
        return Err(RtoolsError::NetworkError(format!("无法解析域名: {}", domain)));
    }
    
    result.query_time_ms = start_time.elapsed().as_millis();
    Ok(result)
}

/// 获取常见端口服务名称
fn get_service_name(port: u16) -> &'static str {
    match port {
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        25 => "SMTP",
        53 => "DNS",
        80 => "HTTP",
        110 => "POP3",
        143 => "IMAP",
        443 => "HTTPS",
        993 => "IMAPS",
        995 => "POP3S",
        3306 => "MySQL",
        5432 => "PostgreSQL",
        6379 => "Redis",
        8080 => "HTTP-Alt",
        8443 => "HTTPS-Alt",
        _ => "Unknown",
    }
}

/// 测试网络连通性（类似ping）
pub async fn ping_host(host: &str, count: usize) -> RtoolsResult<Vec<ConnectivityResult>> {
    let mut results = Vec::new();
    
    for i in 0..count {
        println!("Ping {} ({}/{})", host, i + 1, count);
        
        let result = test_tcp_connection(host, 80, Duration::from_secs(5)).await?;
        results.push(result);
        
        if i < count - 1 {
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }
    
    // 打印统计信息
    let successful = results.iter().filter(|r| r.is_reachable).count();
    let total_time: u128 = results.iter()
        .filter_map(|r| r.response_time_ms)
        .sum();
    
    println!("\nPing统计:");
    println!("- 发送: {}", count);
    println!("- 接收: {}", successful);
    println!("- 丢失: {}", count - successful);
    println!("- 成功率: {:.1}%", (successful as f64 / count as f64) * 100.0);
    
    if successful > 0 {
        let avg_time = total_time / successful as u128;
        println!("- 平均响应时间: {} ms", avg_time);
    }
    
    Ok(results)
} 