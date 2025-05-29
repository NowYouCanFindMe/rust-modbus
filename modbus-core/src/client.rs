use tokio_modbus::prelude::*;
use std::net::SocketAddr;

pub async fn run_client(ip: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Modbus TCP server at 192.168.0.1:503
    let addr: SocketAddr = format!("{}:{}", ip, port).parse()?;
    let mut ctx = tcp::connect(addr).await?;
    
    // Write value 1234 to holding register at address 0x0001
    let register_address = 0x0003;
    let value_to_write = 1234;
    let start_addr = 0;
    let end_addr = 60;

    let result = ctx.read_holding_registers(register_address, 1).await?;
    println!("Read back value: {:?}", result);

    ctx.write_single_register(register_address, value_to_write).await?;
    println!("Wrote value {} to register {}", value_to_write, register_address);

    let data = ctx.read_holding_registers(start_addr, end_addr).await?;
    println!("Response: {:?}", data);
    Ok(())
}
