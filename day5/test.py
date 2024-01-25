file_path = 'sample.txt'


def parse_lines(lines):
    map= {}
    seed_mapped = False

    for line in lines:
        if line[0].isalpha():
            seed_mapped = False
            map_name = lines[0][:-2].split(" ")[0]
            
        if line[0].isdigit():
            ranges = line.split(" ")
            dst_range_start = int(ranges[0])
            src_range_start = int(ranges[1])
            range_len = int(ranges[2])

            if not seed_mapped:
                
            
             


with open(file_path, 'r') as f:
        lines = f.readlines()
        seeds = lines[0][:-1]
        seeds = seeds.split(":")[1].split(" ")[1:]
        seeds= [int(seed) for seed in seeds]
        lines = lines[2:]

print(seeds)

print(lines[0][:-2].split(" ")[0])