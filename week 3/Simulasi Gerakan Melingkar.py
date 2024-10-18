from controller import Robot  # Mengimpor kelas Robot dari modul controller

# Membuat instance Robot
robot = Robot()

# Mengambil nilai timestep dari dunia simulasi saat ini
timestep = int(robot.getBasicTimeStep())

# Mendapatkan motor untuk roda kiri dan roda kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Mengatur motor ke mode kecepatan (velocity mode)
left_motor.setPosition(float('inf'))  # Mengatur posisi tak terbatas
right_motor.setPosition(float('inf'))

# Mengatur kecepatan roda kiri lebih lambat daripada roda kanan agar robot berbelok
kecepatan_kiri = 2.0  # Kecepatan lebih lambat untuk roda kiri
kecepatan_kanan = 4.0  # Kecepatan lebih cepat untuk roda kanan
left_motor.setVelocity(kecepatan_kiri)
right_motor.setVelocity(kecepatan_kanan)

# Loop utama: melakukan simulasi hingga Webots menghentikan kontrol
while robot.step(timestep) != -1:
    pass  # Robot akan bergerak dalam pola lingkaran
