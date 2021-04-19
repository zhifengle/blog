use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use hello_web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // 多个请求不会因为 sleep 挂起。但是会用光系统资源
        //thread::spawn(|| {
        //    handle_connection(stream);
        //});

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/*
   Method Request-URI HTTP-Version CRLF
   headers CRLF
   message-body
*/

// read request
// 这里看起来是读取 stream。但是 TcpStream 在读取新的
// 需要 mut stream
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    // use std::io::prelude::*;
    // 有了上面的就能够读取
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    /*
     * Refactor
    if buffer.starts_with(get) {
        // 返回 html
        let contents = fs::read_to_string("hello.html").unwrap();
        // 响应数据
        //let response = "HTTP/1.1 200 OK\r\n\r\n";   // 改成返回 html
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        // 这里不返回 Content-Length ??
        //let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        //let response = format!("{}{}", status_line, contents);
        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    */
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
