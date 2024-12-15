def count_edges(grid):
    if not grid or not grid[0]:
        return 0

    rows = len(grid)
    cols = len(grid[0])
    edges = 0

    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == "#":
                # Check all 4 directions
                # Top edge
                if i == 0 or grid[i - 1][j] == ".":
                    edges += 1
                # Bottom edge
                if i == rows - 1 or grid[i + 1][j] == ".":
                    edges += 1
                # Left edge
                if j == 0 or grid[i][j - 1] == ".":
                    edges += 1
                # Right edge
                if j == cols - 1 or grid[i][j + 1] == ".":
                    edges += 1

    return edges


print(
    count_edges(
        """..#..
..##.
..###"""
    )
)
