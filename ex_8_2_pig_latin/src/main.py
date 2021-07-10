def main():
    prompt = """================================================================
             Convert strings to pig latin.
================================================================
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

Please input a word:
"""
    x = input(prompt)

    print(pig_latin(x))

def pig_latin(s):
    if len(s) == 0:
        return ""

    first_char = s[0]
    if is_vowel(first_char):
        return s + "-hay"
    else:
        return s[1:] + '-' + first_char + "ay"

def is_vowel(c):
    if c in "aeiouAEIOU":
        return True
    else:
        return False

if __name__ == "__main__":
    main()
