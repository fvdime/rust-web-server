use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use server::ThreadPool;

fn main() {
    // creating new listener
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
      // unwrap gonna panic if there is any error
      let stream = stream.unwrap();

      pool.execute(|| {
        handle_connection(stream)
      });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // long enough for basic reqs
    let mut buffer = [0; 1024];

    // This is a mutable variable binding. When a binding is mutable, it means you're allowed to change what the binding points to
    stream.read(&mut buffer).unwrap();
    // println!(
    //     "Request: {}",
    //     String::from_utf8_lossy(&buffer[..])
    // )

    let get = b"GET / HTTP/1.1\r\n";

    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = 
      if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "index.html")
      } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
      } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
      };

    // let status_line = "HTTP/1.1 404 NOT FOUND";

    let content = fs::read_to_string(filename).unwrap();
    let response = format!(
      "{}\r\nContent Lenght: {}\r\n\r\n{}",
      status_line,
      content.len(),
      content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}