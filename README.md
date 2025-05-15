# Application
- Installation using a single file ✅
- CI/CD pipeline Github ✅
- Docker image in [Docker hub](https://hub.docker.com/repositories/lp02781) ✅
- Multiple docker-compose ✅
- JSON REST API ✅

## node server
```
node_actix_container-> localhost:5000
node_websocket_container-> localhost:5000
node_tcp_container-> localhost:65432
node_sm_cpp_container-> localhost
node_sm_rust_container-> localhost
node_ros2_container-> localhost
node_mqtt_container-> localhost:1883
node_mosquitto_container-> localhost:1883
```
- Next.js frontend with React.js, Javascript, HTML, CSS
- SvelteKit frontend
- Leptos C++ WASM frontend 
- PostgreSql database

- Actix server with Rust ✅
- ROS2 rust communication (Humble) ✅
- Iceoryx sm cpp communication ✅
- Iceoryx2 sm rust communication ✅
- TCP/IP communication ✅
- Websocket communication ✅
- MQTT IoT rust communication ✅
- MQTT IoT broker Mosquitto ✅

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
docker-compose up -d websocket tcp sm_rust sm_cpp mosquitto mqtt ros2 actix 
```
