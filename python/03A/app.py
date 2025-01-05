import sys
import os
import re
sys.path.append(os.path.abspath("../common"))
from globals import *

"""
Question 3A) Total sum of all the multiplication result.
Need to search within the puzzle input for the word "mul" with 2 numbers (1-3 digits long) separated by a comma, inside braces. eg. mul(12,345)
For each "mul" found - multiply the 2 numbers and get the sum of ALL of them.
"""
# For the following entry
test_entries = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"]
# gives: mul(2,4) + mul(5,5) + mul(11,8) + mul(8,5)
# = 8 + 25 + 88 + 40 = 161

def get_sum_of_multiplication_results(memory):
    sum = 0
    # find the pattern using Regex
    pattern = r"mul\(\d{1,3},\d{1,3}\)"
    for memory_line in memory:
        # get a list of all the "mul" results
        mul_results = re.findall(pattern, memory_line)
        # print(mul_results)
        for mul in mul_results:
            # get the first and second numbers out of the mul pattern
            num1 = int(mul.split(',')[0].split('(')[1])
            num2 = int(mul.split(',')[1].split(')')[0])
            print(f"{num1} x {num2}")
            sum += (num1 * num2)
    return sum

input_file = INPUT_LOCATION + "Q3input.txt"
with open(input_file, "r") as file:
    entries = [line.strip() for line in file.readlines()]

print(f"2024 - Question 3A: Total sum of all the multiplication results = {get_sum_of_multiplication_results(entries)}")
