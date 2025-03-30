set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Restart container${NC}"
docker restart node_mosquitto_container
docker restart node_mqtt_container
docker restart node_ros2_container
docker restart node_actix_container