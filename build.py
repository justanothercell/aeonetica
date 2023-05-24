#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

if __name__ == '__main__':
    os.chdir(dname)
    mode = ['--release'] if '--release' in sys.argv[1:] else []

    processes = [
        ('server', subprocess.Popen(['cargo', 'build', *mode],
                                    shell=True, cwd=dname+'/server',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
        ('client', subprocess.Popen(['cargo', 'build', *mode],
                                    shell=True, cwd=dname+'/client',
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
        ('mods/player', subprocess.Popen(['py', 'build.py', '-w', 'player', '-d', '../server/mods', *mode],
                                         shell=True, cwd=dname+'/mods',
                                         stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)),
        ('mods/world', subprocess.Popen(['py', 'build.py', '-w', 'world', '-d', '../server/mods', *mode],
                                        shell=True, cwd=dname+'/mods',
                                        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL))
    ]
    total = len(processes)

    print(f'Queued {total} builds:')
    for n, p in processes:
        print(n)

    def finished(n, p):
        status = p.poll()
        if status is None:
            return False
        print(f'completed build {n} with exit status: {status} ({total - len(processes) + 1} of {total})')
        return True

    while len(processes) > 0:
        processes[:] = [x for x in processes if not finished(*x)]

