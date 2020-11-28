data=[int(x) for x in open('input01.txt').readlines()]
m = [x//3 - 2 for x in data]
print(sum(m))



