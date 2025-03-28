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