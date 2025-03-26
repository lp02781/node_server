#!/usr/bin/env python3

import rclpy
from rclpy.node import Node

from std_msgs.msg import String
import json

class MinimalSubscriber(Node):

    def __init__(self):
        super().__init__('minimal_subscriber')
        self.subscription = self.create_subscription(
            String,
            '/ros2_actix/data',
            self.listener_callback,
            10)
        self.subscription  

    def listener_callback(self, msg):
        print("[Subscriber] " + msg.data)

def main(args=None):
    rclpy.init(args=args)

    minimal_subscriber = MinimalSubscriber()

    rclpy.spin(minimal_subscriber)
    minimal_subscriber.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()