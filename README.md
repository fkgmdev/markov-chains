# PHONOTAX

Name comes from *phonotactics* (branch of science that studies speech sounds).

## What

Detects the language of inputted text. Currently only supports:

- English
- German
- Turkish
- French
(Spanish and maybe Italian in the future)

## How

Well, first you train it with some samples (large amounts for each language preferably).
The program analyzes the ratios of vowel-consonant sequences in trigrams. For example vowel-vowel-consonant or consonant-vowel-consonant.
These values (along with the total ratio of consonants and vowels) are averaged per language to form a "profile" of that language.
Since these values tend to differ per language and are pretty consistent, you can reliably detect the language of inputted text if its at least 3-4 words on average.

## Usage Guide

First, get the binary (no release yet, build with cargo or just rustc.).
Then, make a list.txt, in the format of
```
path_to_file-language_of_file
```
(check list.txt in this repository for an example, it is ready to use for all the data in rawdata/)

Then, generate the profiles for each language using
```bash
phonotax train -i list.txt -o profiles.json
```
Your profiles for each language are now ready.
Run
```bash
phonotax detect -t "text to be language-detected"
```
to detect from inputted text or
```bash
phonotax detect -i file_path
```
to detect from a file.

### Note:
The detect command also has a -d argument, if your profile list isn't named the default filename (profiles.json), you have to specify the path for it using -d profiles_list_path.
