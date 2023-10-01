#!/usr/bin/python2

import os
import json
import matplotlib.pyplot as plt
import numpy as np

enc_path = "diff_lag/encrypted"
ori_path = "diff_lag/original"

ori_file = open(ori_path)

label = []
# original data
chi_sq_o = []
entropy_o = []
mean_o = []
monte_o = []
serial_o = []
# encrypted data
chi_sq_e = []
entropy_e = []
mean_e = []
monte_e = []
serial_e = []



# read original file
lines = ori_file.readlines()
for line in lines :
    data = json.loads(line)
    folder_list = data["path"].split('/')
    label.append(folder_list[len(folder_list) - 1])
    chi_sq_o.append(data["chi_square"])
    entropy_o.append(data["entropy"])
    mean_o.append(data["mean"])
    monte_o.append(data["monte_carlo_pi"])
    serial_o.append(data["serial_correlation"])

files = os.listdir(enc_path)

for file in files :
    i = 0
    if not os.path.isdir(file):
        f = open(enc_path + "/" + file)
        lines = f.readlines()
        for line in lines :
            data = json.loads(line)
            # entropy
            if len(entropy_e)!=len(label) :
                entropy_e.append(data["entropy"])
            else :
                entropy_e[i] = entropy_e[i] + data["entropy"]
            # chi
            if len(chi_sq_e)!=len(label) :
                chi_sq_e.append(data["chi_square"])
            else :
                chi_sq_e[i] = chi_sq_e[i] + data["chi_square"]
            # mean
            if len(mean_e)!=len(label) :
                mean_e.append(data["mean"])
            else :
                mean_e[i] = mean_e[i] + data["mean"]
            # monte
            if len(monte_e)!=len(label) :
                monte_e.append(data["monte_carlo_pi"])
            else :
                monte_e[i] = monte_e[i] + data["monte_carlo_pi"]
            # serial
            if len(serial_e)!=len(label) :
                serial_e.append(data["serial_correlation"])
            else :
                serial_e[i] = serial_e[i] + data["serial_correlation"]
            i += 1

for i in range(0, len(entropy_e)) :
    chi_sq_e[i] = chi_sq_e[i]/len(files)
    entropy_e[i] = entropy_e[i]/len(files)
    mean_e[i] = mean_e[i]/len(files)
    monte_e[i] = monte_e[i]/len(files)
    serial_e[i] = serial_e[i]/len(files)

print(label)
print(monte_e)
print(monte_o)
