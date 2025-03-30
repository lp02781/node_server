set -e
set -x

docker stop node_mosquitto_container || true
docker stop node_mqtt_container || true
docker stop node_ros2_container || true
docker stop node_actix_container || true

docker rm node_mosquitto_container || true
docker rm node_mqtt_container || true
docker rm node_ros2_container || true
docker rm node_actix_container || true

docker rmi eclipse-mosquitto || true
docker rmi lp02781/node_mqtt_image || true
docker rmi lp02781/node_ros2_image || true
docker rmi lp02781/node_actix_image || true

sudo pkill mosquitto || true


