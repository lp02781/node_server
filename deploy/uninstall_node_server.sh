set -e
set -x

docker stop node_tcp_container || true
docker stop node_sm_rust_container || true
docker stop node_sm_cpp_container || true
docker stop node_mosquitto_container || true
docker stop node_mqtt_container || true
docker stop node_ros2_container || true
docker stop node_actix_container || true

docker rm node_tcp_container || true
docker rm node_sm_rust_container || true
docker rm node_sm_cpp_container || true
docker rm node_mosquitto_container || true
docker rm node_mqtt_container || true
docker rm node_ros2_container || true
docker rm node_actix_container || true

docker rmi lp02781/node_tcp_image || true
docker rmi lp02781/node_sm_rust_image || true
docker rmi lp02781/node_sm_cpp_image || true
docker rmi eclipse-mosquitto || true
docker rmi lp02781/node_mqtt_image || true
docker rmi lp02781/node_ros2_image || true
docker rmi lp02781/node_actix_image || true

sudo pkill mosquitto || true
