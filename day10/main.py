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



def count_mismatched(line):
    points = {
        "]": 57,
        ")": 3,
        "}": 1197,
        ">": 25137
    }
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

def score_incomplete_lines(line):
    points = {
        "]": 2,
        ")": 1,
        "}": 3,
        ">": 4
    }
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
                return 0
    # Calculate score from left over stack chars
    score = 0
    while stack:
        score *= 5
        score += points[opposite[stack.pop()]]
    return score


def a():
    df1 = spark.read.text("input.txt")
    score = df1.rdd.map(count_mismatched)
    print(f"a: {score.sum()}")

def b():
    df1 = spark.read.text("input.txt")
    scores = df1.rdd.map(score_incomplete_lines).filter(lambda x:x!=0).collect()
    scores.sort()
    median = scores[len(scores)//2]
    print(f"b: {median}")

if __name__ == '__main__':
    a()
    b()