import keyboard
from time import sleep

delay = 0.4

keyboard.write('cd server\ncargo run')
sleep(delay)
keyboard.press_and_release('ctrl+alt+shift+right')
keyboard.press_and_release('alt+shift+left')
sleep(delay)
keyboard.write('D: & cd D:\\\\Files\\Coding\\Rust\\aeonetica\\client\ncargo build & ..\local_testing\copy.bat')
sleep(delay)
keyboard.press_and_release('ctrl+alt+shift+down')
sleep(delay)
keyboard.write('D: & cd D:\\\\Files\\Coding\\Rust\\aeonetica\\local_testing\\client0\nclient.exe')
sleep(delay)
keyboard.press_and_release('ctrl+alt+shift+right')
sleep(delay)
keyboard.write('D: & cd D:\\\\Files\\Coding\\Rust\\aeonetica\\local_testing\\client0\nclient.exe 127.0.0.1:9001 127.0.0.1:6090')
sleep(delay)
for _ in range(3):
    keyboard.press_and_release('alt+shift+up')

keyboard.press_and_release('alt+left')
keyboard.press_and_release('alt+left')
keyboard.press_and_release('ctrl+alt+shift+down')
sleep(delay)
sleep(delay)
keyboard.write('D: & cd D:\\\\Files\\Coding\\Rust\\aeonetica\\mods\nstopwatch "build_player & build_world"')