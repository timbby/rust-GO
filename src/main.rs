// 引用模块
mod test;
mod http;

// 引用标准库
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;


fn main() {
    // 调用test模块中的main函数
    test::main();
    // 在本地的8080端口建立TCP连接
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // 监听TCP连接
    for stream in listener.incoming() {
        // 获取tcpStream, 如果失败直接panic
        let mut stream = stream.unwrap();
        // 获取请求信息
        let request = http::handle_request(&stream);
        // 通过模式匹配确定请求正常
        let resp = match request {
            // 如果正常则返回正常结果
            Ok(method) => {
                println!("request method is {}", method);
                http::get_resp()
            },
            // 如果异常则返回异常结果
            Err(err) => {
                println!("request err: {}", err);
                http::err_resp()
            }
        };
        // 将结果写入到流中
        stream.write(resp.as_bytes()).unwrap();
        // 刷新流
        stream.flush().unwrap();
    }
}
