fn main() { 
   use std::thread; 
   use std::time::Duration; 
   use std::net::TcpStream; 
   
   loop { 
       match TcpStream::connect("google.com:80") { 
           Ok(_) => { 
               println!("connected"); 
               break; 
           } 
           Err(e) => { 
               println!("failed: {:?}", e); 
           } 
        } 
        thread::sleep(Duration::from_secs(1)); 
    } 
}
