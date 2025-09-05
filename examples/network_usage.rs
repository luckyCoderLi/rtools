use rtools::{
    HttpRequest, HttpMethod, send_request, get, post,
    test_tcp_connection, scan_ports, dns_lookup, ping_host
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 网络工具使用示例 ===\n");

    // 1. HTTP客户端示例
    println!("1. HTTP客户端示例");
    println!("-------------------");
    
    // 简单的GET请求
    println!("发送GET请求到 https://httpbin.org/get");
    match get("https://httpbin.org/get").await {
        Ok(response) => {
            println!("状态码: {}", response.status_code);
            println!("响应时间: {} ms", response.response_time_ms);
            if let Some(content_type) = response.content_type {
                println!("内容类型: {}", content_type);
            }
        }
        Err(e) => println!("GET请求失败: {}", e),
    }
    
    // POST请求
    println!("\n发送POST请求到 https://httpbin.org/post");
    let post_data = r#"{"name": "rtools", "version": "0.1.0"}"#;
    match post("https://httpbin.org/post", post_data).await {
        Ok(response) => {
            println!("状态码: {}", response.status_code);
            println!("响应时间: {} ms", response.response_time_ms);
        }
        Err(e) => println!("POST请求失败: {}", e),
    }
    
    // 自定义请求
    println!("\n发送自定义请求");
    let request = HttpRequest::new(HttpMethod::GET, "https://httpbin.org/headers".to_string())
        .with_header("User-Agent".to_string(), "Rtools/0.1.0".to_string())
        .with_header("Accept".to_string(), "application/json".to_string())
        .with_timeout(Duration::from_secs(10));
    
    match send_request(request).await {
        Ok(response) => {
            println!("状态码: {}", response.status_code);
            if response.is_success() {
                println!("请求成功!");
            }
        }
        Err(e) => println!("自定义请求失败: {}", e),
    }

    // 2. 网络连接测试示例
    println!("\n\n2. 网络连接测试示例");
    println!("---------------------");
    
    // TCP连接测试
    println!("测试TCP连接到 google.com:80");
    match test_tcp_connection("google.com", 80, Duration::from_secs(10)).await {
        Ok(result) => {
            result.print_result();
        }
        Err(e) => println!("连接测试失败: {}", e),
    }
    
    // DNS查询
    println!("\nDNS查询 google.com");
    match dns_lookup("google.com").await {
        Ok(result) => {
            result.print_result();
        }
        Err(e) => println!("DNS查询失败: {}", e),
    }
    
    // 端口扫描（小范围）
    println!("\n扫描 localhost 的常用端口 (80-90)");
    match scan_ports("localhost", 80, 90, Duration::from_secs(5)).await {
        Ok(result) => {
            result.print_result();
        }
        Err(e) => println!("端口扫描失败: {}", e),
    }
    
    // Ping测试
    println!("\nPing测试 google.com (3次)");
    match ping_host("google.com", 3).await {
        Ok(_) => {
            // ping_host 内部会打印结果
        }
        Err(e) => println!("Ping测试失败: {}", e),
    }

    // 3. 错误处理示例
    println!("\n\n3. 错误处理示例");
    println!("-----------------");
    
    // 测试不存在的域名
    println!("测试不存在的域名: nonexistent.example.com");
    match test_tcp_connection("nonexistent.example.com", 80, Duration::from_secs(5)).await {
        Ok(result) => {
            result.print_result();
        }
        Err(e) => println!("预期错误: {}", e),
    }
    
    // 测试不可达的端口
    println!("\n测试不可达的端口: localhost:9999");
    match test_tcp_connection("localhost", 9999, Duration::from_secs(5)).await {
        Ok(result) => {
            result.print_result();
        }
        Err(e) => println!("预期错误: {}", e),
    }

    println!("\n=== 示例完成 ===");
    Ok(())
} 