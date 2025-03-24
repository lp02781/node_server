use rclrs::*;
use std::{sync::Arc, thread, time::Duration};
use std_msgs::msg::String as StringMsg;
use std::time::Duration;
use tokio;

use crate::json;

pub async fn start_ros2_publisher() {
    let context = Context::new([]).expect("Failed to create ROS 2 context");
    let node = Node::new(&context, "ros2_publisher").expect("Failed to create ROS 2 node");

    let publisher = node.create_publisher::<StringMsg>("/actix/connect", &QosProfile::default())
        .expect("Failed to create ROS 2 publisher");

        loop {
            let message = "Connect";
    
            let mut ros_msg = StringMsg::default();
            ros_msg.data = message.to_string();
    
            publisher.publish(ros_msg).expect("Failed to publish message");
    
            println!("Published to ROS 2: {}", message);
    
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
}