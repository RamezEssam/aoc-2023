import sys


bag_contents = {
       "red": 12,
       "green": 13,
       "blue": 14
   }

def calcualtePower(sets):
    min_reds = 0
    min_greens = 0
    min_blues = 0
    for set in sets:
        for draw in set.split(","):
            draw = draw.strip()
            if draw.split(" ")[1].strip() == 'red':
                reds = int(draw.split(" ")[0].strip())
                if reds > min_reds:
                    min_reds = reds
            elif draw.split(" ")[1].strip() == 'green':
                greens = int(draw.split(" ")[0].strip())
                if greens > min_greens:
                    min_greens = greens
            elif draw.split(" ")[1].strip() == 'blue':
                blues = int(draw.split(" ")[0].strip())
                if blues > min_blues:
                    min_blues = blues
    return min_reds* min_greens* min_blues

def checkGame(sets):
    valid_game = True
    for set in sets:
        reds = 0
        greens = 0
        blues = 0
        for draw in set.split(","):
            draw = draw.strip()
            if draw.split(" ")[1].strip() == 'red':
                reds = int(draw.split(" ")[0].strip())
            elif draw.split(" ")[1].strip() == 'green':
                greens = int(draw.split(" ")[0].strip())
            elif draw.split(" ")[1].strip() == 'blue':
                blues = int(draw.split(" ")[0].strip())
        if reds > bag_contents["red"] or greens > bag_contents["green"] or blues > bag_contents["blue"]:
            valid_game = False
    return valid_game

if __name__ == '__main__':
    file_path = sys.argv[1]

    with open(file_path, 'r') as f:
        total = 0
        
        while True:
            line = f.readline()

            if not line:
                break
         
            game_id = int(line.split(":")[0].split(" ")[1])

            sets = line.split(":")[1].strip().split(";")

            sets = [set.strip() for set in sets]

            total += calcualtePower(sets)

            #if checkGame(sets):
                #total += game_id
        print(total)

            

         
     
