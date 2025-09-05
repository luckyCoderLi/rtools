use std::env;
use std::process;
use rtools::{ToolType, get_help_text, get_file_info, analyze_text_file, scan_directory, 
             search_files, SearchCriteria, analyze_log_file, ConfigManager,
             HttpRequest, HttpMethod, send_request,
             test_tcp_connection, scan_ports, dns_lookup, ping_host};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "help" => {
            print_usage();
        }
        _ => {
            if let Some(tool_type) = ToolType::parse(command) {
                handle_tool(tool_type, &args);
            } else {
                eprintln!("未知命令: {}", command);
                print_usage();
                process::exit(1);
            }
        }
    }
}

fn handle_tool(tool_type: ToolType, args: &[String]) {
    match tool_type {
        ToolType::FileInfo => {
            if args.len() != 3 {
                eprintln!("使用方法: {} fileinfo <文件路径>", args[0]);
                process::exit(1);
            }
            handle_fileinfo(&args[2]);
        }
        ToolType::TextStats => {
            if args.len() != 3 {
                eprintln!("使用方法: {} textstats <文件路径>", args[0]);
                process::exit(1);
            }
            handle_textstats(&args[2]);
        }
        ToolType::DirScan => {
            if args.len() < 3 || args.len() > 4 {
                eprintln!("使用方法: {} dirscan <目录路径> [最大深度]", args[0]);
                process::exit(1);
            }
            let max_depth = if args.len() == 4 {
                args[3].parse::<usize>().ok()
            } else {
                None
            };
            handle_dirscan(&args[2], max_depth);
        }
        ToolType::FileSearch => {
            if args.len() < 3 {
                eprintln!("使用方法: {} filesearch <目录路径> [选项]", args[0]);
                process::exit(1);
            }
            handle_filesearch(&args[2..]);
        }
        ToolType::LogAnalyzer => {
            if args.len() != 3 {
                eprintln!("使用方法: {} loganalyzer <日志文件>", args[0]);
                process::exit(1);
            }
            handle_loganalyzer(&args[2]);
        }
        ToolType::Config => {
            if args.len() < 3 {
                eprintln!("使用方法: {} config <配置文件> [操作]", args[0]);
                process::exit(1);
            }
            handle_config(&args[2..]);
        }
        ToolType::HttpClient => {
            if args.len() < 3 {
                eprintln!("使用方法: {} httpclient <URL> [选项]", args[0]);
                process::exit(1);
            }
            handle_httpclient(&args[2..]);
        }
        ToolType::Network => {
            if args.len() < 3 {
                eprintln!("使用方法: {} network <主机> [选项]", args[0]);
                process::exit(1);
            }
            handle_network(&args[2..]);
        }
    }
}

fn print_usage() {
    println!("{}", get_help_text());
}

fn handle_fileinfo(file_path: &str) {
    match get_file_info(file_path) {
        Ok(info) => println!("{}", info),
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn handle_textstats(file_path: &str) {
    match analyze_text_file(file_path) {
        Ok(stats) => {
            stats.print_stats();
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn handle_dirscan(dir_path: &str, max_depth: Option<usize>) {
    match scan_directory(dir_path, max_depth) {
        Ok(stats) => {
            stats.print_stats();
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn handle_filesearch(args: &[String]) {
    if args.is_empty() {
        eprintln!("错误: 需要指定搜索目录");
        process::exit(1);
    }
    
    let dir_path = &args[0];
    let mut criteria = SearchCriteria::new();
    
    // 解析选项
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--name" | "-n" => {
                if i + 1 < args.len() {
                    criteria = criteria.with_name_pattern(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --name 需要指定模式");
                    process::exit(1);
                }
            }
            "--ext" | "-e" => {
                if i + 1 < args.len() {
                    criteria = criteria.with_extension(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --ext 需要指定扩展名");
                    process::exit(1);
                }
            }
            "--size" => {
                if i + 1 < args.len() {
                    let size_arg = &args[i + 1];
                    if let Some(pos) = size_arg.find('-') {
                        let min = size_arg[..pos].parse::<u64>().ok();
                        let max = size_arg[pos+1..].parse::<u64>().ok();
                        criteria = criteria.with_size_range(min, max);
                    } else {
                        let size = size_arg.parse::<u64>().ok();
                        criteria = criteria.with_size_range(size, size);
                    }
                    i += 2;
                } else {
                    eprintln!("错误: --size 需要指定大小范围");
                    process::exit(1);
                }
            }
            "--depth" | "-d" => {
                if i + 1 < args.len() {
                    if let Ok(depth) = args[i + 1].parse::<usize>() {
                        criteria = criteria.with_max_depth(depth);
                        i += 2;
                    } else {
                        eprintln!("错误: --depth 需要指定数字");
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --depth 需要指定深度");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("未知选项: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    match search_files(dir_path, criteria) {
        Ok(result) => {
            result.print_results();
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn handle_loganalyzer(log_file: &str) {
    match analyze_log_file(log_file) {
        Ok(analysis) => {
            analysis.print_analysis();
        }
        Err(e) => {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
    }
}

fn handle_config(args: &[String]) {
    if args.is_empty() {
        eprintln!("错误: 需要指定配置文件");
        process::exit(1);
    }
    
    let config_file = &args[0];
    
    if args.len() == 1 {
        // 只显示配置内容
        match ConfigManager::load_from_file(config_file) {
            Ok(config) => {
                config.print_config();
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                process::exit(1);
            }
        }
    } else {
        // 处理配置操作
        let operation = &args[1];
        match operation.as_str() {
            "get" => {
                if args.len() != 3 {
                    eprintln!("使用方法: config <文件> get <键>");
                    process::exit(1);
                }
                match ConfigManager::load_from_file(config_file) {
                    Ok(config) => {
                        if let Some(value) = config.get(&args[2]) {
                            println!("{} = {:?}", args[2], value);
                        } else {
                            eprintln!("键 '{}' 不存在", args[2]);
                            process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            "set" => {
                if args.len() != 4 {
                    eprintln!("使用方法: config <文件> set <键> <值>");
                    process::exit(1);
                }
                let mut config = match ConfigManager::load_from_file(config_file) {
                    Ok(config) => config,
                    Err(_) => ConfigManager::new(),
                };
                
                let key = args[2].clone();
                let value_str = &args[3];
                
                // 尝试解析不同类型的值
                let value = if let Ok(i) = value_str.parse::<i64>() {
                    rtools::ConfigValue::Integer(i)
                } else if let Ok(f) = value_str.parse::<f64>() {
                    rtools::ConfigValue::Float(f)
                } else if let Ok(b) = value_str.parse::<bool>() {
                    rtools::ConfigValue::Boolean(b)
                } else {
                    rtools::ConfigValue::String(value_str.clone())
                };
                
                config.set(key, value);
                
                match config.save_to_file(config_file) {
                    Ok(_) => println!("配置已更新"),
                    Err(e) => {
                        eprintln!("保存错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("未知操作: {}", operation);
                eprintln!("可用操作: get, set");
                process::exit(1);
            }
        }
    }
}

fn handle_httpclient(args: &[String]) {
    if args.is_empty() {
        eprintln!("错误: 需要指定URL");
        process::exit(1);
    }
    
    let url = &args[0];
    let mut method = HttpMethod::GET;
    let mut body = None;
    let mut timeout = Duration::from_secs(30);
    
    // 解析选项
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--method" | "-m" => {
                if i + 1 < args.len() {
                    if let Some(m) = HttpMethod::parse(&args[i + 1]) {
                        method = m;
                        i += 2;
                    } else {
                        eprintln!("错误: 无效的HTTP方法: {}", args[i + 1]);
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --method 需要指定方法");
                    process::exit(1);
                }
            }
            "--body" | "-b" => {
                if i + 1 < args.len() {
                    body = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("错误: --body 需要指定内容");
                    process::exit(1);
                }
            }
            "--timeout" | "-t" => {
                if i + 1 < args.len() {
                    if let Ok(secs) = args[i + 1].parse::<u64>() {
                        timeout = Duration::from_secs(secs);
                        i += 2;
                    } else {
                        eprintln!("错误: --timeout 需要指定秒数");
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --timeout 需要指定超时时间");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("未知选项: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    // 创建运行时并执行异步函数
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut request = HttpRequest::new(method, url.to_string())
            .with_timeout(timeout);
        
        if let Some(body_content) = body {
            request = request.with_body(body_content);
        }
        
        match send_request(request).await {
            Ok(response) => {
                response.print_response();
            }
            Err(e) => {
                eprintln!("错误: {}", e);
                process::exit(1);
            }
        }
    });
}

fn handle_network(args: &[String]) {
    if args.is_empty() {
        eprintln!("错误: 需要指定主机");
        process::exit(1);
    }
    
    let host = &args[0];
    let mut operation = "connect";
    let mut port = 80;
    let mut start_port = 1;
    let mut end_port = 1024;
    let mut ping_count = 4;
    
    // 解析选项
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--ping" | "-p" => {
                operation = "ping";
                i += 1;
            }
            "--port" => {
                if i + 1 < args.len() {
                    if let Ok(p) = args[i + 1].parse::<u16>() {
                        port = p;
                        i += 2;
                    } else {
                        eprintln!("错误: --port 需要指定端口号");
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --port 需要指定端口");
                    process::exit(1);
                }
            }
            "--scan" => {
                operation = "scan";
                if i + 2 < args.len() {
                    if let (Ok(start), Ok(end)) = (args[i + 1].parse::<u16>(), args[i + 2].parse::<u16>()) {
                        start_port = start;
                        end_port = end;
                        i += 3;
                    } else {
                        eprintln!("错误: --scan 需要指定起始和结束端口");
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --scan 需要指定端口范围");
                    process::exit(1);
                }
            }
            "--dns" => {
                operation = "dns";
                i += 1;
            }
            "--count" | "-c" => {
                if i + 1 < args.len() {
                    if let Ok(count) = args[i + 1].parse::<usize>() {
                        ping_count = count;
                        i += 2;
                    } else {
                        eprintln!("错误: --count 需要指定数字");
                        process::exit(1);
                    }
                } else {
                    eprintln!("错误: --count 需要指定次数");
                    process::exit(1);
                }
            }
            _ => {
                eprintln!("未知选项: {}", args[i]);
                process::exit(1);
            }
        }
    }
    
    // 创建运行时并执行异步函数
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match operation {
            "connect" => {
                match test_tcp_connection(host, port, Duration::from_secs(10)).await {
                    Ok(result) => {
                        result.print_result();
                    }
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            "ping" => {
                match ping_host(host, ping_count).await {
                    Ok(_) => {
                        // ping_host 内部会打印结果
                    }
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            "scan" => {
                match scan_ports(host, start_port, end_port, Duration::from_secs(5)).await {
                    Ok(result) => {
                        result.print_result();
                    }
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            "dns" => {
                match dns_lookup(host).await {
                    Ok(result) => {
                        result.print_result();
                    }
                    Err(e) => {
                        eprintln!("错误: {}", e);
                        process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("未知操作: {}", operation);
                process::exit(1);
            }
        }
    });
}
