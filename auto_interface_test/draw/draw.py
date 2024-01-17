#!/usr/bin/python2

import os
import json
import matplotlib.pyplot as plt
import numpy as np

enc_path = "diff_type/encrypted"
ori_path = "diff_type/original"

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
    folder_list = data["file"].split('/')
    label.append(folder_list[len(folder_list) - 1])
    chi_sq_o.append(data["chisquare"])
    entropy_o.append(data["entropy"])
    mean_o.append(data["mean"])
    monte_o.append(data["montecarlo"])
    serial_o.append(data["serial"])

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
                chi_sq_e.append(data["chisquare"])
            else :
                chi_sq_e[i] = chi_sq_e[i] + data["chisquare"]
            # mean
            if len(mean_e)!=len(label) :
                mean_e.append(data["mean"])
            else :
                mean_e[i] = mean_e[i] + data["mean"]
            # monte
            if len(monte_e)!=len(label) :
                monte_e.append(data["montecarlo"])
            else :
                monte_e[i] = monte_e[i] + data["montecarlo"]
            # serial
            if len(serial_e)!=len(label) :
                serial_e.append(data["serial"])
            else :
                serial_e[i] = serial_e[i] + data["serial"]
            i += 1

for i in range(0, len(entropy_e)) :
    chi_sq_e[i] = chi_sq_e[i]/len(files)
    entropy_e[i] = entropy_e[i]/len(files)
    mean_e[i] = mean_e[i]/len(files)
    monte_e[i] = monte_e[i]/len(files)
    serial_e[i] = serial_e[i]/len(files)

# print(label)
# print(monte_e)
# print(monte_o)

values = (chi_sq_e, entropy_e, mean_e, monte_e, serial_e)
values_o = (chi_sq_o, entropy_o, mean_o, monte_o, serial_o)
name = (['Chi_Square', 'Entropy', 'Mean', 'Monte_Carlo_Pi', 'Serial'])

bar_width = 0.35

bar_positions_e = np.arange(len(label))
bar_positions_o = bar_positions_e + bar_width

for i in range(0, len(values)) :
    plt.bar(bar_positions_e, values[i], width=bar_width, color='orange', label='encrypted')
    plt.bar(bar_positions_o, values_o[i], width=bar_width, color='blue', label='plain')
    
    plt.title('Different text length')
    plt.xlabel('Length')
    plt.ylabel(name[i])

    plt.legend()
    plt.xticks(bar_positions_e + bar_width / 2, label)
    
    filename = name[i] + '.png'
    plt.savefig(filename)

    plt.clf()







