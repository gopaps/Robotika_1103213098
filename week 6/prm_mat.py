import matplotlib.pyplot as plt
import numpy as np

# Parameter PRM
NUM_NODES = 50          # Jumlah node yang ingin dibentuk
MAX_DISTANCE = 20       # Threshold jarak untuk menghubungkan node
START = (0, 0)          # Titik awal
GOAL = (100, 100)       # Titik tujuan

# Fungsi untuk menghasilkan node acak
def generate_random_nodes(num_nodes, x_range, y_range):
    return np.random.rand(num_nodes, 2) * [x_range, y_range]

# Fungsi untuk menghitung jarak antara dua node
def distance(node1, node2):
    return np.linalg.norm(np.array(node1) - np.array(node2))

# Fungsi utama PRM
def prm():
    nodes = [START] + generate_random_nodes(NUM_NODES, 100, 100).tolist() + [GOAL]
    edges = []

    # Membentuk graf dengan menghubungkan node
    for i, node in enumerate(nodes):
        for j in range(i + 1, len(nodes)):
            if distance(node, nodes[j]) < MAX_DISTANCE:
                edges.append((node, nodes[j]))

    return nodes, edges

# Visualisasi hasil PRM
def visualize_prm(nodes, edges):
    plt.figure()
    for edge in edges:
        plt.plot([edge[0][0], edge[1][0]], [edge[0][1], edge[1][1]], 'bo-')
    plt.plot(START[0], START[1], 'go', label="Start")
    plt.plot(GOAL[0], GOAL[1], 'ro', label="Goal")
    plt.legend()
    plt.title("Probabilistic Roadmap (PRM)")
    plt.show()

# Menjalankan PRM dan visualisasi
nodes, edges = prm()
visualize_prm(nodes, edges)
