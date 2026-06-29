# PLACEHOLDER TITLE (for now, i hope)
This is a project of mine, inspired by a Veritasium video where Andrey Markov's work on phonotactics is mentioned. Basically the guy took a russian poem and analyzed it as such:
- What's the probability of a consonant following a vowel?
- Or a vowel following a consonant?
- Or a double consonant?
- Or even a double vowel?
This got me thinking, isn't this very similar to the frequency of a language?
If so, then these probabilites would be unique to every language and be a sort of "fingerprint" for them.
This would make it possible to identify them through these ratios.

**SCROLL TO BOTTOM FOR THE DATA**

## Goal for the project
My goal for this project is to build an algorithm that uses collected data on multiple languages with about ~15 novels per language (fetched from Project Gutenberg) and predicts what language the inputted text is.

## Where the project currently is
I've currently gathered data for 15 English and 15 French novels. Some placeholder data for German and Turkish also exists but they will be replaced by solid data

## TO-DO
- Collect novels for ~~English, French~~, Turkish, Spanish, Italian and German.
- Analyze data from said novels
- Calculate needed values for the data per language (mean, standard deviaton etc.)
- Build a simple algorithm to detect language
- Replace algorithm with a more complex, trained algorithm

## How to run

### TLDR:
don't run it, it takes too long if you don't already have all that you need.
just grab or look at data.txt in this repo.

### I'm a masochist:
Well, first of all clone this repository
```bash
git clone https://github.com/fkgmdev/markov-chains.git
```

Secondly, get Rust along with the Cargo package manager (the ways depend on your platform)

Then, clone my other repo I made for fetching from Project Gutenberg
```bash
git clone https://github.com/fkgmdev/gutenberg-fetcher.git
```

Go into the last repo and copy over the .txt files into the first repo in the following structure:
```folder structure
markov-chains
|
|__rawdata
    |_english
    | |_all the english .txt files
    |
    |_french
    | |_all french .txt files
```

and so on...

Then, build the rust project:
```bash
cargo build --release
```

You're close now, then make a file named list.txt in markov-chains, then run
```bash
./target/release/markov-chains list add rawdata/*/*
```

Lastly, run this to generate the data
```bash
./target/release/markov-chains redo
```

Your data should be in data.txt. Enjoy!

## DATA
```data.txt
english  CC:25.0515% CV:35.2766% VC:34.9712% VV:4.7007% C/V:175.2272% Path: call_of_the_wild.txt
english  CC:22.5721% CV:36.3765% VC:35.2938% VV:5.7576% C/V:160.2350% Path: dorian_gray.txt
english  CC:23.3569% CV:36.1376% VC:35.1008% VV:5.4047% C/V:163.1231% Path: dracula.txt
english  CC:23.8568% CV:35.4565% VC:35.4453% VV:5.2414% C/V:161.2875% Path: frankenstein.txt
english  CC:23.4190% CV:35.7860% VC:35.2512% VV:5.5438% C/V:161.9923% Path: great_expectations.txt
english  CC:23.9228% CV:35.4986% VC:35.1220% VV:5.4566% C/V:162.1921% Path: heart_of_darkness.txt
english  CC:23.7152% CV:35.7159% VC:35.0854% VV:5.4835% C/V:166.2859% Path: huckleberry_finn.txt
english  CC:23.2541% CV:35.9969% VC:35.0348% VV:5.7142% C/V:159.5648% Path: jane_eyre.txt
english  CC:24.4255% CV:35.2518% VC:35.1078% VV:5.2149% C/V:167.2216% Path: moby_dick.txt
english  CC:23.2027% CV:35.8146% VC:35.6915% VV:5.2912% C/V:163.1238% Path: pride_prejudice.txt
english  CC:23.7455% CV:35.7022% VC:35.3489% VV:5.2034% C/V:163.0868% Path: robinson_crusoe.txt
english  CC:24.5399% CV:35.2601% VC:35.4071% VV:4.7929% C/V:166.9418% Path: scarlet_letter.txt
english  CC:23.9307% CV:35.7309% VC:35.0340% VV:5.3044% C/V:168.9119% Path: tom_sawyer.txt
english  CC:23.4827% CV:35.8201% VC:35.2812% VV:5.4159% C/V:163.9769% Path: two_cities.txt
english  CC:23.7324% CV:35.8086% VC:34.8175% VV:5.6415% C/V:162.5962% Path: wuthering_heights.txt
french  CC:14.4850% CV:39.9541% VC:33.2138% VV:12.3471% C/V:120.8040% Path: camelias.txt
french  CC:15.1980% CV:39.5746% VC:34.2074% VV:11.0200% C/V:125.9749% Path: candide.txt
french  CC:16.6628% CV:38.2119% VC:33.6586% VV:11.4667% C/V:127.5330% Path: essaisv1.txt
french  CC:16.7435% CV:38.2184% VC:33.6937% VV:11.3444% C/V:127.5836% Path: essaisv2.txt
french  CC:15.5571% CV:39.4277% VC:33.4859% VV:11.5293% C/V:124.5563% Path: fantome.txt
french  CC:15.4310% CV:38.8668% VC:33.7615% VV:11.9406% C/V:124.1432% Path: germinal.txt
french  CC:14.8439% CV:39.9489% VC:33.9493% VV:11.2579% C/V:124.0970% Path: le_rouge.txt
french  CC:14.8442% CV:40.2017% VC:32.9393% VV:12.0148% C/V:122.4471% Path: liaisons.txt
french  CC:15.7754% CV:38.7014% VC:34.3273% VV:11.1959% C/V:126.6888% Path: madame_bovary.txt
french  CC:15.1734% CV:39.3809% VC:34.0831% VV:11.3626% C/V:124.8457% Path: monte_cristo.txt
french  CC:16.3920% CV:38.2632% VC:34.6895% VV:10.6553% C/V:129.7815% Path: mysterieuse.txt
french  CC:15.7862% CV:39.5791% VC:33.5192% VV:11.1155% C/V:124.6937% Path: notre_dame.txt
french  CC:15.3425% CV:39.0936% VC:33.8539% VV:11.7101% C/V:122.9944% Path: swann.txt
french  CC:15.3291% CV:39.2055% VC:34.0852% VV:11.3802% C/V:125.3816% Path: trois_mousquetaires.txt
french  CC:15.6128% CV:39.3059% VC:34.4538% VV:10.6275% C/V:128.2711% Path: twenty_leagues.txt
german  CC:25.2971% CV:33.1499% VC:34.9896% VV:6.5634% C/V:175.4920% Path: faust.txt
german  CC:23.3498% CV:34.2453% VC:35.6651% VV:6.7398% C/V:160.7827% Path: mabuse.txt
turkish  CC:13.4094% CV:45.5268% VC:40.8600% VV:0.2038% C/V:137.3775% Path: aigenturkish.txt
turkish  CC:12.0489% CV:46.3965% VC:41.1126% VV:0.4419% C/V:131.3470% Path: nutuk.txt
```