FROM osrf/ros:humble-desktop

WORKDIR /ros2_ws

RUN mkdir -p /ros2_ws/src
COPY . /ros2_ws/src/ros2_rust/

ENV ROS_DOMAIN_ID=42
ENV ROS_VERSION=2
ENV ROS_PYTHON_VERSION=3
ENV ROS_DISTRO=humble

RUN echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc
RUN colcon build
RUN echo "source install/setup.bash" >> ~/.bashrc

CMD ["bash", "-c", "source ~/.bashrc && ros2 launch ros2_rust haha.launch.py"]