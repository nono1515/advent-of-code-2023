input_ = """\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""

grid = [list(line) for line in input_.splitlines()]
pattern = "XMAS"
dirs = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]
count = 0

for i in range(len(grid)):
    for j in range(len(grid[0])):
        for d in dirs:
            pos = (i, j)
            for c in pattern:
                try:
                    if c == grid[pos[0]][pos[1]]:
                        pos = (pos[0] + d[0], pos[1] + d[1])
                    else:
                        break
                except IndexError:
                    break
            else:
                print(pos, d)
                count += 1

print(count)
