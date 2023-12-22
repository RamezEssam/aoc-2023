import sys


def parseLine(line):
    row = []
    row.append('.')
    for char in line:
        if char != '\n':
            row.append(char)
    row.append('.')
    return row


def getImmediateNeighbors(x,y, grid):
    rows = len(grid[0])
    cols= len(grid)
    neighbors= set()

    if x+1 < rows:
        neighbors.add(grid[y][x+1])
    if x-1 > 0:
        neighbors.add(grid[y][x-1])
    if y+1 < cols:
        neighbors.add(grid[y+1][x])
    if y-1 > 0:
        neighbors.add(grid[y-1][x])
    if x+1 < rows and y+1 < cols:
        neighbors.add(grid[y+1][x+1])
    if x-1 > 0 and y-1 > 0:
        neighbors.add(grid[y-1][x-1])
    if x+1 < rows and y-1 > 0:
        neighbors.add(grid[y-1][x+1])
    if x-1 > 0 and y+1 < cols:
        neighbors.add(grid[y+1][x-1])

    return neighbors


def fetchDigits(x,y, grid):
    rows = len(grid[0])
    set_of_numbers = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}
    visited_indices = set()

    middle_part = grid[y][x]
    left_part = ''
    right_part = ''
    visited_indices.add(x)
    for i in reversed(range(0, x)):
        if grid[y][i] in set_of_numbers:
            visited_indices.add(i)
            left_part = ''.join([grid[y][i], left_part])
        else:
            break
    for i in range(x+1, rows):
        if grid[y][i] in set_of_numbers:
            visited_indices.add(i)
            right_part = ''.join([right_part, grid[y][i]])
        else:
            break
    number = int(''.join([left_part, middle_part, right_part]))
    return (number, visited_indices)
    

def getNeighboringNumbers(x, y, grid):
    rows = len(grid[0])
    cols= len(grid)
    set_of_numbers = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'}
    neighbors= []
    visited = {i: set() for i in range(cols)}
    

    if x+1 < rows:
        if grid[y][x+1] in set_of_numbers:
            number, visited_indices = fetchDigits(x+1, y, grid)
            if not visited_indices.issubset(visited[y]):
                visited[y].update(visited_indices)
                neighbors.append(number)
                
    if x-1 >= 0:
        if grid[y][x-1] in set_of_numbers:
            number, visited_indices = fetchDigits(x-1, y, grid)
            if not visited_indices.issubset(visited[y]):
                visited[y].update(visited_indices)
                neighbors.append(number)
    
    if y+1 < cols:
        if grid[y+1][x] in set_of_numbers:
            number, visited_indices = fetchDigits(x, y+1, grid)
            if not visited_indices.issubset(visited[y+1]):
                visited[y+1].update(visited_indices)
                neighbors.append(number)
    if y-1 >= 0:
        if grid[y-1][x] in set_of_numbers:
            number, visited_indices = fetchDigits(x, y-1, grid)
            if not visited_indices.issubset(visited[y-1]):
                visited[y-1].update(visited_indices)
                neighbors.append(number)

    if x+1 < rows and y+1 < cols:
        if grid[y+1][x+1] in set_of_numbers:
            number, visited_indices = fetchDigits(x+1, y+1, grid)
            if not visited_indices.issubset(visited[y+1]):
                visited[y+1].update(visited_indices)
                neighbors.append(number)

    if x-1 >= 0 and y-1 >= 0:
        if grid[y-1][x-1] in set_of_numbers:
            number, visited_indices = fetchDigits(x-1, y-1, grid)
            if not visited_indices.issubset(visited[y-1]):
                visited[y-1].update(visited_indices)
                neighbors.append(number)

    if x+1 < rows and y-1 >= 0:
        if grid[y-1][x+1] in set_of_numbers:
            number, visited_indices = fetchDigits(x+1, y-1, grid)
            if not visited_indices.issubset(visited[y-1]):
                visited[y-1].update(visited_indices)
                neighbors.append(number)

    if x-1 >= 0 and y+1 < cols:
        if grid[y+1][x-1] in set_of_numbers:
            number, visited_indices = fetchDigits(x-1, y+1, grid)
            if not visited_indices.issubset(visited[y+1]):
                visited[y+1].update(visited_indices)
                neighbors.append(number)
    
    return neighbors


def checkNeighbors(neighbors):
    valid_digit = False
    for neighbor in neighbors:
        if neighbor not in {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'}:
            valid_digit = True
    return valid_digit

def getSumOfPartNumbers(schematic):
    rows = len(schematic[0])
    cols = len(schematic)
    total = 0
    visited = {i : set() for i in range(cols)}
    for y in range(cols):
        part_number = ''
        all_neighbors = set()
        for x in range(rows):
            if schematic[y][x] in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
                part_number = ''.join([part_number,schematic[y][x]])
                neighbors = getImmediateNeighbors(x, y, schematic)
                all_neighbors.update(neighbors)
            else:
                if part_number != '':
                    if checkNeighbors(all_neighbors):
                        total += int(part_number)
                all_neighbors = set()
                part_number = ''
    return total  

def getSumOfGearRatios(schematic):
    rows = len(schematic[0])
    cols = len(schematic)
    total = 0

    for y in range(cols):
        for x in range(rows):
            neighbors = []
            if schematic[y][x] == '*':
                neighbors.extend(getNeighboringNumbers(x,y, schematic))
                if len(neighbors) == 2:
                    total += neighbors[0]*neighbors[1]
    return total
                       
                

if __name__ == '__main__':
    file_path = sys.argv[1]
    schematic = []
    with open(file_path, 'r') as f:
        while True:
            line = f.readline()
            if not line:
                break
            row = parseLine(line)
            #print(row)
            schematic.append(row)
    
print(getSumOfPartNumbers(schematic))

print(getSumOfGearRatios(schematic))