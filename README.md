# Application
- Installation using a single file ✅
- CI/CD pipeline Github ✅
- Docker image in [Docker hub](https://hub.docker.com/repositories/lp02781) ✅
- Multiple docker-compose ✅
- JSON REST API ✅

## node server
```
node_actix_container-> localhost:5000    
node_interface_container-> localhost:3000    
node_websocket_container-> localhost:5000      
node_tcp_container-> localhost:65432        
node_sm_cpp_container-> localhost          
node_sm_rust_container-> localhost          
node_mqtt_container-> localhost:1883        
node_mosquitto_container-> localhost:1883
node_postgres_container-> localhost:5432 
```

- Actix server with Rust ✅
- Node.js, Next.js, React.js, Javascript, HTML, CSS
- Webassembly application
- Websocket communication ✅
- TCP/IP communication ✅
- Iceoryx sm cpp communication ✅
- Iceoryx2 sm rust communication ✅
- IoT MQTT rust communication ✅
- IoT MQTT broker Mosquitto ✅
- PostgreSQL database ✅

# Installation
```
cd deploy
sudo chmod +x install_node_server.sh
sudo ./install_node_server.sh
```

# Uninstall
```
cd deploy
sudo chmod +x uninstall_node_server.sh
sudo ./uninstall_node_server.sh
```

# Local Development

## Running 
```
sudo systemctl stop mosquitto
docker-compose up -d actix interface websocket tcp sm_cpp sm_rust mqtt mosquitto postgres
```
