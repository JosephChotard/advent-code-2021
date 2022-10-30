import heapq
from functools import reduce

grid = []
with open('input.txt') as f:
    for line in f:
        grid.append([int(c) for c in list(line.strip())])

def get_neighbour_pos(x,y):
    for (i,j) in [[x-1,y], [x+1,y], [x,y-1], [x,y+1]]:
        if i >= 0 and i < len(grid) and j >= 0 and j < len(grid[0]):
            yield (i,j)

def get_low_point_coord():
    low_points = []
    for x, row in enumerate(grid):
        for y, value in enumerate(row):
            lowest = True
            for (i,j) in get_neighbour_pos(x,y):
                if grid[i][j] <= value:
                    lowest = False
            if lowest:
                low_points.append((x,y))
    return low_points


def part1():
    return reduce(lambda accum, coord: grid[coord[0]][coord[1]]+1+accum, get_low_point_coord(), 0)

def size_of_basin(x,y, visited=set()) -> int:
    size = 0
    if (x,y) not in visited and grid[x][y] < 9:
        size = 1
        visited.add((x,y))
        for (i,j) in get_neighbour_pos(x,y):
            size += size_of_basin(i,j,visited)
    return size


def part2() -> int:
    three_largest= heapq.nlargest(3, [size_of_basin(x,y) for (x,y) in get_low_point_coord()])
    return reduce((lambda x,y: x*y), three_largest)

if __name__ == '__main__':
    print(part1())
    print(size_of_basin(0,0))
    print(part2())
