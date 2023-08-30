import os
import json

path = "draw_pics/threshold_files/encrypted"

files = os.listdir(path)
for file in files :
    if not os.path.isdir(file):
        f = open(path + "/" + file)
        line = f.readline()
        while line:
            data = json.loads(line)
            print(data["path"])
            line = f.readline()
    break

