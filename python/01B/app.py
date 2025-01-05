import sys
import os
sys.path.append(os.path.abspath("../common"))
from globals import *

"""
Question 1B) Calculate the total similarity score
To calculate the total similarity score you multiply each number in the LEFT list by the number of times that number appears in the RIGHT list, and sum all together.
"""
# For the following 6 lines, values are: (3 * 3), (4 * 1), (2 * 0), (1 * 0), (3 * 3), (3 * 3). ie. 9 + 4 + 0 + 0 + 9 + 9 = 31 
test_entries = ["3   4", "4   3", "2   5",  "1   3", "3   9", "3   3"]


def get_similarity_score(values):
    sum = 0
    left_list = []
    right_list = []
    for line in values:
        # Split line based on the 3 spaces apart for every line
        left_list.append(int(line.split('   ')[0].strip()));
        right_list.append(int(line.split('   ')[1].strip()));

    # Sort the left and the right lists
    left_list.sort()
    right_list.sort()

    # print(f"Sorted Left list = {left_list}")
    # print(f"Sorted Right list = {right_list}")  

    last_id = -1
    last_score = 0
    for i in range(0, len(left_list)):
        if last_id != left_list[i]: 
            count = 0
            # count number of times left list number appears in the right list
            for right_id in right_list:
                if right_id == left_list[i]:
                    count += 1
            last_score = left_list[i] * count
            last_id = left_list[i]
        sum += last_score

    return sum

input_file = INPUT_LOCATION + "Q1input.txt"
with open(input_file, "r") as file:
    entries = [line.strip() for line in file.readlines()]

print(f"2024 - Question 1B: Total similarity score for the location IDs = {get_similarity_score(entries)}")
