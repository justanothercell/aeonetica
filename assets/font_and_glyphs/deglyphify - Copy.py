from PIL import Image, ImageFilter
import numpy as np
import sys
from math import sin, pi

if len(sys.argv) == 1:
    print('usage: deglyphify <image>')
    exit()

image = Image.open(sys.argv[1]).convert('L')

edges = image.filter(ImageFilter.FIND_EDGES)

edges.save('edges.png')

wave_range = 32

phase_space = np.zeros((wave_range, wave_range))

def sum_1d_array(arr, o, t):
    s = 0
    m = 0
    for i in range(len(arr)):
        #h = sin((i+o) / (pi * 2) * t)
        #h = 1 if (i+o) % t == 0 else 0
        #h = ((i+o) % t) / t
        #h = 1 if (i+o) % t > t//2 else 0
        #h = 1 if (i+o) % t > t//2 else -1
        #h = 1 if (i+o) % t == 0 else -1
        #h = ((i+o) % t) / (t/2) - 1
        #h = ((i+o) % t) / (t/2) - 1
        #h = 1 if (i+o) % t > t//2 else -1
        #h = 1 if (i+o) % t > t//2 else -1
        h = 1 if (i+o) % t == 0 else -1
        s += h * arr[i]
        m += arr[i]
    return (s / m) if m > 0 else 0

edges_array = np.array(edges.getdata()).reshape(edges.size[0], edges.size[1])

for o in range(wave_range):
    for t in range(0, wave_range):
        if t > 0:
            r = 0
            for row in range(edges_array.shape[1]):
                r += sum_1d_array(edges_array[:, row], o, t)
            phase_space[o, t] = r / edges_array.shape[1]
    print(o)

phase_space -= phase_space.min()

phase_space *= 255 / phase_space.max()

ps_img = Image.fromarray(phase_space)

ps_img.convert('RGB').save('phase_space10mul-3.png')