use tokio_modbus::prelude::*;
use tokio_modbus::server::{self, Service};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

#[derive(Debug, Default)]
struct ServerService {
    holding: Arc<Mutex<[u16; 10]>>, // Shared data storage
}

impl Service for ServerService {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;

    fn call(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let mut holding = self.holding.lock().unwrap();

        match req {
            Request::ReadHoldingRegisters(addr, cnt) => {
                let data = holding[addr as usize..(addr + cnt) as usize].to_vec();
                Ok(Response::ReadHoldingRegisters(data))
            }
            Request::WriteSingleRegister(addr, val) => {
                holding[addr as usize] = val;
                Ok(Response::WriteSingleRegister(addr, val))
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported request")),
        }
    }
}

pub async fn run_server(ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    let service = ServerService::default();
    server::tcp::serve(addr, service).await?;
    Ok(())
}
