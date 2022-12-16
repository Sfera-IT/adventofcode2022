"""Day08 - puzzle solutions for day 08."""
def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

def part1(lines):
    p1 = 0
    G = []
    for line in lines:
        row = line
        G.append(row)

    DIR = [(-1,0),(0,1),(1,0),(0,-1)]
    R = len(G)
    C = len(G[0])

    for r in range(R):
        for c in range(C):
            vis = False
            for (dr,dc) in DIR:
                rr = r
                cc = c
                ok = True
                while True:
                    rr += dr
                    cc += dc
                    if not (0<=rr<R and 0<=cc<C):
                        break
                    if G[rr][cc] >= G[r][c]:
                        ok = False
                if ok:
                    vis = True
            if vis:
                p1 += 1
    return p1

def part2(lines):
    p2 = 0
    G = []
    for line in lines:
        row = line
        G.append(row)

    DIR = [(-1,0),(0,1),(1,0),(0,-1)]
    R = len(G)
    C = len(G[0])

    for r in range(R):
        for c in range(C):
            score = 1
            for (dr,dc) in DIR:
                dist = 1
                rr = r+dr
                cc = c+dc
                while True:
                    if not (0<=rr<R and 0<=cc<C):
                        dist -= 1
                        break
                    if G[rr][cc]>=G[r][c]:
                        break
                    dist += 1
                    rr += dr
                    cc += dc
                score *= dist
            p2 = max(p2, score)
    return p2

if __name__ == "__main__":
    data = load_data("8.txt")
    print(f'Part 1 answer: {part1(data)}')
    print(f'Part 2 answer: {part2(data)}')