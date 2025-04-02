#include "iceoryx_posh/popo/publisher.hpp"
#include "iceoryx_posh/popo/subscriber.hpp"
#include "iceoryx_posh/runtime/posh_runtime.hpp"
#include <iostream>
#include <chrono>
#include <thread>
#include <random>

int main() {
    iox::runtime::PoshRuntime::initRuntime("sm_cpp_1");

    iox::capro::ServiceDescription temperature_service("sensor_data", "sm_cpp_1", "temperature");
    iox::capro::ServiceDescription humidity_service("sensor_data", "sm_cpp_1", "humidity");
    iox::capro::ServiceDescription current_service("sensor_data", "sm_cpp_1", "current");

    iox::popo::Publisher<float> pub_temperature(temperature_service);
    iox::popo::Publisher<int> pub_humidity(humidity_service);
    iox::popo::Publisher<float> pub_current(current_service);

    iox::popo::Subscriber<float> sub_temperature({"sensor_data", "sm_cpp_2", "temperature"});
    iox::popo::Subscriber<int> sub_humidity({"sensor_data", "sm_cpp_2", "humidity"});
    iox::popo::Subscriber<float> sub_current({"sensor_data", "sm_cpp_2", "current"});

    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<float> temp_dist(15.0, 40.0);
    std::uniform_int_distribution<int> humidity_dist(30, 80);
    std::uniform_real_distribution<float> current_dist(1.0, 10.0);

    std::cout << "[sm_cpp_1] sm_cpp_1 is ready" << std::endl;

    while (true) {
        float temperature = temp_dist(gen);
        int humidity = humidity_dist(gen);
        float current = current_dist(gen);

        pub_temperature.loan()
            .and_then([&](auto& sample) {
                *sample = temperature;
                sample.publish();
            });

        pub_humidity.loan()
            .and_then([&](auto& sample) {
                *sample = humidity;
                sample.publish();
            });

        pub_current.loan()
            .and_then([&](auto& sample) {
                *sample = current;
                sample.publish();
            });

        std::cout << "[sm_cpp_1] Publishing temp: " << temperature 
                  << " hum: " << humidity 
                  << " current: " << current << std::endl;

        if (auto sample = sub_temperature.take()) {
            std::cout << "[sm_cpp_1] Received temp: " << **sample << std::endl;
        }
        if (auto sample = sub_humidity.take()) {
            std::cout << "[sm_cpp_1] Received hum: " << **sample << std::endl;
        }
        if (auto sample = sub_current.take()) {
            std::cout << "[sm_cpp_1] Received current: " << **sample << std::endl;
        }

        std::this_thread::sleep_for(std::chrono::seconds(10));
    }

    return 0;
}
