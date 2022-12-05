
# win = 6
# draw = 3
# loss = 0
# rock = 1 = A
# paper = 2 = B
# scissor = 3 = C
# X = LOSE
# Y = DRAW
# Z = WIN
score_map = {
    ("A", "X"): 3,
    ("A", "Y"): 4,
    ("A", "Z"): 8,
    ("B", "X"): 1,
    ("B", "Y"): 5,
    ("B", "Z"): 9,
    ("C", "X"): 2,
    ("C", "Y"): 6,
    ("C", "Z"): 7,
}

# A beats

with open("input.txt", "r") as fp:
    rounds = fp.readlines()

score = 0
for my_round in rounds:
    my_round = my_round.rstrip("\n")
    opponent, me = my_round.split(" ")
    print(opponent, me)
    score += score_map[(opponent, me)]

print(score)

