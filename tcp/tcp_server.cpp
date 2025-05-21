#include <iostream>
#include <cstring>
#include <unistd.h>
#include <arpa/inet.h>
#include <nlohmann/json.hpp>

#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <boost/asio/connect.hpp>
#include <boost/asio/ip/tcp.hpp>

#define SERVER_PORT 65432
#define BUFFER_SIZE 4096

namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using tcp = net::ip::tcp;

void forward_to_http(const nlohmann::json& data);

int main() {
    int server_fd, new_socket;
    sockaddr_in address{};
    int opt = 1;
    int addrlen = sizeof(address);
    char buffer[BUFFER_SIZE];

    if ((server_fd = socket(AF_INET, SOCK_STREAM, 0)) == 0) {
        perror("[tcp_server] socket failed");
        exit(EXIT_FAILURE);
    }

    if (setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR | SO_REUSEPORT, &opt, sizeof(opt))) {
        perror("[tcp_server] setsockopt");
        exit(EXIT_FAILURE);
    }

    address.sin_family = AF_INET;
    address.sin_addr.s_addr = INADDR_ANY;
    address.sin_port = htons(SERVER_PORT);

    if (bind(server_fd, (struct sockaddr*)&address, sizeof(address)) < 0) {
        perror("[tcp_server] bind failed");
        exit(EXIT_FAILURE);
    }

    if (listen(server_fd, 3) < 0) {
        perror("[tcp_server] listen");
        exit(EXIT_FAILURE);
    }

    std::cout << "[tcp_server] Listening on port " << SERVER_PORT << "...\n";

    while (true) {
        if ((new_socket = accept(server_fd, (struct sockaddr*)&address, (socklen_t*)&addrlen)) < 0) {
            perror("[tcp_server] accept");
            continue;
        }

        std::cout << "[tcp_server] Connection accepted.\n";

        while (true) {
            ssize_t valread = recv(new_socket, buffer, BUFFER_SIZE - 1, 0);
            if (valread <= 0) {
                if (valread == 0) {
                    std::cout << "[tcp_server] Client disconnected.\n";
                } else {
                    perror("[tcp_server] recv");
                }
                close(new_socket);
                break;
            }

            buffer[valread] = '\0';
            std::string received_msg(buffer);

            std::cout << "[tcp_server] Received: " << received_msg << "\n";

            try {
                auto json_msg = nlohmann::json::parse(received_msg);

                // Forward to HTTP
                forward_to_http(json_msg);

            } catch (std::exception& e) {
                std::cerr << "[tcp_server] Invalid JSON: " << e.what() << std::endl;
            }

            const char* response = "ok";
            ssize_t sent = send(new_socket, response, strlen(response), 0);
            if (sent < 0) {
                perror("[tcp_server] send");
                close(new_socket);
                break;
            }
        }
    }

    close(server_fd);
    return 0;
}

void forward_to_http(const nlohmann::json& data) {
    try {
        net::io_context ioc;
        tcp::resolver resolver(ioc);
        beast::tcp_stream stream(ioc);

        auto const results = resolver.resolve("127.0.0.1", "5000");
        stream.connect(results);

        std::string body = data.dump();

        http::request<http::string_body> req(http::verb::post, "/node/tcp/data", 11);
        req.set(http::field::host, "127.0.0.1");
        req.set(http::field::content_type, "application/json");
        req.body() = body;
        req.prepare_payload();

        http::write(stream, req);

        beast::flat_buffer buffer;
        http::response<http::string_body> res;
        http::read(stream, buffer, res);

        std::cout << "[tcp_server] HTTP Response: " << res.body() << std::endl;

        beast::error_code ec;
        stream.socket().shutdown(tcp::socket::shutdown_both, ec);
    }
    catch (const std::exception& e) {
        std::cerr << "[tcp_server] Error: " << e.what() << std::endl;
    }
}
