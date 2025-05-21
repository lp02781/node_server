set -e
set -x

GREEN='\e[32m'

echo "${GREEN}Uninstall first${NC}"
sudo ./uninstall_node_server.sh

echo "${GREEN}Build and run docker compose${NC}"
docker-compose -f compose_node_server.yml up -d

sleep 5

echo "${GREEN} Configure database${NC}"
docker exec -i node_postgres_container psql -U admin_haha -d db_haha <<EOF
CREATE TABLE websocket (timestamp DOUBLE PRECISION, temperature REAL, humidity INTEGER, current REAL);
CREATE TABLE tcp (timestamp DOUBLE PRECISION, temperature REAL, humidity INTEGER, current REAL);
CREATE TABLE sm_cpp (timestamp DOUBLE PRECISION, temperature REAL, humidity INTEGER, current REAL);
CREATE TABLE sm_rust (timestamp DOUBLE PRECISION, temperature REAL, humidity INTEGER, current REAL);
CREATE TABLE mqtt (timestamp DOUBLE PRECISION, temperature REAL, humidity INTEGER, current REAL);
EOF

echo "${GREEN}Restart container${NC}"
sudo ./restart_node_server.sh