import sys
from PIL import Image
import numpy as np
import re

# This is an example text\nwhich i will now rewrite using\nonly the available glyphs\nand also the combinators

# this is very inefficiently implemented, jsut want to make sure the order is right

# order of appliance
# 1. substitute the missing letters
# -> groupings in those substitutions immediately get grouped
# 2. 3 letter substiutions, will not be replaced for groupings from step 1
# 3. group the rest strictly from left to right.

if len(sys.argv) < 2:
    print('usage: scriptify.py <text>')
    exit()
    
text = ' '.join(sys.argv[1:]).lower()

#with open('genesis.txt.txt') as g:
#    text = g.read().lower()

text = text.replace('\n', '\\')

text = text.replace(':', '').replace(',', '').replace('.', '').replace(';', '').replace(':', '').replace('"', '');

text = text.replace('q', 'c').replace('qu', '[CW]').replace('ck', 'c').replace('k', 'c').replace('v', 'w') \
           .replace('z', 's').replace('x', '[CS]').replace('y', 'i')
           
text = text.replace('cs', '[CS]').replace('cw', '[CW]')

text = text.replace('the', '[THE]').replace('and', '[AND]').replace('ing', '[ING]')

out = ''

for c in text:
    out += c
    out = out.replace('th', '[TH]').replace('te', '[TE]').replace('in', '[IN]') \
             .replace('er', '[ER]').replace('re', '[RE]').replace('an', '[AN]') \
             .replace('ed', '[ED]').replace('nd', '[ND]').replace('on', '[ON]') \
             .replace('en', '[EN]').replace('rt', '[RT]').replace('ou', '[OU]') \
             .replace('ha', '[HA]').replace('to', '[TO]').replace('or', '[OR]') \
             .replace('it', '[IT]').replace('is', '[IS]').replace('hi', '[HI]') \
             .replace('es', '[ES]').replace('ng', '[NG]')
             
print(out)

with Image.open('glyph_gen.bmp') as glyph_img:
    glyphs = np.array(glyph_img)

glyphs_pos = {
    'a': (0, 0), 'b': (1, 0), 'c': (2, 0), 'd': (3, 0), 'e': (4, 0), 
    'f': (5, 0), 'g': (6, 0), 'h': (7, 0), 'i': (8, 0), 'j': (9, 0),
    'l': (0, 1), 'm': (1, 1), 'n': (2, 1), 'o': (3, 1), 'p': (4, 1), 
    'r': (5, 1), 's': (6, 1), 't': (7, 1), 'u': (8, 1), 'w': (9, 1),
    '[TH]': (0, 2), '[TE]': (0, 3), '[IN]': (0, 4), '[ER]': (0, 5), '[RE]': (0, 6), '[AN]': (0, 7),
    '[ED]': (1, 2), '[ND]': (1, 3), '[ON]': (1, 4), '[EN]': (1, 5), '[AT]': (1, 6), '[OU]': (1, 7),
    '[HA]': (2, 2), '[TO]': (2, 3), '[OR]': (2, 4), '[IT]': (2, 5), '[IS]': (2, 6), '[HI]': (2, 7),
    '[ES]': (3, 2), '[NG]': (3, 3), '[CS]': (3, 4), '[CW]': (3, 5),
    '[THE]': (4, 2), '[AND]': (4, 3), '[ING]': (4, 4)
}

rows_len = re.sub(r'\[.*?\]', 'x', out).split('\\')

max_w = max([len(r) for r in rows_len])
h = len(rows_len)

outimg = np.zeros((h * 9 + 2, max_w * 9 + 2, 3), dtype=np.uint8)

x = 0
y = 0
mx = 0

for item in re.findall(r'\[[^\]]*\]|.', out):
    p = glyphs_pos.get(item)
    mdx = 0
    if item == '\\':
        mx = max(x, mx)
        x = 0
        y += 9
    else:
        if p is not None:
            for dx in range(9):
                for dy in range(9):
                    if sum(glyphs[p[1] * 9 + dy][p[0] * 9  + dx]) > 0:
                        mdx = dx
                    outimg[y + dy + 1,x + dx + 1,:] = glyphs[p[1] * 9  + dy,p[0] * 9  + dx,:]
        else:
            mdx = 3
        x += mdx + 1

mx = max(x, mx)

outimg = outimg[:,:mx+3]

print()

ascii_img = ''
ascii_opts = [' ', '▄', '▀', '█']

for y in range(0, outimg.shape[0]-1, 2):
    for x in range(outimg.shape[1]):
        ascii_img += ascii_opts[(outimg[y, x].max() > 0) * 2 + (outimg[y+1, x].max() > 0)]
    ascii_img += '\n'

print(ascii_img)

print()

discord_img = ''

for y in range(1, outimg.shape[0]-1):
    for x in range(outimg.shape[1]):
        discord_img += '⬛' if outimg[y, x].max() > 0 else '⬜'
    discord_img += '\n'

print(discord_img)

im = Image.fromarray(outimg, mode='RGB')
im.save('generated.png')

im.resize((im.width * 16, im.height * 16), resample=Image.Resampling.NEAREST).save('generated_x16.png')

