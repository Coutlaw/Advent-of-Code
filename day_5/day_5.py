data = open("./day_5.txt", "r").read().split("\n")

rows = list(range(128))
columns = list(range(8))
row, row_low, column_low, column = 0, 0 , 0 , 0
row_high = 127
column_high = 7

seatIds = []
for i in range(0, len(data)):
    moves = data[i]
    # first 7 chars define the row
    for j in range(0, 7):
        if moves[j] == "F":
            row_high = int(((row_high + row_low + 1) / 2)) - 1 # final -1 to offset for index
        else:
            row_low = int(((row_high + row_low + 1) / 2))
    row = row_low

    # next 3 define the column
    for k in range(7, 10):
        if moves[k] == "L":
            column_high = int(((column_high + column_low + 1) / 2)) - 1 # final -1 to offset for index
        else:
            column_low = int(((column_high + column_low + 1) / 2))
    column = column_low

    seatIds.append(int((row * 8) + column))
    # reset the loop vars    
    row, row_low, column_low, column = 0, 0 , 0 , 0
    row_high = 127
    column_high = 7
seatIds.sort()
for i in range(0, len(seatIds)): 
        if (seatIds[i] - seatIds[i-1] > 1) : 
            print(seatIds[i-1] + 1)