import sys
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

text = text.replace('q', 'c').replace('qu', '[CW]').replace('k', 'c').replace('v', 'w') \
           .replace('z', 's').replace('x', '[CS]').replace('y', 'i')
           
text = text.replace('cs', '[CS]').replace('cw', '[CW]')

text = text.replace('the', '[THE]').replace('and', '[AND]').replace('ing', '[ING]')

out = ''

for c in text:
    out += c
    out = out.replace('th', '[TH]').replace('te', '[TE]').replace('in', '[IN]') \
             .replace('er', '[ER]').replace('re', '[RE]').replace('an', '[AN]') \
             .replace('ed', '[ED]').replace('nd', '[ND]').replace('on', '[ON]') \
             .replace('en', '[en]').replace('rt', '[RT]').replace('ou', '[OU]') \
             .replace('ha', '[HA]').replace('to', '[TO]').replace('or', '[OR]') \
             .replace('it', '[IT]').replace('is', '[IS]').replace('hi', '[HI]') \
             .replace('es', '[ES]').replace('ng', '[NG]')
             
print(out)