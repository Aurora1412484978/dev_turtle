import math
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from turtlesim.msg import Pose
from turtle_interfaces.srv import SwitchMode

class TurtleController(Node):
    def __init__(self):
        super().__init__('turtle_controller')
        
        # 1. 声明并读取参数
        self.declare_parameter('linear_vel', 2.0)
        self.declare_parameter('angular_vel', 1.5)
        self.declare_parameter('boundary_limit', 1.5)

        # 2. 发布者和订阅者
        self.cmd_pub = self.create_publisher(Twist, '/turtle1/cmd_vel', 10)
        self.pose_sub = self.create_subscription(Pose, '/turtle1/pose', self.pose_callback, 10)

        # 3. 运动状态变量
        self.current_pose = None
        self.mode = 0  # 0: 正常边界反弹, 1: 正弦前进, 2: 方波前进, 3: 遥控中断
        self.turning = False
        self.start_time = self.get_clock().now()

        # 4. 服务端
        self.srv = self.create_service(SwitchMode, 'switch_mode', self.switch_mode_callback)

        # 5. 主控制循环：10Hz
        self.timer = self.create_timer(0.1, self.control_loop)
        self.get_logger().info("小海龟高级控制器节点已启动，当前为默认运动模式（0）")

    def pose_callback(self, msg):
        self.current_pose = msg

    def switch_mode_callback(self, request, response):
        if request.mode in [0, 1, 2, 3]:
            self.mode = request.mode
            self.start_time = self.get_clock().now() # 重置时间基准
            response.success = True
            response.message = f"成功切换到运动模式: {self.mode}"
        else:
            response.success = False
            response.message = f"未知的模式代码: {request.mode}，切换失败。"
        self.get_logger().info(response.message)
        return response

    def control_loop(self):
        if self.current_pose is None:
            return

        # 动态刷新配置参数
        v_limit = self.get_parameter('linear_vel').value
        w_limit = self.get_parameter('angular_vel').value
        b_limit = self.get_parameter('boundary_limit').value

        twist = Twist()

        # 模式 3 为遥控中断模式，不在此计算自动轨迹
        if self.mode == 3:
            return

        # 模式 0：默认模式。沿直线向前走，遇到边界自动转弯
        if self.mode == 0:
            x, y = self.current_pose.x, self.current_pose.y
            
            # 边界碰撞检测 (默认 11.0x11.0 大小)
            if x < b_limit or x > (11.0 - b_limit) or y < b_limit or y > (11.0 - b_limit):
                self.turning = True
            else:
                self.turning = False

            if self.turning:
                twist.linear.x = 0.5  # 慢速试探
                twist.angular.z = w_limit # 原地打转掉头
            else:
                twist.linear.x = v_limit
                twist.angular.z = 0.0

        # 模式 1：正弦式前进
        elif self.mode == 1:
            elapsed = (self.get_clock().now() - self.start_time).nanoseconds / 1e9
            twist.linear.x = v_limit
            twist.angular.z = w_limit * math.sin(2 * math.pi * 0.5 * elapsed) # 周期 2 秒的正弦摆动

        # 模式 2：方波式前进
        elif self.mode == 2:
            elapsed = (self.get_clock().now() - self.start_time).nanoseconds / 1e9
            twist.linear.x = v_limit
            # 每隔 1 秒交替改变转向，模拟方波
            if int(elapsed) % 2 == 0:
                twist.angular.z = w_limit
            else:
                twist.angular.z = -w_limit

        self.cmd_pub.publish(twist)

def main(args=None):
    rclpy.init(args=args)
    node = TurtleController()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()
