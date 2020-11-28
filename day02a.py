a=[int(x) for x in open('input02.txt').read().split(',')]
a[1]=12
a[2]=2
i = 0
while a[i] != 99:
    if a[i] == 1:
        a[a[i+3]] = a[a[i+1]] + a[a[i+2]]
        i += 4

    elif a[i] == 2:
        a[a[i+3]] = a[a[i+1]] * a[a[i+2]]
        i += 4

print(a[0])
