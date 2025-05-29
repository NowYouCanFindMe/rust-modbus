# run modbus client
`cargo run -p modbus-client-bin`

To build 

`cargo build -p modbus-client-bin`


Specify modbus tcp ip and port of the server you want to connect to. 

### ip and port
```
// modbus-client-bin/src/main.rs

run_client("192.168.0.1", 503).await

```

read holding register. provide start and end register

```
    let result = ctx.read_holding_registers(register_address, 1).await?;
```

write to single register 

```
    ctx.write_single_register(register_address, value_to_write).await?;
```

### Dependencies

using tokio, tokio-modbus