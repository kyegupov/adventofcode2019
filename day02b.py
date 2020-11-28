a0=[int(x) for x in open('input02.txt').read().split(',')]
xx = {}
for noun in range(10):
    for verb in range(10):
        a = a0[:]
        a[1]=noun
        a[2]=verb
        i = 0
        while a[i] != 99:
            if a[i] == 1:
                a[a[i+3]] = a[a[i+1]] + a[a[i+2]]
                i += 4

            elif a[i] == 2:
                a[a[i+3]] = a[a[i+1]] * a[a[i+2]]
                i += 4

        xx[(noun,verb)] = a[0]

k = (xx[(1,0)]-xx[(0,0)])
b = xx[(0,0)]

noun = (19690720-b) // k
verb = (19690720-b) % k

print(noun*100+verb)

a = a0[:]
a[1]=noun
a[2]=verb
i = 0
while a[i] != 99:
    if a[i] == 1:
        a[a[i+3]] = a[a[i+1]] + a[a[i+2]]
        i += 4

    elif a[i] == 2:
        a[a[i+3]] = a[a[i+1]] * a[a[i+2]]
        i += 4

print(a[0])