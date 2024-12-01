"""
Question 1A) Calculate the total difference between the left and right lists of location IDs
To calculate the distance you must get the lowest number from the left listand compare it with the lowest number of the right list, and then sum all the differences for every line.
"""
# For the following 6 lines, values to compare are (1,3),(2,3),(3,3),(3,4),(3,5) and (4,9). 2 + 1 + 0 + 2 + 2 + 5 = 11
test_entries = ["3   4", "4   3", "2   5",  "1   3", "3   9", "3   3"]


def get_distance_sum(values):
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

    for i in range(0, len(left_list)):
        # print(f"i = {i} left = {left_list[i]}, right = {right_list[i]}")
        # difference needs to be an absolute distance
        sum += abs(right_list[i] - left_list[i])
    return sum

with open("Q1input.txt", "r") as file:
    entries = [line.strip() for line in file.readlines()]

print(f"2024 - Question 1A: Total distance between location IDs = {get_distance_sum(entries)}")
