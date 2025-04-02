#include "iceoryx_posh/popo/publisher.hpp"
#include "iceoryx_posh/runtime/posh_runtime.hpp"
#include <iostream>
#include <chrono>
#include <thread>
#include <random>  

int main() {
    iox::runtime::PoshRuntime::initRuntime("sm_1");

    iox::popo::Publisher<float> pub_temperature({"sensor_data", "sm_1", "temperature"});
    iox::popo::Publisher<int> pub_humidity({"sensor_data", "sm_1", "humidity"});
    iox::popo::Publisher<float> pub_current({"sensor_data", "sm_1", "current"});

    iox::popo::Subscriber<float> sub_temperature({"sensor_data", "sm_actix", "temperature"});
    iox::popo::Subscriber<int> sub_humidity({"sensor_data", "sm_actix", "humidity"});
    iox::popo::Subscriber<float> sub_current({"sensor_data", "sm_actix", "current"});

    std::random_device rd;
    std::mt19937 gen(rd());

    std::uniform_real_distribution<float> temp_dist(15.0, 40.0);
    std::uniform_int_distribution<int> humidity_dist(30, 80);
    std::uniform_real_distribution<float> current_dist(1.0, 10.0);

    std::cout << "sm_1 is ready" <<std::endl;
    std::cout << "sm_1 is ready" <<std::endl;
    std::cout << "sm_1 is ready" <<std::endl;

    while (true) {
        float temperature = temp_dist(gen);
        int humidity = humidity_dist(gen);
        float current = current_dist(gen);

        std::cout << "Publishing temp: " << temperature << " hum: " << humidity << " current: " << current << std::endl;

        pub_temperature.publish(temperature);  
        pub_humidity.publish(humidity);        
        pub_current.publish(current);          

        auto sample1 = sub_temperature.take();
        auto sample2 = sub_humidity.take();
        auto sample3 = sub_current.take();

        std::cout << "Receiving temp: " << sample1 << " hum: " << sample2 << " current: " << sample3 << std::endl;

        std::this_thread::sleep_for(std::chrono::seconds(10));
    }

    return 0;
}
