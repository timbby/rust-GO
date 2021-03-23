use std::net::TcpStream;
use std::io::Read;
use std::fs;

pub fn http_method (content: String) -> Result<&'static str, &'static str> {
    // 根据请求内容判断HTTP请求method 正确则返回包含HTTPMethod的OK
    if content.starts_with("GET") {
        Result::Ok("GET")
    } else {
        Result::Err("http method not support")
    }
}

pub fn handle_request(mut stream: &TcpStream) -> Result<&'static str, &'static str> {
    // 声明一个512字节的buffer来存放从流中读取到的数据
    let mut buffer = [0; 512];
    // 将流中的数据读到buffer中
    stream.read(&mut buffer).unwrap();
    // 将buffer中的字节转换为字符串
    let request = String::from_utf8_lossy(&buffer).to_string();
    println!("request is {}", request);
    // 调用http method检测并返回
    http_method(request)
}

pub fn get_resp() -> String {
    // 从文件读取正常返回结果
    let contents = fs::read_to_string("public/success.html").unwrap();
    // 拼接正常的HTTP相应报文
    format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
}
pub fn err_resp() -> String {
    // 从文件读取异常返回结果
    let contents = fs::read_to_string("public/404.html").unwrap();
    // 拼接异常的HTTP相应报文
    format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents)
}