import sys
import termios
import tty
import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from turtle_interfaces.srv import SwitchMode

msg = """
========================================
       自制高级遥控器 (终端控制版)
========================================
移动控制 (W/S):
   w : 前进
   s : 后退
   a : 左转
   d : 右转

模式切换 (0/1/2):
   0 : 边界反弹模式 (默认)
   1 : 正弦波前进模式
   2 : 方波前进模式

其他:
   空格键 : 停止海龟
   Ctrl+C : 退出遥控
========================================
"""

class TeleopNode(Node):
    def __init__(self):
        super().__init__('turtle_teleop')
        self.pub = self.create_publisher(Twist, '/turtle1/cmd_vel', 10)
        self.cli = self.create_client(SwitchMode, 'switch_mode')
        
        while not self.cli.wait_for_service(timeout_sec=1.0):
            self.get_logger().info('正在连接控制服务服务端...')
        self.get_logger().info('服务已连接，可以开始遥控。')

    def send_mode_request(self, mode):
        req = SwitchMode.Request()
        req.mode = mode
        future = self.cli.call_async(req)
        # 不使用阻塞 spin，避免死锁

def get_key(settings):
    tty.setraw(sys.stdin.fileno())
    key = sys.stdin.read(1)
    termios.tcsetattr(sys.stdin, termios.TCSADRAIN, settings)
    return key

def main():
    settings = termios.tcgetattr(sys.stdin)
    rclpy.init()
    node = TeleopNode()
    print(msg)

    # 默认线速度和角速度
    speed = 2.0
    turn = 1.5

    try:
        while True:
            key = get_key(settings)
            twist = Twist()

            # 前后左右遥控 (进入模式 3)
            if key == 'w':
                node.send_mode_request(3)
                twist.linear.x = speed
            elif key == 's':
                node.send_mode_request(3)
                twist.linear.x = -speed
            elif key == 'a':
                node.send_mode_request(3)
                twist.angular.z = turn
            elif key == 'd':
                node.send_mode_request(3)
                twist.angular.z = -turn
            elif key == ' ':
                node.send_mode_request(3)
                twist.linear.x = 0.0
                twist.angular.z = 0.0
            
            # 模式切换
            elif key in ['0', '1', '2']:
                mode = int(key)
                node.send_mode_request(mode)
                continue

            elif key == '\x03': # Ctrl+C 退出
                break
            else:
                continue

            node.pub.publish(twist)

    except Exception as e:
        print(e)
    finally:
        twist = Twist()
        node.pub.publish(twist)
        termios.tcsetattr(sys.stdin, termios.TCSADRAIN, settings)
        node.destroy_node()
        rclpy.shutdown()

if __name__ == '__main__':
    main()