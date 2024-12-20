from controller import Robot

TIME_STEP = 32

# Fungsi Kalman Filter
def kalman_filter(z, u, x, P):
    x_pred = x + u  # Prediksi posisi
    P_pred = P + 0.1  # Noise proses
    K = P_pred / (P_pred + 1)  # Gain Kalman
    x = x_pred + K * (z - x_pred)  # Pembaruan posisi
    P = (1 - K) * P_pred  # Pembaruan ketidakpastian
    return x, P

# Inisialisasi robot
robot = Robot()

# Motor roda
left_motor = robot.getDevice("left motor")
right_motor = robot.getDevice("right motor")
left_motor.setPosition(float('inf'))  # Mode kecepatan
right_motor.setPosition(float('inf'))  # Mode kecepatan
left_motor.setVelocity(0.0)
right_motor.setVelocity(0.0)

# Sensor jarak
distance_sensors = [robot.getDevice(f"ps{i}") for i in range(8)]
for sensor in distance_sensors:
    sensor.enable(TIME_STEP)

# Variabel untuk Kalman Filter
x = 0.0  # Posisi awal
P = 1.0  # Ketidakpastian awal

# Loop utama
while robot.step(TIME_STEP) != -1:
    # Ambil nilai sensor jarak (gunakan salah satu, misalnya ps0)
    z = distance_sensors[0].getValue()  # Sensor depan
    z = z / 1000  # Normalisasi jika perlu

    # Input pergerakan u (diatur manual sebagai contoh)
    u = 0.05  # Misalnya robot bergerak maju dengan kecepatan tertentu

    # Terapkan Kalman Filter
    x, P = kalman_filter(z, u, x, P)

    # Tampilkan hasil
    print(f"Estimasi Posisi Robot: {x}")

    # Contoh penggerak robot
    left_motor.setVelocity(1.0)
    right_motor.setVelocity(1.0)
