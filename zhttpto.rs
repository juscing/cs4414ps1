//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut visitor_count: int = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            println(format!("Received request :\n{:s}", request_str));
            
            //Get the first line of the request
            let fl_items: ~[&str] = request_str.split(' ').collect();
            let mut path = fl_items[1];
            
            path = path.slice_from(1);
            
            let ext;
            if path.len() > 4 {
	      ext = path.slice_from(path.len() - 4);
	    } else {
	      ext = "null"
	    }
            
            let p = Path::new(path);
            
            //Remove this
            //print(path);
            
            let mut response: ~str = ~"";
            
            if str::eq(&~"html", &ext.into_owned()) {
            if p.is_file() {
	      match File::open(&Path::new(path)) {
	      Some(file) => {
		let mut reader = BufferedReader::new(file);
		response = response + "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n";
		for line in reader.lines() {
		  response = response + line;
		}
		
	      }
	      None =>{
		response = foo();
	      }
	    }
            } else {
	      response = foo();
            }
            } else {
	      response = foo();
            }
            
            
            
            
            stream.write(response.as_bytes());
            println!("Connection terminates.");
        }
    }
}
fn foo() -> ~str {
	unsafe {visitor_count += 1;}
	"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty " + unsafe{visitor_count.to_str()} + "!</h1>
                 </body></html>\r\n"
}
