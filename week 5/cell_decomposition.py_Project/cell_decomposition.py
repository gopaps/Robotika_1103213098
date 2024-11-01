import numpy as np
import matplotlib.pyplot as plt
from queue import Queue

class CellDecomposition:
    def __init__(self, grid_size, obstacles):
        self.grid_size = grid_size
        self.grid = np.zeros(grid_size)
        self.set_obstacles(obstacles)

    def set_obstacles(self, obstacles):
        for (x, y) in obstacles:
            self.grid[x, y] = 1  # Mark as obstacle

    def is_free(self, x, y):
        return 0 <= x < self.grid_size[0] and 0 <= y < self.grid_size[1] and self.grid[x, y] == 0

    def find_path(self, start, goal):
        q = Queue()
        q.put((start, [start]))  # Queue stores (current_position, path)
        visited = set()
        visited.add(start)

        while not q.empty():
            (x, y), path = q.get()
            if (x, y) == goal:
                return path

            # Explore neighbors (up, down, left, right)
            for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                nx, ny = x + dx, y + dy
                if self.is_free(nx, ny) and (nx, ny) not in visited:
                    visited.add((nx, ny))
                    q.put(((nx, ny), path + [(nx, ny)]))

        return None  # No path found

    def plot_path(self, path):
        plt.imshow(self.grid, cmap="Greys", origin="upper")
        if path:
            x_coords, y_coords = zip(*path)
            plt.plot(y_coords, x_coords, color="blue", marker="o", linestyle="-")
        plt.show()

# Define grid size and obstacles
grid_size = (10, 10)
obstacles = [(3, 3), (3, 4), (3, 5), (6, 6), (7, 6), (8, 6)]

# Initialize cell decomposition and find path
cell_decomposition = CellDecomposition(grid_size, obstacles)
start = (0, 0)  # Starting point
goal = (9, 9)   # Goal point
path = cell_decomposition.find_path(start, goal)

# Display the grid and path
cell_decomposition.plot_path(path)
