set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Restart container${NC}"
docker restart node_actix_container
#docker restart node_interface_container
#docker restart node_websocket_container
#docker restart node_tcp_container
#docker restart node_sm_cpp_container
#docker restart node_sm_rust_container
#docker restart node_mqtt_container
#docker restart node_mosquitto_container
#docker restart node_postgres_container