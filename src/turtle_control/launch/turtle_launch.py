import os
from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch_ros.actions import Node

def generate_launch_description():
    # 1. 获取 YAML 配置文件路径
    pkg_share = get_package_share_directory('turtle_control')
    # 兼容 colcon 安装目录结构与本地开发路径
    params_file = os.path.join(pkg_share, 'config', 'params.yaml')

    # 2. 定义小海龟模拟器节点
    turtlesim_node = Node(
        package='turtlesim',
        executable='turtlesim_node',
        name='turtlesim'
    )

    # 3. 定义我们自编的核心运动控制节点（读取 YAML）
    controller_node = Node(
        package='turtle_control',
        executable='turtle_controller',
        name='turtle_controller',
        parameters=[params_file],
        output='screen'
    )

    # 4. 遥控器节点需要终端支持输入
    teleop_node = Node(
        package='turtle_control',
        executable='turtle_teleop',
        name='turtle_teleop',
        output='screen',
        emulate_tty=True
    )

    return LaunchDescription([
        turtlesim_node,
        controller_node,
        teleop_node
    ])
