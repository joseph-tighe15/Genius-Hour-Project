from time import sleep
import os
import getch

with open('text/book.txt', 'r') as file:
    content = file.read().replace("\n", "")#.replace(" ", "")
    charecters = {}
    counts = {}
    c = 0
    for i in content:
        if i in charecters.keys():
            counts[i] += 1
        else:
            charecters[i] = c
            c+=1
            counts[i] = 1
    before = charecters
    for i in charecters.keys():
        before[i] = counts.copy()
        for j in list(before[i].keys()):
            before[i][j] = 0
    for j in range(len(content)-2):
        before[content[j]][content[j+1]] += 11
def PredictNext(char):
    maxChar = char
    maxVal = 0
    for i in before[char].keys():
        if before[char][i] > maxVal:
            maxChar = i
            maxVal = before[char][i]
    return maxChar

text = ": "
while True:
    os.system('cls' if os.name == 'nt' else 'clear')
    recomendedWord = ""
    nextChar = text[-1]
    i = 0
    while nextChar != " " and i < 10:
        i+=1
        nextChar = PredictNext(nextChar)
        recomendedWord += nextChar
    recomendedWord = recomendedWord[0:]
    print(text.split(" ")[-1]+recomendedWord)
    print(text)
    char = getch.getch().replace("\x7f", "\b")
    if char != "\b":
        text += char
    else:
        text = text[:-1]