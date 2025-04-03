#include "iceoryx_posh/popo/publisher.hpp"
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
    iox::runtime::PoshRuntime::initRuntime("sm_cpp_2");

    iox::capro::ServiceDescription temperature_service("sensor_data", "sm_cpp_2", "temperature");
    iox::capro::ServiceDescription humidity_service("sensor_data", "sm_cpp_2", "humidity");
    iox::capro::ServiceDescription current_service("sensor_data", "sm_cpp_2", "current");

    iox::popo::Publisher<float> pub_temperature(temperature_service);
    iox::popo::Publisher<int> pub_humidity(humidity_service);
    iox::popo::Publisher<float> pub_current(current_service);

    iox::popo::Subscriber<float> sub_temperature({"sensor_data", "sm_cpp_1", "temperature"});
    iox::popo::Subscriber<int> sub_humidity({"sensor_data", "sm_cpp_1", "humidity"});
    iox::popo::Subscriber<float> sub_current({"sensor_data", "sm_cpp_1", "current"});

    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> temp_dist(15.0, 40.0);
    std::uniform_int_distribution<int> humidity_dist(30, 80);
    std::uniform_real_distribution<float> current_dist(1.0, 10.0);

    std::cout << "[sm_cpp_2] sm_cpp_2 is ready" << std::endl;

    while (true) {
        float temperature = temp_dist(gen);
        int humidity = humidity_dist(gen);
        float current = current_dist(gen);

        pub_temperature.loan().and_then([&](auto& sample) {
            *sample = temperature;
            sample.publish();
        });

        pub_humidity.loan().and_then([&](auto& sample) {
            *sample = humidity;
            sample.publish();
        });

        pub_current.loan().and_then([&](auto& sample) {
            *sample = current;
            sample.publish();
        });

        std::cout << "[sm_cpp_2] Publishing temp: " << temperature
                  << " hum: " << humidity
                  << " current: " << current << std::endl;

        send_data(temperature, humidity, current);

        if (auto sample = sub_temperature.take()) {
            std::cout << "[sm_cpp_2] Received temp: " << **sample << std::endl;
        }
        if (auto sample = sub_humidity.take()) {
            std::cout << "[sm_cpp_2] Received hum: " << **sample << std::endl;
        }
        if (auto sample = sub_current.take()) {
            std::cout << "[sm_cpp_2] Received current: " << **sample << std::endl;
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
            {"id", "sm_cpp_2"},
            {"timestamp", std::time(nullptr)},
            {"data", {
                {"temperature", temperature},
                {"humidity", humidity},
                {"current", current}
            }}
        };

        std::string body = sensor_data.dump();

        http::request<http::string_body> req(http::verb::post, "/node/sm_cpp_2/data", 11);
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