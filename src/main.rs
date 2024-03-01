mod load_balancer_proxy;
use crate::load_balancer_proxy::LB;
use pingora::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;


// Defina uma estrutura para representar suas configurações
#[derive(Debug, Deserialize, Serialize)]
struct ServerConfig {
    address: String,
    peers: Vec<String>,
}

fn main() {
    // Carregue as configurações do servidor de um arquivo JSON
    let file = File::open("config.json").expect("Failed to open config file");
    let reader = BufReader::new(file);
    let server_configs: Vec<ServerConfig> = serde_json::from_reader(reader)
        .expect("Failed to parse config file");

    // Inicie o servidor para cada configuração
    for config in server_configs {
        let mut my_server = Server::new(None).unwrap();
        my_server.bootstrap();

        let upstreams = LoadBalancer::try_from_iter(config.peers).unwrap();
        let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
        lb.add_tcp(&config.address);

        my_server.add_service(lb);
        my_server.run_forever();
    }
}
