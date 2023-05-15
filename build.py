#!/usr/bin/python3

import subprocess
import sys
import os

abspath = os.path.abspath(__file__)
dname = os.path.dirname(abspath)

if __name__ == '__main__':
    os.chdir(dname)
    subprocess.call([sys.executable, 'server/build.py'])
    subprocess.call([sys.executable, 'client/build.py'])

