# Copyright 1996-2023 Cyberbotics Ltd.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     https://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Simulation of a lidar.
"""

from controller import Robot

# Konstanta waktu langkah simulasi
TIME_STEP = 32

# Indeks untuk sensor kiri dan kanan
LEFT = 0
RIGHT = 1

# Inisialisasi robot
robot = Robot()

# Inisialisasi lidar
lidar = robot.getDevice("lidar")  # Nama perangkat LIDAR harus sama dengan konfigurasi di Webots
lidar.enable(TIME_STEP)
lidar.enablePointCloud()

# Inisialisasi sensor jarak (ultrasonic)
us = [robot.getDevice("us0"), robot.getDevice("us1")]  # Nama sensor jarak harus sama dengan konfigurasi di Webots
for sensor in us:
    sensor.enable(TIME_STEP)

# Inisialisasi motor
left_motor = robot.getDevice("left wheel motor")  # Nama motor kiri
right_motor = robot.getDevice("right wheel motor")  # Nama motor kanan
left_motor.setPosition(float('inf'))  # Aktifkan mode kecepatan
right_motor.setPosition(float('inf'))  # Aktifkan mode kecepatan
left_motor.setVelocity(0.0)
right_motor.setVelocity(0.0)

# Koefisien empiris untuk penghindaran tabrakan
coefficients = [[12.0, -6.0], [-10.0, 8.0]]
base_speed = 6.0

# Fungsi untuk membaca data lidar
def extract_lidar_data():
    lidar_data = lidar.getRangeImage()  # Mendapatkan data jarak dari LIDAR
    print(f"Lidar Data: {lidar_data[:10]}...")  # Menampilkan 10 data pertama
    return lidar_data

# Fungsi untuk membaca data dari sensor jarak
def read_distance_sensors():
    distances = [sensor.getValue() for sensor in us]  # Mendapatkan nilai jarak dari sensor ultrasonic
    print(f"Distance Sensor Readings: Left={distances[LEFT]:.2f}, Right={distances[RIGHT]:.2f}")
    return distances

# Fungsi untuk menghitung kecepatan berdasarkan data sensor
def compute_speeds(us_values):
    speed = [0.0, 0.0]  # Inisialisasi kecepatan kiri dan kanan
    for i in range(2):
        for k in range(2):
            speed[i] += us_values[k] * coefficients[i][k]  # Menghitung kecepatan berdasarkan koefisien
    return speed

# Loop utama
while robot.step(TIME_STEP) != -1:
    # Baca data lidar dan ekstrak informasi
    lidar_data = extract_lidar_data()

    # Baca data sensor jarak
    us_values = read_distance_sensors()

    # Hitung kecepatan roda berdasarkan data sensor
    speeds = compute_speeds(us_values)

    # Atur kecepatan motor
    left_motor.setVelocity(base_speed + speeds[LEFT])
    right_motor.setVelocity(base_speed + speeds[RIGHT])

# Program selesai
