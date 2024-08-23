import pandas
import sqlite3
from itertools import zip_longest

connection = sqlite3.connect("./bible.db")

books = pandas.read_sql_query("SELECT * FROM books", connection)
verses = pandas.read_sql_query("SELECT * FROM verses", connection)
#print(verses)

# verses
# book_number, chapter, text

# {{{ Generate meta for words
# Split verses into words, and separate puctuation and quotation.
word_lists = verses['text'].astype(str)
book_nums = verses['book_number']
chapter_nums = verses['chapter']
verse_nums = verses['verse']
processed_words = []
for line, book_num, chapter_num, verse_num in zip_longest(word_lists, book_nums, chapter_nums, verse_nums):
    words = line.split()
    for word in words:
        processed_word = {}
        word: str

        # " may prefix a word.
        if word.startswith('"'):
            processed_word["starts_with_dquote"] = 1
            word = word[1:]
        elif word.startswith("'"):
            processed_word["starts_with_quote"] = 1
        elif word.startswith("("):
            processed_word["starts_with_lparen"] = 1

        ## DOUBLE QUOTE 2-CHR ENDINGS
        # ": may postfix a word.
        if word.endswith('":'):
            processed_word["ends_with_dquote"] = 1
            processed_word["ends_with_colon"] = 2
            word = word[:-2]

        # ". may postfix a word.
        elif word.endswith('".'):
            processed_word["ends_with_dquote"] = 1
            processed_word["ends_with_period"] = 2
            word = word[:-2]
        
        # ." may postfix a word.
        elif word.endswith('."'):
            processed_word["ends_with_period"] = 1
            processed_word["ends_with_dquote"] = 2
            word = word[:-2]

        # ", may postfix a word.
        elif word.endswith('",'):
            processed_word["ends_with_dquote"] = 1
            processed_word["ends_with_comma"] = 2
            word = word[:-2]

        # ," may postfix a word.
        elif word.endswith(',"'):
            processed_word["ends_with_comma"] = 1
            processed_word["ends_with_dquote"] = 2
            word = word[:-2]

        ## SINGLE QUOTE 2-CHR ENDINGS
        # ': may postfix a word.
        elif word.endswith("':"):
            processed_word["ends_with_quote"] = 1
            processed_word["ends_with_colon"] = 2
            word = word[:-2]

        # '. may postfix a word.
        elif word.endswith("'."):
            processed_word["ends_with_quote"] = 1
            processed_word["ends_with_comma"] = 2
            word = word[:-2]
        
        # .' may postfix a word.
        elif word.endswith(".'"):
            processed_word["ends_with_period"] = 1
            processed_word["ends_with_quote"] = 2
            word = word[:-2]

        # ', may postfix a word.
        elif word.endswith("',"):
            processed_word["ends_with_quote"] = 1
            processed_word["ends_with_comma"] = 2
            word = word[:-2]

        # ,' may postfix a word.
        elif word.endswith(",'"):
            processed_word["ends_with_comma"] = 1
            processed_word["ends_with_quote"] = 2
            word = word[:-2]

        ## SINGULAR PUNCTUATION GRAPHEMES
        elif word.endswith(";"):
            processed_word["ends_with_semicolon"] = 1
            word = word[:-1]
        elif word.endswith("."):
            processed_word["ends_with_period"] = 1
            word = word[:-1]
        elif word.endswith(","):
            processed_word["ends_with_comma"] = 1
            word = word[:-1]
        elif word.endswith(":"):
            processed_word["ends_with_colon"] = 1
            word = word[:-1]
        elif word.endswith("'"):
            processed_word["ends_with_quote"] = 1
            word = word[:-1]
        elif word.endswith('"'):
            processed_word["ends_with_dquote"] = 1
            word = word[:-1]
        elif word.endswith(')'):
            processed_word["ends_with_rparen"] = 1
            word = word[:-1]

        processed_word["word"] = word
        processed_word["ends_with_rparen"] =  processed_word.get("ends_with_rparen", 0)
        processed_word["ends_with_colon"] =   processed_word.get("ends_with_colon", 0)
        processed_word["ends_with_period"] =  processed_word.get("ends_with_period", 0)
        processed_word["ends_with_comma"] =   processed_word.get("ends_with_comma", 0)
        processed_word["ends_with_quote"] =   processed_word.get("ends_with_quote", 0)
        processed_word["ends_with_dquote"] = processed_word.get("ends_with_dquote", 0)
        processed_word["ends_with_semicolon"] = processed_word.get("ends_with_semicolon", 0)
        processed_word["starts_with_quote"] = processed_word.get("starts_with_quote", 0)
        processed_word["starts_with_dquote"] = processed_word.get("starts_with_dquote", 0)
        processed_word["starts_with_lparen"] = processed_word.get("starts_with_lparen", 0)
        processed_word["book_number"] = book_num
        processed_word["chapter_number"] = chapter_num
        processed_word["verse_number"] = verse_num
        processed_words.append(processed_word)
# }}}

df = pandas.DataFrame(processed_words)
print(df.iloc[20:40][["word", "ends_with_period", "ends_with_dquote"]])
#df.to_sql("new_words", connection)
#print(verses["chapter"] == 1)


