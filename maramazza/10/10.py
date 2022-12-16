"""Day10 - puzzle solutions for day 10."""
from typing import List

def load_data(path: str) -> list[str]:
    """Load and split data from file."""
    rows = []
    with open(path, encoding="ascii") as file:
        for row in file:
            rows.append(row.rstrip())
    return rows

def print_screen(lines: List[str], signal_selection: List[int]):
    register: int = 1
    cycle: int = 1
    signal_strengths: List[str] = [register * cycle]

    screen_pixels = ""

    for line in lines:
        screen_pixels = print_pixel(screen_pixels, cycle, register)
        cycle += 1
        signal_strengths.append(register * cycle)
        
        if "addx" in line:
            screen_pixels = print_pixel(screen_pixels, cycle, register)           
            cycle += 1
            register += int(line.split()[1])
            signal_strengths.append(register * cycle)
            
    sum_selected_signal_strengths = sum(signal_strengths[i] for i in signal_selection)

    print(f"We have {sum_selected_signal_strengths=}")

    print(screen_pixels)

def print_pixel(screen_pixels, cycle, register):
    row_number = cycle // 40
    selected_pixel = cycle - row_number * 40 - 1
    if abs((selected_pixel - register)) <= 1:
        screen_pixels += "#"
    else:
        screen_pixels += "."
    if selected_pixel % 40 == 39:
        screen_pixels += "\n"
    return screen_pixels

if __name__ == "__main__":
    lines = load_data("10.txt")
    signal_selection = [19, 59, 99, 139, 179, 219]
    print_screen(lines, signal_selection)