set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Uninstall first${NC}"
sudo ./uninstall_node_server.sh

echo "${GREEN}Build and run docker compose${NC}"
docker-compose -f compose_node_server.yml up -d

echo "${GREEN}Restart container${NC}"
sudo ./restart_node_server.sh