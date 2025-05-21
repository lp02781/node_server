#include "iceoryx_posh/popo/subscriber.hpp"
#include "iceoryx_posh/runtime/posh_runtime.hpp"
#include <iostream>
#include <chrono>
#include <thread>
#include <random>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <boost/asio/connect.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <nlohmann/json.hpp>

namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using tcp = net::ip::tcp;

void send_data(float temperature, int humidity, float current);

int main() {
    iox::runtime::PoshRuntime::initRuntime("sm_cpp_sub");

    iox::popo::Subscriber<float> sub_temperature({"sensor_data", "sm_cpp_pub", "temperature"});
    iox::popo::Subscriber<int> sub_humidity({"sensor_data", "sm_cpp_pub", "humidity"});
    iox::popo::Subscriber<float> sub_current({"sensor_data", "sm_cpp_pub", "current"});

    std::cout << "[sm_cpp_sub] sm_cpp_sub is ready" << std::endl;

    while (true) {
        auto temp_sample = sub_temperature.take();
        auto hum_sample = sub_humidity.take();
        auto curr_sample = sub_current.take();

        if (!temp_sample.has_error() && !hum_sample.has_error() && !curr_sample.has_error()) { 
            float temperature = **temp_sample;
            int humidity = **hum_sample;
            float current = **curr_sample;

            std::cout << "[sm_cpp_sub] Received temp: " << temperature
                      << " hum: " << humidity
                      << " current: " << current << std::endl;

            send_data(temperature, humidity, current);
        } else {
            std::cout << "[sm_cpp_sub] Waiting for all data..." << std::endl;
        }

        std::this_thread::sleep_for(std::chrono::seconds(10));
    }

    return 0;
}

void send_data(float temperature, int humidity, float current) {
    try {
        net::io_context ioc;
        tcp::resolver resolver(ioc);
        beast::tcp_stream stream(ioc);

        auto const results = resolver.resolve("127.0.0.1", "5000");
        stream.connect(results);

        nlohmann::json sensor_data = {
            {"id", "sm_cpp"},
            {"timestamp", std::time(nullptr)},
            {"data", {
                {"temperature", temperature},
                {"humidity", humidity},
                {"current", current}
            }}
        };

        std::string body = sensor_data.dump();

        http::request<http::string_body> req(http::verb::post, "/node/sm_cpp/data", 11);
        req.set(http::field::host, "127.0.0.1");
        req.set(http::field::content_type, "application/json");
        req.body() = body;
        req.prepare_payload();

        http::write(stream, req);

        beast::flat_buffer buffer;
        http::response<http::string_body> res;
        http::read(stream, buffer, res);

        std::cout << "[HTTP Response] " << res.body() << std::endl;

        beast::error_code ec;
        stream.socket().shutdown(tcp::socket::shutdown_both, ec);
    }
    catch (const std::exception &e) {
        std::cerr << "HTTP Error: " << e.what() << std::endl;
    }
}