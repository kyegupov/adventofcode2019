data=[int(x) for x in open('input01.txt').readlines()]
s = 0
for x in data:
    x2 = x//3 - 2
    while x2 > 0:
        s += x2
        x = x2
        x2 = x//3 - 2
print(s)



