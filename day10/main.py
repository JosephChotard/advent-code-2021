from pyspark.sql import SparkSession
import pyspark.sql.functions as F
from collections import deque



spark = SparkSession \
    .builder \
    .appName("Advent code day 10") \
    .getOrCreate()

opposite = {
    "[": "]",
    "(": ")",
    "<": ">",
    "{": "}"
}

points = {
    "]": 57,
    ")": 3,
    "}": 1197,
    ">": 25137
}

def count_mismatched(line):
    stack = deque()
    for char in line[0]:
        if char not in "[]()<>{}":
            raise ValueError(f"character {char} not in set of valid chars")
        if char in opposite.keys():
            stack.append(char)
        else:
            if opposite[stack[-1]] == char:
                stack.pop()
            else:
                return points[char]
    return 0

def a():
    df1 = spark.read.text("input.txt")
    score = df1.rdd.map(count_mismatched)
    print(score.sum())



if __name__ == '__main__':
    a()