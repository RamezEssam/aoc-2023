import sys


def parseLine(line):

    winning_cards = line.split('|')[0].strip().split(':')[1].strip().split(' ')
    winning_cards = [card for card in winning_cards if card != '']

    card_id = int(line.split('|')[0].strip().split(':')[0].split(" ")[-1].strip())

    my_cards = line.split('|')[1].strip().split(' ')
    my_cards = [card for card in my_cards if card != '']

    return card_id,set(winning_cards), my_cards

def calculatePoints(winning_cards, my_cards):
    point_value = 0
    match_id = 0

    for card in my_cards:
        if card in winning_cards:
            if match_id == 0:
                point_value = 1
                match_id += 1
            else:
                point_value *= 2

    return point_value

def processCard(card, table):
    matches = 0
    card_index = card[0]-1
    for candid in card[2]:
        if candid in card[1]:
            matches += 1
    if matches == 0:
        won_cards = []
        return table
    else:
        won_cards = [card for card in table[card_index+1:card_index+1+matches]]
        for card in won_cards:
            table.append(card)
            #processCard(card, table)
        return table

def countCards(table):
    winnngs = dict()

    for entry in table:
        if entry[0] in winnngs:
            winnngs[entry[0]] += 1
        else:
            winnngs[entry[0]] = 1
    return sum(winnngs.values())
        
        
        

def processTable(table):
    updated_table = table
    
    for card in table:
        updated_table = processCard(card, updated_table)
    
    return updated_table  


if __name__ == '__main__':
    file_path = sys.argv[1]
    total = 0
    table = []
    with open(file_path, 'r') as f:
        while True:
            line = f.readline()

            if not line:
                break

            card_id,winning_cards, my_cards = parseLine(line)

            table.append([card_id, winning_cards, my_cards])
            
            
            total += calculatePoints(winning_cards, my_cards)
    
updated_table = processTable(table)

print(total)
print(countCards(updated_table))