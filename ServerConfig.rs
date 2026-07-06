/*
Задача 6: Builder Pattern для конфигурации Реализуйте Шаблон Строитель 
для структуры ServerConfig с полями: host, port, max_connections, timeout.
 */

 #[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u32,
    max_connections: u32,
    timeout: u32,
}

impl ServerConfig {
    fn new(host: String, port: u32, max_connections: u32, timeout: u32) -> Self {
        Self {
            host,
            port,
            max_connections,
            timeout,
        }
    }
}

impl ServerConfig {
    fn add_host(&mut self, host: String) -> &mut Self {
        self.host = host;
        self
    }

    fn add_port(&mut self, port: u32) -> &mut Self {
        self.port = port;
        self
    }

    fn add_max_connections(&mut self, max_connections: u32) -> &mut Self {
        self.max_connections = max_connections;
        self
    }

    fn add_timeout(&mut self, timeout: u32) -> &mut Self {
        self.timeout = timeout;
        self
    }
}

fn main() {
    let mut server = ServerConfig::new("localhost".to_string(), 0000, 0, 0);
    println!("{:#?}", server.add_host("1.888.1.888".to_string()).add_port(8000).add_max_connections(100).add_timeout(30));
}