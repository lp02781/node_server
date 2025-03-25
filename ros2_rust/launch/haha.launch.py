import os
from launch import LaunchDescription
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare
from launch.actions import DeclareLaunchArgument, ExecuteProcess, IncludeLaunchDescription, TimerAction
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import Command, LaunchConfiguration, PythonExpression
from launch.event_handlers import OnProcessExit
from ament_index_python.packages import get_package_share_directory

start_publisher = Node(package='ros2_rust', executable='publisher.py', name='publisher', output='screen')
start_subscriber = Node(package='ros2_rust', executable='subscriber.py', name='subscriber', output='screen')
             
def generate_launch_description():
    return LaunchDescription([
        start_publisher,
        start_subscriber 
    ])