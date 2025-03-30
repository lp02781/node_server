# Application
- Installation using a single file ✅
- CI/CD pipeline Github ✅
- Docker image in [Docker hub](https://hub.docker.com/repositories/lp02781) ✅
- Multiple docker-compose ✅

## [node_server](https://github.com/lp02781/node_server)
- Actix backend with Rust ✅
- Next.js frontend with React.js, Javascript, HTML, CSS
- JSON REST API with protobuf
- ROS2 rust communication
- Iceoryx shared memory communication
- gRPC communication
- jsonRPC communication
- Websocket communication
- MQTT IoT rust communication ✅
- MQTT IoT broker Mosquitto ✅
- PostgreSql database
```
node_actix_container-> localhost:5000
node_mqtt_container-> localhost:1883
node_mosquitto_container-> localhost:1883
```

# Installation
```
sudo chmod +x install.sh
sudo ./install.sh
```

# Uninstall
```
sudo chmod +x uninstall.sh
sudo ./uninstall.sh
```

# Local Development
## Delete Everything
```
rm -rf actix/target  

docker stop node_ros2_container
docker rm node_ros2_container
docker rmi node_ros2_image

docker stop node_mqtt_container
docker rm node_mqtt_container
docker rmi node_mqtt_image

docker stop node_mosquitto_container
docker rm node_mosquitto_container
docker rmi eclipse-mosquitto

docker rmi python:3.10-slim osrf/ros:foxy-desktop
```

## Running 
```
docker-compose up -d <mosquitto> <mqtt> <ros2> <actix>
```

```
cd actix
cargo run
```
