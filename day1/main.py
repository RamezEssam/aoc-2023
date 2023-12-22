import sys
import re


def parseLine(line):
    pattern = r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))'
    results = re.findall(pattern, line)
    if len(results) > 1:
        if results[0] in ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']:
            match results[0]:
                case 'one':
                    digit_one = '1'
                case 'two':
                    digit_one = '2'
                case 'three':
                    digit_one = '3'
                case 'four':
                    digit_one = '4'
                case 'five':
                    digit_one = '5'
                case 'six':
                    digit_one = '6'
                case 'seven':
                    digit_one = '7'
                case 'eight':
                    digit_one = '8'
                case 'nine':
                    digit_one = '9'
        else:
            digit_one = results[0]

        if results[-1] in ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']:
            match results[-1]:
                case 'one':
                    digit_two = '1'
                case 'two':
                    digit_two = '2'
                case 'three':
                    digit_two = '3'
                case 'four':
                    digit_two = '4'
                case 'five':
                    digit_two = '5'
                case 'six':
                    digit_two = '6'
                case 'seven':
                    digit_two = '7'
                case 'eight':
                    digit_two = '8'
                case 'nine':
                    digit_two = '9'
        else:
            digit_two = results[-1]
        return int(f"{digit_one}{digit_two}")

    elif len(results) == 1:
        if results[0] in ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']:
            match results[0]:
                case 'one':
                    digit_one = '1'
                case 'two':
                    digit_one = '2'
                case 'three':
                    digit_one = '3'
                case 'four':
                    digit_one = '4'
                case 'five':
                    digit_one = '5'
                case 'six':
                    digit_one = '6'
                case 'seven':
                    digit_one = '7'
                case 'eight':
                    digit_one = '8'
                case 'nine':
                    digit_one = '9'
        else:
            digit_one = results[0]
        return int(f"{digit_one}{digit_one}")
    else:
        return 0


if __name__ == '__main__':
    file_path = sys.argv[1]
    with open(file_path, 'r') as f:
        total = 0
        while True:
            line = f.readline()
            if not line:
                break
            number = parseLine(line)
            total += number
    print(total)