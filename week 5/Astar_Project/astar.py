import heapq

# Fungsi heuristic menggunakan jarak Manhattan
def heuristic(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def a_star(grid, start, goal):
    # Inisialisasi struktur data untuk A*
    open_set = []
    heapq.heappush(open_set, (0, start))
    came_from = {}
    g_score = {start: 0}
    f_score = {start: heuristic(start, goal)}

    while open_set:
        # Ambil node dengan nilai f_score terkecil
        current = heapq.heappop(open_set)[1]

        # Jika mencapai goal, rekonstruksi jalur
        if current == goal:
            path = []
            while current in came_from:
                path.append(current)
                current = came_from[current]
            path.append(start)
            path.reverse()
            return path

        # Cek tetangga dari node saat ini
        neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        for dx, dy in neighbors:
            neighbor = (current[0] + dx, current[1] + dy)
            tentative_g_score = g_score[current] + 1

            # Validasi posisi tetangga dalam grid dan apakah ada rintangan
            if 0 <= neighbor[0] < len(grid) and 0 <= neighbor[1] < len(grid[0]):
                if grid[neighbor[0]][neighbor[1]] == 1:
                    continue  # Abaikan jika ada rintangan

                # Update g_score dan f_score jika ditemukan jalur lebih baik
                if tentative_g_score < g_score.get(neighbor, float('inf')):
                    came_from[neighbor] = current
                    g_score[neighbor] = tentative_g_score
                    f_score[neighbor] = tentative_g_score + heuristic(neighbor, goal)
                    heapq.heappush(open_set, (f_score[neighbor], neighbor))

    return None  # Jika tidak ada jalur

# Definisikan grid dengan 0 sebagai jalan bebas dan 1 sebagai rintangan
grid = [
    [0, 0, 0, 0, 0],
    [1, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0]
]

# Titik awal dan tujuan
start = (0, 0)
goal = (4, 4)

# Jalankan algoritma A*
path = a_star(grid, start, goal)
if path:
    print("Path found:", path)
else:
    print("No path found.")
