from functools import reduce
from pprint import pprint


def read_input():
    positions = []
    instructions = []
    reading_positions = True
    with open('./input.txt') as f:
        for row in f:
            row = row.strip()
            if len(row) == 0:
                reading_positions = False
                continue
            if reading_positions:
                (x,y) = row.split(',')
                positions.append((int(x), int(y)))
            else:
                row = row.replace('fold along ', '')
                (axis, pos) = row.split('=')
                instructions.append((axis, int(pos)))

    max_x = reduce(lambda c_max, pos: c_max if c_max > pos[0] else pos[0], positions, 0)
    max_y = reduce(lambda c_max, pos: c_max if c_max > pos[1] else pos[1], positions, 0)

    grid = [[False for _ in range(max_x+1)] for _ in range(max_y+1)]

    for (x,y) in positions:
        grid[y][x] = True

    return (grid, instructions)

def rot_90(l):
    return [list(reversed(x)) for x in zip(*l)]

def fold(grid, axis, pos):
    if axis == 'x':
        grid = rot_90(grid)

    new_grid_height = max(pos, len(grid)-pos-1)
    
    new_grid = [[False for _ in range(len(grid[0]))] for _ in range(new_grid_height)]
    

    for i, row in enumerate(grid[pos+1:]):
        new_grid_pos = new_grid_height-1-i
        new_grid[new_grid_pos] = row
    for i, row in enumerate(grid[:pos]):
        new_grid_pos = new_grid_height-pos+i
        new_grid[new_grid_pos] = [
            row[index] or cell for index, cell in enumerate(new_grid[new_grid_pos])
        ]

    if axis == 'x':
        new_grid = rot_90(rot_90(rot_90(new_grid)))
    return new_grid

def a(grid, instructions):
    new_grid = fold(grid, instructions[0][0], instructions[0][1])
    count = 0
    for row in new_grid:
        for cell in row:
            if cell:
                count += 1
    return count

def b(grid, instructions):
    return 0
    

if __name__ == '__main__':
    grid, instructions = read_input()
    print(f"A: {a(grid, instructions)}")
    print(f"B: {b(grid, instructions)}")