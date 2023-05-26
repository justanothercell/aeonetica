#!/usr/bin/python3

import subprocess
import sys
import os

import server.build as server

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

BOLD = '\033[1m'
ENDC = '\033[0m'
BLUE = '\033[94m'
GREEN = '\033[92m'

if __name__ == '__main__':
    os.chdir(dname)
    mode = ['--release'] if '--release' in sys.argv[1:] else []

    processes = [
        ('server', subprocess.Popen(['cargo', 'build', *mode],
                                    cwd=dname+'/server',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
        ('client', subprocess.Popen(['cargo', 'build', *mode],
                                    cwd=dname+'/client',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
    ]
    
    for mod in server.fetch_mods('server/mods/mods.ron'):
        process = subprocess.Popen([sys.executable, 'build.py', '-w', mod, '-d', '../server/mods', *mode],
                                   cwd=dname+'/mods', 
                                   stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        processes.append((f'mods/{mod}', process))
    
    total = len(processes)
    
    print(f'{BOLD}{BLUE}=>> COMPILING EVERYTHING:{ENDC}')
    print(f'Queued {total} builds: {[x[0] for x in processes]}')

    def finished(n, p):
        status = p.poll()
        if status is None:
            return False
        print(f'completed build {BOLD}{n}{ENDC} with exit status: {status} ({total - len(processes) + 1} of {total})')
        return True

    while len(processes) > 0:
        processes[:] = [x for x in processes if not finished(*x)]

