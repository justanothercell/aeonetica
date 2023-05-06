import sys
import os
import re

if len(sys.argv) != 2:
    print('Usage: py new.py <mod_name>')
    exit()

mod_name = sys.argv[1]

mod_name_capitalized = mod_name[0].upper() + mod_name[1:]

if not re.match(r'^[a-z][a-z0-9]+(_[a-z0-9]+)*$', mod_name):
    print('ERROR: mod name does not match snake_case naming convention')
    exit()
    
if os.path.exists(mod_name):
    print(f'ERROR: directory/mod \'{mod_name}\' already exists')
    exit()

def copytree(src, dst):
    if not os.path.exists(dst):
        os.makedirs(dst)
    for item in os.listdir(src):
        s = os.path.join(src, item)
        d = os.path.join(dst, item)
        if os.path.isdir(s):
            copytree(s, d)
        else:
            with open(s, encoding='utf-8') as from_file:
                with open(d, 'w', encoding='utf-8') as to_file:
                    text = from_file.read()
                    text = text.replace('{{MOD_NAME}}', mod_name)
                    text = text.replace('{{MOD_NAME_CAPITALIZED}}', mod_name_capitalized)
                    to_file.write(text)
                    
copytree('template', mod_name)

print(f'Successfully set up mod \'{mod_name}\'')