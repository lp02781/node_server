# Application
- Installation using a single file ✅
- CI/CD pipeline Github ✅
- Docker image in [Docker hub](https://hub.docker.com/repositories/lp02781) ✅
- Multiple docker-compose ✅

```
node_actix_container-> localhost:5000
node_sm_cpp_container-> localhost
node_sm_rust_container-> localhost
node_mqtt_container-> localhost:1883
node_mosquitto_container-> localhost:1883
```

## node server
- Actix backend with Rust ✅
- Next.js frontend with React.js, Javascript, HTML, CSS
- JSON REST API with protobuf
- ROS2 rust communication
- Iceoryx sm cpp communication ✅
- Iceoryx sm rust communication ✅
- gRPC communication
- jsonRPC communication
- TCP/IP communication
- Websocket communication
- MQTT IoT rust communication ✅
- MQTT IoT broker Mosquitto ✅
- PostgreSql database

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
docker-compose up -d sm_rust sm_cpp mosquitto mqtt actix 
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

docker rmi python:3.10-slim osrf/ros:foxy-desktop ubuntu:22.04 rustlang/rust:nightly
```

