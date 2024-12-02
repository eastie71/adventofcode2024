"""
Question 2A) Calculate the total safe reports. Each line is considered a report.
To calculate if a line is safe, all levels must be increasing or decreasing AND any 2 adjacent levels must be between 1 and 3 difference.
"""
# For the following 6 report lines
test_entries = ["7 6 4 2 1", # SAFE - all decreasing
                "1 2 7 8 9", # UNSAFE - gap b/w 2 and 7 is > 3
                "9 7 6 2 1", # UNSAFE - gap b/w 6 and 2 is > 3 
                "1 3 2 4 5", # UNSAFE - level increases and then decreases
                "8 6 4 4 1", # UNSAFE - no difference b/w 4 and 4
                "1 3 6 7 9"] # SAFE - all increasing

def is_safe_report_line(results):
    first_num = int(results[0].strip())
    second_num = int(results[1].strip())
    diff = second_num - first_num
    increase = False
    if diff == 0 or abs(diff) > 3:
        return False
    elif diff > 0:
        increase = True
    for i in range(1, len(results)-1):
        first_num = int(results[i].strip())
        second_num = int(results[i+1].strip())
        diff = second_num - first_num
        if diff == 0 or abs(diff) > 3:
            return False
        if diff > 0 and increase == False:
            return False
        if diff < 0 and increase == True:
            return False
    return True

def get_num_of_safe_reports(reports):
    sum = 0
    for report_line in reports:
        results = report_line.split(' ')
        if is_safe_report_line(results):
            sum += 1
    return sum

with open("Q2input.txt", "r") as file:
    entries = [line.strip() for line in file.readlines()]

print(f"2024 - Question 2A: Total number of safe reports = {get_num_of_safe_reports(entries)}")
