
# win = 6
# draw = 3
# loss = 0
# rock = 1 = x
# paper = 2 = y
# scissor = 3 = z
score_map = {
    ("A", "X"): 4,
    ("A", "Y"): 8,
    ("A", "Z"): 3,
    ("B", "X"): 1,
    ("B", "Y"): 5,
    ("B", "Z"): 9,
    ("C", "X"): 7,
    ("C", "Y"): 2,
    ("C", "Z"): 6,
}




with open("input.txt", "r") as fp:
    rounds = fp.readlines()

score = 0
for my_round in rounds:
    my_round = my_round.rstrip("\n")
    opponent, me = my_round.split(" ")
    print(opponent, me)
    score += score_map[(opponent, me)]

print(score)

