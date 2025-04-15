use std::net::TcpStream;
use std::io::{ErrorKind, Write, Result};


pub struct TcpConnection {
    pub live: bool,
    pub connection: TcpStream,
} 
    
impl TcpConnection {
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = match self.connection.write(buf) {
            Ok(o) => {
                if o == 0 {
                    self.live = false;
                    let _ = self.connection.shutdown(std::net::Shutdown::Both);
                }
                Ok(o)
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => {
                    self.write(buf)
            },
            e =>  {
                self.live = false;
                let _ = self.connection.shutdown(std::net::Shutdown::Both);
                e
            },
        };
        res
    }
}
