import numpy as np
import re
import matplotlib.pyplot as plt

def read_input_14(filename):
    f = open(filename)
    robots = []
    for l in f.readlines():
        x = [int(d) for d in re.findall(r"[-+]?\d+",l)]
        p = x[0]+x[1]*1j
        v = x[2]+x[3]*1j
        robots.append((p,v))
    return robots

def move_robots(robots,xmax,ymax):
    newrobots = []
    for p,v in robots:
        p1 = p+v
        x = p1.real
        y = p1.imag
        if x<0: x+=xmax
        if x>=xmax: x-=xmax
        if y<0: y+=ymax
        if y>=ymax: y-=ymax
        newrobots.append((x+y*1j,v))
    return newrobots

def evolve_robots(robots,xmax,ymax,nsec=1):
    for i in range(nsec):
        newrobots = move_robots(robots,xmax,ymax)
        robots = newrobots
    return robots

def robots_to_grid(robots,xmax,ymax):
    grid = np.zeros((ymax,xmax),dtype=int)
    for p,v in robots:
        x = int(p.real)
        y = int(p.imag)
        grid[y,x] += 1
    return grid

def entropy(img):
    marg = np.histogramdd(np.ravel(img), bins = 256)[0]/img.size
    marg = list(filter(lambda p: p > 0, np.ravel(marg)))
    entropy = -np.sum(np.multiply(marg, np.log2(marg)))
    return entropy

filename = "../../data/inputs/14.txt"
robots = read_input_14(filename)
xmax = 101
ymax = 103

entropies = []
for i in range(10000):
    newrobots = move_robots(robots,xmax,ymax)
    grid = robots_to_grid(newrobots,xmax,ymax)
    entropies.append(entropy(grid))
    robots = newrobots

fig, ax = plt.subplots(figsize=(8,6),dpi=100)

plt.plot([i+1 for i in range(10000)],entropies,linestyle="",marker=".")

ax.set_xlabel("time (s)")
ax.set_ylabel("entropy")

easter_egg = entropies.index( min(entropies) )+1
plt.plot(easter_egg,entropies[easter_egg-1],linestyle="",marker="o")

print(easter_egg)
