FROM osrf/ros:foxy-desktop

WORKDIR /ros2_ws

RUN mkdir -p /ros2_ws/src
COPY . /ros2_ws/src/ros2_rust/

ENV ROS_DOMAIN_ID=42
ENV ROS_VERSION=2
ENV ROS_PYTHON_VERSION=3
ENV ROS_DISTRO=foxy

RUN apt install -y python3-colcon-common-extensions

CMD ["bash", "-c", "source /opt/ros/foxy/setup.bash && colcon build && source install/setup.bash && ros2 launch ros2_rust haha.launch.py"]
