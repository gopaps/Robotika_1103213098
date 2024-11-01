
import heapq

def dijkstra(graph, start, goal):
    # Inisialisasi
    queue = [(0, start)]
    distances = {node: float('infinity') for node in graph}
    distances[start] = 0
    previous_nodes = {node: None for node in graph}

    while queue:
        (current_distance, current_node) = heapq.heappop(queue)
        
        # Jika mencapai node tujuan, hentikan
        if current_node == goal:
            break

        # Cek semua tetangga node saat ini
        for neighbor, weight in graph[current_node].items():
            distance = current_distance + weight

            # Jika jarak lebih pendek, perbarui jarak dan tambahkan ke antrean
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                previous_nodes[neighbor] = current_node
                heapq.heappush(queue, (distance, neighbor))

    # Rekonstruksi jalur terpendek
    path, node = [], goal
    while previous_nodes[node] is not None:
        path.append(node)
        node = previous_nodes[node]
    path.append(start)
    path.reverse()

    # Output hasil
    print("Shortest path:", path)
    print("Total cost:", distances[goal])

# Contoh graf sebagai dictionary
graph = {
    'A': {'B': 1, 'C': 4},
    'B': {'A': 1, 'C': 2, 'D': 5},
    'C': {'A': 4, 'B': 2, 'D': 1},
    'D': {'B': 5, 'C': 1}
}

# Jalankan algoritma Dijkstra
dijkstra(graph, 'A', 'D')
