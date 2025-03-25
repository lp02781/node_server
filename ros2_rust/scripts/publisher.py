import rclpy
from rclpy.node import Node

from std_msgs.msg import String

import time
import random

class MinimalPublisher(Node):

    def __init__(self):
        super().__init__('minimal_publisher')
        timer_period = 10  
        self.publisher_1 = self.create_publisher(String, '/ros2_1/data', 10)
        self.publisher_2 = self.create_publisher(String, '/ros2_2/data', 10)
        self.timer1 = self.create_timer(timer_period, self.timer_callback_1)
        self.timer2 = self.create_timer(timer_period, self.timer_callback_2)

    def timer_callback_1(self):
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": "ros_1",
            "timestamp": time.time(),
            "data": {  
                "temperature": temperature,
                "humidity": humidity,
                "current": current 
            }
        }

        message = json.dumps(sensor_data)
        
        msg = String()
        msg.data = message 
        self.publisher_1.publish(msg)

        print(msg)
    
    def timer_callback_1(self):
        temperature = random.uniform(15, 40)
        humidity = random.randint(30, 80)
        current = random.uniform(1, 10)
        sensor_data = {
            "id": "ros_2",
            "timestamp": time.time(),
            "data": {  
                "temperature": temperature,
                "humidity": humidity,
                "current": current 
            }
        }

        message = json.dumps(sensor_data)
        
        msg = String()
        msg.data = message 
        self.publisher_2.publish(msg)

        print(msg)

def main(args=None):
    rclpy.init(args=args)

    minimal_publisher = MinimalPublisher()

    rclpy.spin(minimal_publisher)

    minimal_publisher.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()