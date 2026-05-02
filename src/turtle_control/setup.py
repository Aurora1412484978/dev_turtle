import os
from glob import glob
from setuptools import find_packages, setup

package_name = 'turtle_control'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages', ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        # 安装 launch 文件夹下的所有 python launch 脚本
        (os.path.join('share', package_name, 'launch'), glob('launch/*.py')),
        # 安装 config 文件夹下的所有 yaml 参数配置文件
        (os.path.join('share', package_name, 'config'), glob('config/*.yaml')),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='your_name',
    maintainer_email='your_email@todo.todo',
    description='TODO: Package description',
    license='TODO: License declaration',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'turtle_controller = turtle_control.turtle_controller:main',
            'turtle_teleop = turtle_control.turtle_teleop:main',
        ],
    },
)
