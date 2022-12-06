"""Day06 - puzzle solutions for day 06."""

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

def sol(input,value):
    currentStream = []
    completedPack = []

    for x in range(value):
        completedPack.append(input[0][x])
        currentStream.append(input[0][x])

    for charIndex in range(len(input[0])):
        currentStream.append(input[0][charIndex+value])

        presentChars = set()
        duplicates = [x for x in currentStream if x in presentChars or presentChars.add(x)] # find duplicates

        completedPack.append(input[0][charIndex+value])
        if len(duplicates) == 0:
            return len(completedPack)
        currentStream.pop(0)

if __name__ == "__main__":
    data = load_data("6.txt")
    print(f'Part 1 answer: {sol(data,3)}')
    print(f'Part 2 answer: {sol(data,13)}')
