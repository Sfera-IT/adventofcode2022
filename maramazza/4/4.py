"""Day04 - puzzle solutions for day 04."""

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

def part1(input):
    containedPairs = 0
    for line in input:
        firstP,secondP = line.split(",")
        firstPStart, firstPEnd = firstP.split("-")
        secondPStart, secondPEnd = secondP.strip().split("-")
        firstPStart, firstPEnd, secondPStart, secondPEnd = [int(x) for x in [firstPStart, firstPEnd, secondPStart, secondPEnd]]
        if ((firstPStart >= secondPStart and firstPEnd <= secondPEnd) or (secondPStart >= firstPStart and secondPEnd <= firstPEnd)):
            containedPairs += 1
    return containedPairs

def part2(input):
    containedPairs = 0
    for line in input:
        firstP,secondP = line.split(",")
        firstPStart, firstPEnd = firstP.split("-")
        secondPStart, secondPEnd = secondP.strip().split("-")
        firstPStart, firstPEnd, secondPStart, secondPEnd = [int(x) for x in [firstPStart, firstPEnd, secondPStart, secondPEnd]]
        if ((firstPStart >= secondPStart and firstPStart <= secondPEnd) or 
            (firstPEnd >= secondPStart and firstPEnd <= secondPEnd) or
            (secondPStart >= firstPStart and secondPStart <= firstPEnd) or
            (secondPEnd >= firstPStart and secondPEnd <= firstPEnd)):
            containedPairs += 1
    return containedPairs

if __name__ == "__main__":
    data = load_data("4.txt")
    print(f'Part 1 answer: {part1(data)}')
    print(f'Part 2 answer: {part2(data)}')