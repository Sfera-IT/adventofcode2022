"""Day07 - puzzle solutions for day 07."""
from collections import defaultdict

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

if __name__ == "__main__":
    data = load_data("7.txt")
    SZ = defaultdict(int)
    path = []
    for line in data:
        words = line.strip().split()
        if words[1] == 'cd':
            if words[2] == '..':
                path.pop()
            else:
                path.append(words[2])
        elif words[1] == 'ls':
            continue
        elif words[0] == 'dir':
            continue
        else:
            sz = int(words[0])
            for i in range(1, len(path)+1):
                SZ['/'.join(path[:i])] += sz

    max_used = 70000000 - 30000000
    total_used = SZ['/']
    need_to_free = total_used - max_used

    p1 = 0
    p2 = 1e9
    for k,v in SZ.items():
        #print(k,v)
        if v <= 100000:
            p1 += v
        if v>=need_to_free:
            p2 = min(p2, v)
    print(f'Part 1 answer: {p1}')
    print(f'Part 2 answer: {p2}')