"""kf_waypoint1 controller."""

from controller import Robot
import math

# Create Robot instance
robot = Robot()

# Timestep of current world
timestep = int(robot.getBasicTimeStep())

# Helper function to safely get devices
def get_device_safe(robot, device_name):
    device = robot.getDevice(device_name)
    if device is None:
        print(f"Error: Device '{device_name}' not found. Check your .wbt file.")
    return device

# Device Setup
# Rear motors
left_motor = get_device_safe(robot, 'left_motor')
right_motor = get_device_safe(robot, 'right_motor')

# Steering motors
steer_left_motor = get_device_safe(robot, 'steer_left_motor')
steer_right_motor = get_device_safe(robot, 'steer_right_motor')

# Front motors
front_left_motor = get_device_safe(robot, 'front_left_motor')
front_right_motor = get_device_safe(robot, 'front_right_motor')

# Brakes
left_brake = get_device_safe(robot, 'left_brake')
right_brake = get_device_safe(robot, 'right_brake')

# Steering brakes
steer_left_brake = get_device_safe(robot, 'steer_left_brake')
steer_right_brake = get_device_safe(robot, 'steer_right_brake')

# Odometers
right_pos_sensor = get_device_safe(robot, 'right_pos_sensor')
left_pos_sensor = get_device_safe(robot, 'left_pos_sensor')

# Accelerometer and IMU
accelerometer = get_device_safe(robot, 'accelerometer')
imu = get_device_safe(robot, 'imu')

# Device Initialization
# Rear motors
if left_motor:
    left_motor.setPosition(float('inf'))
    left_motor.setVelocity(0.0)
if right_motor:
    right_motor.setPosition(float('inf'))
    right_motor.setVelocity(0.0)

# Front motors
if front_left_motor:
    front_left_motor.setPosition(float('inf'))
    front_left_motor.setVelocity(0.0)
if front_right_motor:
    front_right_motor.setPosition(float('inf'))
    front_right_motor.setVelocity(0.0)

# Odometers
if right_pos_sensor:
    right_pos_sensor.enable(timestep)
if left_pos_sensor:
    left_pos_sensor.enable(timestep)

# Brakes
if left_brake:
    left_brake.setDampingConstant(0)
if right_brake:
    right_brake.setDampingConstant(0)

# Steering brakes
if steer_left_brake:
    steer_left_brake.setDampingConstant(0)
if steer_right_brake:
    steer_right_brake.setDampingConstant(0)

# Accelerometer and IMU
if accelerometer:
    accelerometer.enable(timestep)
if imu:
    imu.enable(timestep)

# Variables Initialization
max_speed = 2 * math.pi
left_speed = 0
right_speed = 0
front_left_speed = 0
front_right_speed = 0
leftpos = 0
rightpos = 0

# Waypoints and other variables
waypoints = [[0.8, 0], [0, 0.8], [-0.8, 0], [0, 0]]
wp_counter = 0
num_points = len(waypoints)
currx, currz = 0, 0

# Main control loop
while robot.step(timestep) != -1:
    # Sense
    right_distance = -right_pos_sensor.getValue() * 0.025 if right_pos_sensor else 0
    left_distance = -left_pos_sensor.getValue() * 0.025 if left_pos_sensor else 0
    acc = accelerometer.getValues() if accelerometer else [0, 0, 0]
    roll_pitch_yaw = imu.getRollPitchYaw() if imu else [0, 0, 0]

    # Actuate
    if left_motor:
        left_motor.setVelocity(left_speed)
    if right_motor:
        right_motor.setVelocity(right_speed)
    if front_left_motor:
        front_left_motor.setVelocity(front_left_speed)
    if front_right_motor:
        front_right_motor.setVelocity(front_right_speed)
    if steer_left_motor:
        steer_left_motor.setPosition(leftpos)
    if steer_right_motor:
        steer_right_motor.setPosition(rightpos)

# Exit cleanup
