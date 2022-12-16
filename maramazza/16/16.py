"""Day16 - puzzle solutions for day 16."""
most_pressure = 0

def parse_data(filename: str) -> tuple[dict, dict, dict]:
    flow_rates = {}
    valves_to = {}
    valves_open = {}

    with open(filename) as file:
        for line in file:
            words = line.strip().split()
            valve = words[1]
            flow_rate = int(words[4].split("=")[1].split(";")[0])
            to_valves = list(map(lambda w: w.replace(",", ""), words[9:]))

            flow_rates[valve] = flow_rate
            valves_to[valve] = to_valves
            valves_open[valve] = False

    return flow_rates, valves_to, valves_open

def calculate_pressure(valves_open: dict, flow_rates: dict):
    return sum(flow_rates[k] for k, v in valves_open.items() if v)

def simulate1(
    minute,
    valve: str,
    pressures: list[int],
    valves_open: dict,
    flow_rates: dict,
    valves_to: dict,
    visited: dict,
):
    global most_pressure

    if visited.get((minute, valve), -1) >= sum(pressures):
        return
    visited[minute, valve] = sum(pressures)

    if minute == 30:
        most_pressure = max(most_pressure, sum(pressures))
        return

    # check every 2 min
    for should_try_open in [True, False]:
        if should_try_open:

            # If already open, don't open
            if valves_open[valve] or flow_rates[valve] <= 0:
                continue

            valves_open[valve] = True
            pressure = calculate_pressure(valves_open, flow_rates)
            simulate1(
                minute + 1,
                valve,
                pressures + [pressure],
                valves_open,
                flow_rates,
                valves_to,
                visited,
            )
            valves_open[valve] = False
        else:
            pressure = calculate_pressure(valves_open, flow_rates)
            for to_valve in valves_to[valve]:
                simulate1(
                    minute + 1,
                    to_valve if to_valve is not None else valve,
                    pressures + [pressure],
                    valves_open,
                    flow_rates,
                    valves_to,
                    visited,
                )

def simulate2(
    minute,
    valve_a: str,
    valve_b: str,
    pressures: list[int],
    valves_open: dict,
    flow_rates: dict,
    valves_to: dict,
    visited: dict,
):
    global most_pressure

    if visited.get((minute, valve_a, valve_b), -1) >= sum(pressures):
        return
    visited[minute, valve_a, valve_b] = sum(pressures)

    if minute == 26:
        if sum(pressures) > most_pressure:
            most_pressure = sum(pressures)
        return

    # check if all valves open
    if all(v for k, v in valves_open.items() if flow_rates[k] > 0):
        pressure = calculate_pressure(valves_open, flow_rates)
        simulate2(
            minute + 1,
            valve_a,
            valve_b,
            pressures + [pressure],
            valves_open,
            flow_rates,
            valves_to,
            visited,
        )
        return

    # check every 2 min
    for should_try_open_valve_a in [True, False]:
        if should_try_open_valve_a:
            # If already open, don't open
            if valves_open[valve_a] or flow_rates[valve_a] <= 0:
                continue

            valves_open[valve_a] = True

            for should_try_open_valve_b in [True, False]:
                if should_try_open_valve_b:
                    if valves_open[valve_b] or flow_rates[valve_b] <= 0:
                        continue

                    valves_open[valve_b] = True
                    pressure = calculate_pressure(valves_open, flow_rates)
                    simulate2(
                        minute + 1,
                        valve_a,
                        valve_b,
                        pressures + [pressure],
                        valves_open,
                        flow_rates,
                        valves_to,
                        visited,
                    )
                    valves_open[valve_b] = False
                else:
                    pressure = calculate_pressure(valves_open, flow_rates)
                    for to_valve in valves_to[valve_b]:
                        simulate2(
                            minute + 1,
                            valve_a,
                            to_valve,
                            pressures + [pressure],
                            valves_open,
                            flow_rates,
                            valves_to,
                            visited,
                        )
            valves_open[valve_a] = False
        else:
            for to_valve in valves_to[valve_a]:
                for should_try_open_valve_b in [True, False]:
                    if should_try_open_valve_b:
                        if valves_open[valve_b] or flow_rates[valve_b] <= 0:
                            continue

                        valves_open[valve_b] = True
                        pressure = calculate_pressure(valves_open, flow_rates)
                        simulate2(
                            minute + 1,
                            to_valve,
                            valve_b,
                            pressures + [pressure],
                            valves_open,
                            flow_rates,
                            valves_to,
                            visited,
                        )
                        valves_open[valve_b] = False
                    else:
                        pressure = calculate_pressure(valves_open, flow_rates)
                        for to_valve_2 in valves_to[valve_b]:
                            simulate2(
                                minute + 1,
                                to_valve,
                                to_valve_2,
                                pressures + [pressure],
                                valves_open,
                                flow_rates,
                                valves_to,
                                visited,
                            )

def solve_p1():
    flow_rates, valves_to, valves_open = parse_data("16.txt")
    simulate1(1, "AA", [0], valves_open, flow_rates, valves_to, {})
    return most_pressure

def solve_p2():
    flow_rates, valves_to, valves_open = parse_data("16.txt")
    simulate2(1, "AA", "AA", [0], valves_open, flow_rates, valves_to, {})
    return most_pressure

if __name__ == "__main__":
    p1 = solve_p1()
    p2 = solve_p2()
    print(f'Part 1 answer: {p1}')
    print(f'Part 2 answer: {p2}')