#include <iostream>
#include <cstring>
#include <unistd.h>
#include <arpa/inet.h>
#include <nlohmann/json.hpp>

#define SERVER_PORT 65432
#define BUFFER_SIZE 4096

int main() {
    int server_fd, new_socket;
    sockaddr_in address{};
    int opt = 1;
    int addrlen = sizeof(address);
    char buffer[BUFFER_SIZE];

    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT,
                   &opt, sizeof(opt))) {
        perror("setsockopt");
        exit(EXIT_FAILURE);
    }

    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY; // Listen on any IP
    address.sin_port = htons(SERVER_PORT);

    if (bind(server_fd, (struct sockaddr*)&address, sizeof(address)) < 0) {
        perror("bind failed");
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, 3) < 0) {
        perror("listen");
        exit(EXIT_FAILURE);
    }

    std::cout << "[tcp_server] Listening on port " << SERVER_PORT << "...\n";

    while (true) {
        if ((new_socket = accept(server_fd, (struct sockaddr*)&address,
                                 (socklen_t*)&addrlen)) < 0) {
            perror("accept");
            continue;
        }

        std::cout << "[tcp_server] Connection accepted.\n";

        while (true) {
            ssize_t valread = recv(new_socket, buffer, BUFFER_SIZE - 1, 0);
            if (valread <= 0) {
                if (valread == 0) {
                    std::cout << "[tcp_server] Client disconnected.\n";
                } else {
                    perror("recv");
                }
                close(new_socket);
                break;
            }

            buffer[valread] = '\0';
            std::string received_msg(buffer);

            std::cout << "[tcp_server] Received: " << received_msg << std::endl;

            try {
                auto json_msg = nlohmann::json::parse(received_msg);
            } 
            catch (std::exception& e) {
                std::cerr << "[tcp_server] Invalid JSON received: " << e.what() << std::endl;
            }

            const char* response = "ok";
            ssize_t sent = send(new_socket, response, strlen(response), 0);
            if (sent < 0) {
                perror("send");
                close(new_socket);
                break;
            }
        }
    }

    close(server_fd);
    return 0;
}
