def parse_grid():
    grid = []
    with open("input.txt", "r") as input:
        for line in input:
            grid.append([int(n) for n in list(line.strip())])
    return grid

def get_neighbour_pos(x,y, grid):
    neigbours = []
    for y_i in range(-1,2):
        for x_i in range(-1,2):
            new_x = x+x_i
            new_y = y+y_i
            if (not (new_x == x and new_y == y)) and new_x >= 0 and new_x < len(grid[0]) and new_y >= 0 and new_y < len(grid):
                neigbours.append((new_x,new_y))
    return neigbours

def a():
    grid = parse_grid()
    flashes = 0
    for _ in range(100):
        for y in range(len(grid)):
            for x in range(len(grid)):
                grid[y][x] += 1
        is_flashable = True
        while is_flashable:
            is_flashable = False
            for y in range(len(grid)):
                for x in range(len(grid)):
                    if grid[y][x] > 9:
                        is_flashable = True
                        grid[y][x] = 0
                        flashes += 1
                        for (n_x, n_y) in get_neighbour_pos(x,y,grid):
                            if grid[n_y][n_x] != 0:
                                grid[n_y][n_x] += 1
    
    print(f"A. Flashes: {flashes}")

if __name__ == '__main__':
    a()