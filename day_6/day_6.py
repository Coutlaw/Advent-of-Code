def part_1(groups: list) -> int:
    total = 0
    for group in groups:
        total += len(set(group.replace("\n", "")))
    return total

def part_2(groups: list) -> int:
    total = 0
    for group in groups:
        num_responses = group.count('\n')
        (response, _, rest) = group.partition("\n")
        rest = rest.replace("\n", "")
        for letter in response:
            if rest.count(letter) == num_responses:
                total += 1

    return total

file = open("test2.txt")
groups = file.read().split("\n\n")
file.close()
print("part 1:", part_1(groups))
print("part 2:", part_2(groups))