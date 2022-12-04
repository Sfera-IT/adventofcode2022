"""Day03 - puzzle solutions for day 03."""

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

def part1(input):
    sumOfItemPriorities = 0
    for line in input:
        firstComp = line[0:int(len(line)/2)]
        secondComp = line[int(len(line)/2):len(line)]
        wrongItemType = ''.join(set(firstComp).intersection(secondComp))
        sumOfItemPriorities += itemPriorities.index(wrongItemType) + 1
    return sumOfItemPriorities

def part2(input):
    sumOfBadgePriorities = 0
    for group in range(1, int(len(input)/3)+1):
        badgeItemType = ''.join(set(
                ''.join(set(input[(group*3)-3]).intersection(input[(group*3)-2]))
            ).intersection(input[(group*3)-1]))
        sumOfBadgePriorities += itemPriorities.index(badgeItemType.strip()) + 1
    return sumOfBadgePriorities

if __name__ == "__main__":
    itemPriorities = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]
    data = load_data("3.txt")
    print(f'Part 1 answer: {part1(data)}')
    print(f'Part 2 answer: {part2(data)}')