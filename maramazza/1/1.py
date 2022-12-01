with open('1.txt') as file:
  inventories = []
  summary = 0
  for row in file:
    if row == '\n':
      inventories.append(summary)
      summary = 0
    else:
      summary += int(row)
  inventories.sort()

# part 1
print(f'part 1: {inventories[-1:][0]} calories')


# part 2
print(f'part 2: {sum(inventories[-3:])} calories')