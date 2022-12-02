"""Day02 - puzzle solutions for day 02."""

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

# A, X - rock (1); B, Y - paper (2); C, Z - scissors (3)
# 0 - loss, 3 - draw, 6 - win
def point_dict(play_round):
    """Read round result from dictionary."""
    points = {'A X': 4,
              'B X': 1,
              'C X': 7,
              'A Y': 8,
              'B Y': 5,
              'C Y': 2,
              'A Z': 3,
              'B Z': 9,
              'C Z': 6}
    return points[play_round]

def solve_part1(letters):
    """Solve part 1 of the puzzle."""
    result = 0
    for row in letters:
        result += point_dict(row)
    return result

# A - rock (1); B - paper (2); C - scissors (3)
# X, 0 - loss, Y - draw, Z - win
def point_dict_2(play_round):
    """Read round result from dictionary."""
    points = {'A X': 3,
              'B X': 1,
              'C X': 2,
              'A Y': 4,
              'B Y': 5,
              'C Y': 6,
              'A Z': 8,
              'B Z': 9,
              'C Z': 7}
    return points[play_round]

def solve_part2(letters):
    """Solve part 2 of the puzzle."""
    result = 0
    for row in letters:
        result += point_dict_2(row)
    return result

if __name__ == "__main__":
    data = load_data("2.txt")
    print(f'Part 1 answer: {solve_part1(data)}')
    print(f'Part 2 answer: {solve_part2(data)}')