# Application
- Installation using a single file ✅
- CI/CD pipeline Github ✅
- Docker image in [Docker hub](https://hub.docker.com/repositories/lp02781) ✅
- Multiple docker-compose ✅
- JSON REST API ✅
```
node_actix_container-> localhost:5000
node_tcp_container-> localhost:65432
node_sm_cpp_container-> localhost
node_sm_rust_container-> localhost
node_ros2_container-> localhost
node_mqtt_container-> localhost:1883
node_mosquitto_container-> localhost:1883
```

## interface server
- Node.js server with Javascript
- Next.js frontend with React.js, Javascript, HTML, CSS 

## database server
- Django server with Python
- PostgreSql database
- Pytorch machine learning

## node server
- Actix server with Rust ✅
- ROS2 rust communication (Humble)
- Iceoryx sm cpp communication ✅
- Iceoryx2 sm rust communication ✅
- TCP/IP communication ✅
- Websocket communication
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
docker-compose up -d tcp sm_rust sm_cpp mosquitto mqtt actix 
```

```
cd actix
cargo run
```

## Delete Everything
```
rm -rf actix/target  

docker stop node_actix_container
docker rm node_actix_container
docker rmi node_actix_image

docker stop node_tcp_container
docker rm node_tcp_container
docker rmi node_tcp_image

docker stop node_mqtt_container
docker rm node_mqtt_container
docker rmi node_mqtt_image

docker stop node_mosquitto_container
docker rm node_mosquitto_container
docker rmi eclipse-mosquitto

docker stop node_sm_cpp_container
docker rm node_sm_cpp_container
docker rmi node_sm_cpp_image

docker stop node_sm_rust_container
docker rm node_sm_rust_container
docker rmi node_sm_rust_image

docker stop node_ros2_container
docker rm node_ros2_container
docker rmi node_ros2_image

docker rmi python:3.10-slim 
docker rmi osrf/ros:foxy-desktop 
docker rmi ubuntu:22.04 
docker rmi rustlang/rust:nightly
```

