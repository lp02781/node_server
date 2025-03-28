# Local Development
## Delete Everything
```
rm -rf actix/target  
```

```
docker stop node_ros2_container
docker rm node_ros2_container
docker rmi node_ros2_image
```

## Running Actix
```
cd actix/
cargo run
```

## Running ROS2
```
docker build -t node_ros2_image .
docker run -d --name node_ros2_container node_ros2_image
docker exec -it node_ros2_container bash
```

## Running MQTT


## Running Mosquitto
