import re, z3

print("Day 24 Part 2 (in python)")

lines = open("input/my/day_24.data", "r").readlines()

stones = [list([int(n) for n in re.findall("-?\d+", l)]) for l in lines]

px, py, pz, vx, vy, vz = z3.Ints("pxi pyi pzi vxi vyi vzi")
ts = [z3.Int("t" + str(i)) for i in range(len(stones))]

solver = z3.Solver()
for i, (pxi, pyi, pzi, vxi, vyi, vzi) in enumerate(stones):
    solver.add(px + vx * ts[i] == pxi + vxi * ts[i])
    solver.add(py + vy * ts[i] == pyi + vyi * ts[i])
    solver.add(pz + vz * ts[i] == pzi + vzi * ts[i])
solver.check()
print(f'Answer is: {solver.model().evaluate(px + py + pz)}')
