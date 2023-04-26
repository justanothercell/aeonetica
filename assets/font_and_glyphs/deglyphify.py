from PIL import Image, ImageFilter
import numpy as np
import sys
from math import sin, pi

if len(sys.argv) == 1:
    print('usage: deglyphify <image>')
    exit()

image = Image.open(sys.argv[1]).convert('L')

edges = image.filter(ImageFilter.Kernel((3, 3), (0, 0, 0, 1, -1, 0, 0, 0, 0), 2, 128))
e_arr = np.array(edges.getdata()).reshape(edges.size[1], edges.size[0])
e_arr[e_arr==128] = 0
e_arr[e_arr!=0] = 255
e_arr[0, :] = 0
e_arr[-1, :] = 0
e_arr[:, 0] = 0
e_arr[:, -1] = 0
edges = Image.fromarray(np.uint8(e_arr), 'L')

edges.save('gen/edges.png')

wave_range = 32

def sum_1d_array(arr, o, t):
    f = [0] * t
    
    #for i in range(len(arr)):
    #    f[(i+o)%t] += 1 if arr[i] > 0 else -1
    #    
    #return (f[0] / sum(f[1:]) * (t/len(arr))) if sum(f[1:]) != 0 else 0
    for i in range(len(arr)):
        f[(i+o)%t] += 1 if arr[i] > 0 else 0
    return (f[0] - sum(f[1:]))
    
phase_space = np.zeros((wave_range, wave_range-2))
edges_array = np.array(edges.getdata()).reshape(edges.size[0], edges.size[1])

for o in range(wave_range):
    for t in range(2, wave_range):
        r = 0
        for row in range(edges_array.shape[1]):
            r += sum_1d_array(edges_array[:, row], o, t)
        phase_space[o, t-2] = r / edges_array.shape[1]
    print(o)

phase_space -= phase_space.min()

phase_space *= 255 / phase_space.max()

ps_img = Image.fromarray(phase_space)

ps_img.convert('RGB').save('gen/phase_space.png')