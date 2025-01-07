import requests, random, time, subprocess
from bs4 import BeautifulSoup

solution = subprocess.run("../rust/target/release/word_puzzle", capture_output=True).stdout.decode("utf-8")
print(solution)

[horizontal_words, vertical_words, *_] = [i.split(",") for i in solution.splitlines()]

horizontal_clues = []
vertical_clues = []

for word in horizontal_words:
    word_page = requests.get(f"https://www.mijnwoordenboek.nl/puzzelwoordenboek/{word}/1")
    # convert to soup
    soup = BeautifulSoup(word_page.text, 'html.parser')
    # find all table rows (these contain div's with the words as strings)
    table_entries = soup.find_all("td")
    # go through all table rows, then for every entry, find all the divs within
    # for every div, get the inner text (some divs aren't words, which is 
    # why the None filter is there)
    candidates = [
        s for i in table_entries 
        for j in i.find_all("div") 
        if (s := j.string) is not None
    ]
    print(candidates)
    if candidates:
        horizontal_clues.append(random.choice(candidates))
    else:
        horizontal_clues.append("geen betekenis gevonden")

    time.sleep(3)

for word in vertical_words:
    word_page = requests.get(f"https://www.mijnwoordenboek.nl/puzzelwoordenboek/{word}/1")
    soup = BeautifulSoup(word_page.text, 'html.parser')
    table_entries = soup.find_all("td")
    candidates = [
        s for i in table_entries 
        for j in i.find_all("div") 
        if (s := j.string) is not None
    ]
    if candidates:
        vertical_clues.append(random.choice(candidates))
    else:
        vertical_clues.append("geen betekenis gevonden")
    time.sleep(3)

# implicit string concatination ftw
print("Horizontaal:\n" + "\n".join([f"{i + 1}: {word}" for (i,word) in enumerate(horizontal_clues)]))
print("Verticaal:\n"   + "\n".join([f"{i + 1}: {word}" for (i,word) in enumerate(vertical_clues)]))
print(f"Oplossing horizontaal: {horizontal_words}")
print(f"Oplossing verticaal: {vertical_words}")
