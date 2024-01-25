import sys
from tqdm import tqdm


def find_location(seeds, lines):
    
    min_location = sys.maxsize
    for seed in seeds:
        seed = int(seed)
        seed_mapped = False

        
        for line in lines:
            if line[0].isalnum():
                
                if line[0].isalpha():
                    seed_mapped = False

                if line[0].isdigit():
                    
                    ranges = line.split(" ")
                    dst_range_start = int(ranges[0])
                    src_range_start = int(ranges[1])
                    range_len = int(ranges[2])

                    if not seed_mapped and seed >= src_range_start and seed <= src_range_start+range_len:
                        seed = dst_range_start + (seed - src_range_start)
                        seed_mapped = True

        if seed < min_location:
            min_location = seed 
                        
    return min_location



def Part_One(file_path):
    with open(file_path, 'r') as f:
        lines = f.readlines()
        seeds = lines[0][:-1]
        seeds = seeds.split(":")[1].split(" ")[1:]
        lines = lines[2:]

    return find_location(seeds, lines)


def Part_Two(file_path):

    with open(file_path, 'r') as f:
        lines = f.readlines()
        seeds = lines[0][:-1]
        seeds = seeds.split(":")[1].split(" ")[1:]
        seeds= [int(seed) for seed in seeds]
        
        lines = lines[2:]
        i = 0
        j = 1
        global_min = sys.maxsize
        
        while i < len(seeds) and j < len(seeds):
            for seed in range(seeds[i], seeds[i]+seeds[j]):
                location = find_location([seed], lines)
                #print(seed, location)
                if location < global_min:
                    global_min = location
            i += 2
            j += 2
                
            
                
                

    return global_min

    

if __name__== '__main__':

    file_path = sys.argv[1]

    part_one_solution = Part_One(file_path)

    print(f'Part 1 Solution: {part_one_solution}')

    part_two_solution = Part_Two(file_path)

    print(f'Part 2 Solution: {part_two_solution}')
    