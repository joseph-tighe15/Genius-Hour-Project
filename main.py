import os
import getch
from collections import defaultdict

checkBefore = 5

def predict_next(context: str, before: dict) -> str:
    max_char = " "
    max_val = 0
    for char, counts in before.items():
        if context in counts and counts[context] > max_val:
            max_char = char
            max_val = counts[context]
    return max_char


def get_before_string(content: str, index: int, before: int) -> str:
    start = max(0, index - before)
    return content[start:index]


def get_before(content: str, search_before: int) -> dict:
    before = {char: defaultdict(int) for char in characters}
    for index in range(search_before, len(content)):
        context = get_before_string(content, index, search_before)
        before[content[index]][context] += 1
    return before


path = "text/"
files = [path + f for f in os.listdir(path) if os.path.isfile(os.path.join(path, f))]
for passage in files:
    with open(passage, 'r', encoding='utf-8') as file:
        content = file.read().replace("\n", "")
        while "  " in content:
            content = content.replace("  ", " ")

        characters = {}
        c = 0
        for char in content:
            if char not in characters:
                characters[char] = c
                c += 1

        beforeList = [get_before(content, i+1) for i in range(0, checkBefore)]

text = ": "
while True:
    os.system('cls' if os.name == 'nt' else 'clear')
    recommended_word = ""
    next_context = text[-checkBefore:]
    i = 0
    c = checkBefore-1
    while text and text[-1] != " " and i < 10:
        i += 1
        next_char = predict_next(next_context, beforeList[c])
        if not next_char:
            c -= 1
            if c < 0:
                break
        if next_char == " ":
            break
        recommended_word += next_char
        next_context = (next_context + next_char)[-checkBefore:]

    print(text.split(" ")[-1] + recommended_word)
    print(text)
    char = getch.getch().replace("\x7f", "\b")
    if char != "\b":
        text += char
    elif len(text) > 0:
        text = text[:-1]