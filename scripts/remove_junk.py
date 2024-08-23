
import pandas as pd
import sqlite3
from itertools import zip_longest
import re

connection = sqlite3.connect("./bible.db")

verses = pd.read_sql_query("SELECT * FROM verses", connection)
#words = pd.read_sql_query("SELECT * FROM words", connection)
lala = re.compile(r"(\(H\d+:\d+\))")

print(verses.replace(r"(\(H\d+:\d+\))", "", regex=True, inplace=True))

print(verses)

verses.to_sql("new_verses", connection)

#text_column = verses["text"]
#text_column.apply(lambda x: lala.sub(x, ""))

#verses.to_sql("new_verses", connection)


#print(text_column)
# 543921


