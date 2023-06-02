#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'
GREEN = '\033[92m'

def fetch_mods(ron_file):
    mods = []
    print(f'{BLUE}{BOLD}=>> FETCHING MODS: {ENDC}')
    
    with open(ron_file, 'r') as file:
        for line in file:
            line = line.strip()
            if not line.startswith('"'):
                continue
            
            mod = line[1:line.find(':')]
            mods.append(mod)
            print(f'{GREEN} -> found `{mod}` {ENDC}')
    return mods

if __name__ == '__main__':
    os.chdir(f'{dname}/../mods')
    
    mod_override = [i for i, arg in enumerate(sys.argv) if arg == '-m' or arg == '--mods']
    mods = fetch_mods(f'{dname}/mods/mods.ron') if len(mod_override) == 0 else [sys.argv[mod_override[0] + 1]]

    for mod in mods:
        print(f'{BLUE}{BOLD}=>> COMPILING MOD {mod}:{ENDC}')
        subprocess.call([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods'])
    
    os.chdir(dname)
    print(f'{BLUE}{BOLD}=>> COMPILING SERVER: {ENDC}')
    
    subprocess.call(['cargo', 'run' if '-r' in sys.argv or '--run' in sys.argv else 'build'])
