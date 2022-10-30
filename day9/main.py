def part1():
    grid = []
    with open('input.txt') as f:
        for line in f:
            grid.append([int(c) for c in list(line.strip())])
    
    lowest_sum = 0
    for x, row in enumerate(grid):
        for y, value in enumerate(row):
            lowest = True
            for (i,j) in [[x-1,y], [x+1,y], [x,y-1], [x,y+1]]:
                if i >= 0 and i < len(grid) and j >= 0 and j < len(row):
                    if grid[i][j] <= value:
                        lowest = False
            if lowest:
                lowest_sum += value+1
    return lowest_sum

if __name__ == '__main__':
    print(part1())
