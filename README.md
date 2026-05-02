# ROS 2 小海龟高级控制项目 (`dev_turtle`)

这是一个基于 **ROS 2 Humble** 开发的小海龟（Turtlesim）高级控制项目。项目实现了自定义服务接口通讯、多运动模式切换、自动边缘避障以及键盘遥控无缝接管等功能。

## 📌 项目功能特性

* **多运动模式切换**：通过自定义服务 `SwitchMode` 动态切换小海龟的运动状态：
  * **模式 0（默认）**：直线前进，碰到边界时自动反弹转向。
  * **模式 1**：正弦波运动（走 "S" 形轨迹）。
  * **模式 2**：方波运动（走直角折线轨迹）。
* **键盘遥控接管（模式 3）**：通过键盘节点发布控制指令，随时打断自动运行模式，转为手动遥控。
* **参数动态配置**：支持在线修改小海龟的线速度等参数，无需重启节点。

---

## 📂 项目目录结构

```text
dev_turtle/
├── src/
│   ├── turtle_interfaces/           # 自定义服务接口包
│   │   ├── srv/
│   │   │   └── SwitchMode.srv       # 模式切换服务定义 (int32 mode -> bool success, string message)
│   │   ├── CMakeLists.txt
│   │   └── package.xml
│   └── turtle_control/              # 核心控制与遥控功能包
│       ├── launch/
│       │   └── turtle_launch.py     # 一键启动仿真和控制器
│       ├── turtle_control/
│       │   ├── turtle_controller.py # 核心控制器节点
│       │   └── turtle_teleop.py     # 键盘遥控节点
│       ├── setup.py
│       └── package.xml
└── .gitignore                       # Git 忽略规则
