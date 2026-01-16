mod app;
mod proto;
mod grpc;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut descriptor_path: Option<String> = None;
    let mut addr: String = "http://127.0.0.1:50051".to_string();
    let mut service: Option<String> = None;
    let mut method: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--descriptor" => {
                descriptor_path = args.get(i + 1).cloned();
                i += 2;
            }
            "--addr" => {
                addr = args.get(i + 1).cloned().unwrap_or(addr);
                i += 2;
            }
            "--service" => {
                service = args.get(i + 1).cloned();
                i += 2;
            }
            "--method" => {
                method = args.get(i + 1).cloned();
                i += 2;
            }
            _ => {
                eprintln!("unknown argument: {}", args[i]);
                print_usage();
                std::process::exit(1);
            }
        }
    }

    let descriptor_path = match descriptor_path {
        Some(p) => p,
        None => {
            print_usage();
            std::process::exit(1);
        }
    };

    let service = match service {
        Some(s) => s,
        None => {
            print_usage();
            std::process::exit(1);
        }
    };

    let method = match method {
        Some(m) => m,
        None => {
            print_usage();
            std::process::exit(1);
        }
    };

    let config = app::AppConfig {
        descriptor_path,
        addr,
        service,
        method,
    };

    if let Err(e) = app::run(config).await {
        eprintln!("error: {}", e);
    }
}

fn print_usage() {
    eprintln!(
        r#"
usage:
  rpcsnap \
    --descriptor <descriptor.bin> \
    --service <package.Service> \
    --method <Method> \
    [--addr <http://host:port>]

example:
  rpcsnap \
    --descriptor ping.bin \
    --service demo.v1.PingService \
    --method Ping \
    --addr http://127.0.0.1:50051
"#
    );
}
