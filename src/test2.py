column, row = map(int, input().split())

x = 0b1<<((row+1) * 8 - (column+1))

print(bin(x))
print(len(str(bin(x)))-2)
lsst = []
counter = -3
for i in str(bin(x)):
    counter += 1
    if counter == 0:
        lsst.append(i)
        continue

    if counter%8==0:
        lsst.append("_")
    lsst.append(i)

print(*lsst)
# for i in range(200):
    # print(i%10)