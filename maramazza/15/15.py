"""Day15 - puzzle solutions for day 15."""
import re
target = 2000000
pattern = re.compile("Sensor at x=([\d-]*), y=([\d-]*): closest beacon is at x=([\d-]*), y=([\d-]*)")

def convert(line):
    res = pattern.match(line)
    return [int(k) for k in res.groups()]

data = [s.rstrip() for s in open('15.txt').readlines()]
data = [convert(line) for line in data]

def part1(data):
    filled = set()
    beacons = set()
    for x0,y0,x1,y1 in data:
        if y0 == target:
            beacons.add(x0)
        if y1 == target:
            beacons.add(x1)
        fromtgt = abs(target-y0)

        dx = abs(x1-x0)+abs(y1-y0)
        if dx < fromtgt:
            continue
        
        x00 = x0 - dx + fromtgt
        x99 = x0 + dx - fromtgt
        for i in range(x00,x99+1):
            filled.add(i)
    filled -= beacons
    return len(filled)

def part2(data):
    rows = [[] for _ in range(target+target+2)]
    for x0,y0,x1,y1 in data:
        print(x0,y0,x1,y1)
        dist = abs(x1-x0)+abs(y1-y0)
        for dy in range(-dist,dist+1):
            if y0+dy not in range(target+target+1):
                continue
            x0t = max( 0, min( target+target, x0 - (dist-abs(dy))))
            x1t = max( 0, min( target+target, x0 + (dist-abs(dy))))
            rows[y0+dy].append( (x0t, x1t) )

    for y, row in enumerate(rows):
        row.sort()
        combine = []
        xlo, xhi = row[0]
        for x0, x1 in row:
            if x0-1 <= xhi:
                xhi = max(xhi, x1)
            else:
                combine.append( (xlo, xhi) )
                xlo, xhi = x0, x1
        combine.append( (xlo, xhi) )
        if len(combine) > 1:
            assert len(combine) == 2
            assert combine[0][1]+2 == combine[1][0]
            return (combine[0][1]+1) * 4000000 + y

print(f'Part 1 answer: {part1(data)}')
print(f'Part 2 answer: {part2(data)}')