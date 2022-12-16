"""Day11 - puzzle solutions for day 11."""
class Monke:
    def __init__(self, rawMonke):
        self.Counter = 0
        self.Items = list(map(int, rawMonke[0][17:].strip().split(',')))
        self.DivTest = int(rawMonke[2][21:].strip())
        rawOp = rawMonke[1][18:].strip().split()
        if rawOp[1] == '*' and rawOp[0] == rawOp[2]: self.Operation = lambda x: x*x
        elif rawOp[1] == '*': self.Operation = lambda x, arg = int(rawOp[2]): x * arg
        else: self.Operation = lambda x, arg = int(rawOp[2]): x + arg
        self.Targets = [int(rawMonke[4][30:].strip()), int(rawMonke[3][29:].strip())]

def Part1(monke, monkeIndex, item, monkes):
    item = int(int(monke.Operation(item)) / 3)
    monkes[monke.Targets[item % monke.DivTest == 0]].Items.append(item)

def Part2(monke, monkeIndex, item, monkes):
    item = [monke.Operation(item[i]) % monkes[i].DivTest for i in range(len(item))]
    monkes[monke.Targets[item[monkeIndex] == 0]].Items.append(item)

def Process(monkes, roundsAmount, func):
    for round in range(roundsAmount):
        for monkeIndex, monke in enumerate(monkes):
            monke.Counter += len(monke.Items)
            for item in monke.Items: func(monke, monkeIndex, item, monkes)
            monke.Items.clear()
    results = sorted(monkes, key= lambda x: x.Counter)
    print(results[-1].Counter * results[-2].Counter)

with open("11.txt") as file: raw = file.readlines()
Process([Monke(x) for x in [raw[i+1:i+6] for i in range(0, len(raw), 7)]], 20, Part1)
monkes = [Monke(x) for x in [raw[i+1:i+6] for i in range(0, len(raw), 7)]]
for monke in monkes: monke.Items = [[x%y.DivTest for y in monkes] for x in monke.Items]
Process(monkes, 10000, Part2)