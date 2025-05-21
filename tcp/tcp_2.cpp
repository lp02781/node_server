#include <iostream>
#include <chrono>
#include <thread>
#include <random>
#include <cstring>
#include <unistd.h>
#include <arpa/inet.h>
#include <nlohmann/json.hpp>

#define SERVER_IP "127.0.0.1"
#define SERVER_PORT 65432
#define BUFFER_SIZE 4096

int create_connection() {
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) return -1;

    sockaddr_in serv_addr{};
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(SERVER_PORT);

    if (inet_pton(AF_INET, SERVER_IP, &serv_addr.sin_addr) <= 0) {
        close(sock);
        return -1;
    }

    if (connect(sock, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0) {
        close(sock);
        return -1;
    }

    std::cout << "[tcp_2] Connected to server.\n";
    return sock;
}

int main() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> temp_dist(15.0, 40.0);
    std::uniform_int_distribution<int> humidity_dist(30, 80);
    std::uniform_real_distribution<float> current_dist(1.0, 10.0);

    int sock = -1;
    char buffer[BUFFER_SIZE];

    while (true) {
        if (sock < 0) {
            sock = create_connection();
            if (sock < 0) {
                std::cerr << "Retrying connection in 5 seconds...\n";
                std::this_thread::sleep_for(std::chrono::seconds(5));
                continue;
            }
        }

        float temperature = temp_dist(gen);
        int humidity = humidity_dist(gen);
        float current = current_dist(gen);

        nlohmann::json sensor_data = {
            {"id", "tcp_2"},
            {"timestamp", std::time(nullptr)},
            {"data", {
                {"temperature", temperature},
                {"humidity", humidity},
                {"current", current}
            }}
        };

        std::string message = sensor_data.dump();

        ssize_t sent = send(sock, message.c_str(), message.size(), 0);
        if (sent < 0) {
            std::cerr << "Send failed, reconnecting...\n";
            close(sock);
            sock = -1;
            continue;
        }

        std::cout << "[tcp_2] Sent: " << message << std::endl;

        ssize_t bytes = recv(sock, buffer, BUFFER_SIZE - 1, 0);
        if (bytes > 0) {
            buffer[bytes] = '\0';
            std::cout << "[tcp_2] Received: " << buffer << std::endl;
        } else if (bytes == 0) {
            std::cerr << "Server closed connection\n";
            close(sock);
            sock = -1;
        } else {
            std::cerr << "Receive error\n";
            close(sock);
            sock = -1;
        }

        std::this_thread::sleep_for(std::chrono::seconds(7));
    }

    return 0;
}
