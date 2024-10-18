from controller import Robot

TIME_STEP = 64
MAX_SPEED = 6.28
DELAY_STEPS = 20  # Penundaan awal

robot = Robot()

# Inisialisasi motor
leftMotor = robot.getDevice('left wheel motor')
rightMotor = robot.getDevice('right wheel motor')

# Inisialisasi sensor proximity
proximitySensor = robot.getDevice('ps0')  # Sensor depan harus sesuai dengan nama perangkat

# Atur posisi motor agar berjalan terus
leftMotor.setPosition(float('inf'))
rightMotor.setPosition(float('inf'))

# Aktifkan sensor proximity
print("Sensor proximity diaktifkan")
proximitySensor.enable(TIME_STEP)

# Penundaan awal sebelum menggunakan sensor proximity
for _ in range(DELAY_STEPS):
    leftMotor.setVelocity(MAX_SPEED)
    rightMotor.setVelocity(MAX_SPEED)
    robot.step(TIME_STEP)

# Loop utama untuk deteksi objek menggunakan sensor
while robot.step(TIME_STEP) != -1:
    proximityValue = proximitySensor.getValue()
    print("Nilai Sensor Proximity:", proximityValue)

    if proximityValue < 100:  # Jika objek terdeteksi lebih dekat dari threshold
        print("Objek terdeteksi, robot berhenti.")
        leftMotor.setVelocity(0)
        rightMotor.setVelocity(0)  # Hentikan robot
    else:
        leftMotor.setVelocity(MAX_SPEED)
        rightMotor.setVelocity(MAX_SPEED)  # Bergerak maju jika tidak ada halangan