# Jane Street June 2026 Puzzle

## Description

The [June 2026 puzzle](https://www.janestreet.com/puzzles/regional-artwork-index/), "Regional Artwork", gives a list of clue sentences. The instructions indicate that the answers to the clues should be sorted somehow and used to populate the final clue, a series of blank spaces numbered non-sequentially as 2-15.

Clue 3 notes that it's a hint to answering all other clues, and clue 10 notes that its answer will appear last after sorting.

## Solution

The answers to the clues, following from answer 3, all have the first letter ***A*** and the last letter ***I***.

1. Frequent Frazier foe: Muhammad ***ALI***, who fought Joe Frazier three times in heavyweight championship boxing matches.
2. Type of magazine you might start getting in your 20's: An ***ALUMNI*** magazine.
3. Arguably, the story of our time: ***AI***.
4. Stage name of a famous skeptic (with "The"): James Randi, stage magician and debunker of claimed paranormal abilities, performed as The ***AMAZING RANDI***.
5. Oscar-winning role from the 80's: F. Murray Abraham won the Academy Award for Best Actor playing ***ANTONIO SALIERI*** in *Amadeus* (1984).
6. Often-inset pair, for short: Alaska and Hawaii are often displayed as insets on maps of the United States. Taken together, their state abbreviations are ***AK HI***.
7. Microwavable snack brand that replaced a refrigerated product: ***ACT II*** microwave popcorn replaced the original Act I version, which contained real butter and needed to be refrigerated when stored.
8. Where to find "take thou this vial" and "double, double toil and trouble": The quotes are from ***ACT IV SCENE I*** of *Romeo and Juliet* and *Macbeth* respectively.
9. You might break it in Kabul: The currency of Afghanistan is the ***AFGHANI***, although it could be hard to break one in a physical transaction today. The coin worth 1/100th of an afghani, the *pul*, is no longer circulated [per Wikipedia](https://en.wikipedia.org/wiki/Afghan_pul), and there are no coins or bills worth less than one afghani in current use.
10. Eponym of multiple continents: North and South America are named for the Florentine explorer ***AMERIGO VESPUCCI*** (1454-1512).
11. Onetime Outkast output: ***AQUEMINI***, Outkast's third studio album.
12. Notable 2026 launch: The crewed lunar flyby mission ***ARTEMIS II***.
13. The present, in the past: ***ANNO DOMINI***, out of use in favor of "Common Era" as the term for the post-epoch period in the Gregorian calendar.
14. Our nearest neighbors, in some sense (though we haven't visited yet): ***ALPHA CENTAURI***, the triple star system closest to Sol.

The answers, if spaces are omitted, have lengths 2-15. Sorting the clue/answer pairs by ascending answer length, counting the number of positions moved in the list for each entry and taking the character at that position in the answer word gives the letter to be filled in for the final clue in the position corresponding to the answer length.

| Answer          | Length | Positions moved | Letter at move index |
|-----------------|--------|-----------------|----------------------|
| AI              | 2      | 2               | I                    |
| ALI             | 3      | 1               | A                    |
| AKHI            | 4      | 3               | H                    |
| ACTII           | 5      | 3               | T                    |
| ALUMNI          | 6      | 3               | U                    |
| AFGHANI         | 7      | 3               | G                    |
| AQUEMINI        | 8      | 4               | E                    |
| ARTEMISII       | 9      | 4               | E                    |
| ANNODOMINI      | 10     | 4               | O                    |
| ACTIVSCENEI     | 11     | 2               | C                    |
| AMAZINGRANDI    | 12     | 7               | G                    |
| ALPHACENTAURI   | 13     | 2               | L                    |
| ANTONIOSALIERI  | 14     | 8               | S                    |
| AMERIGOVESPUCCI | 15     | 4               | R                    |


The final clue with the letters entered in the spaces indicated by the answer lengths `[7,8,10,15,12,9] [13,6,11,3,14] [4,2,5]` is ***GEORGE LUCAS HIT***. The puzzle solution is ***AMERICAN GRAFITTI***, the 1973 Lucas-directed box office success, agreeing with the puzzle title "Regional Artwork". 


## Discussion

This puzzle must have been quite complicated to compose.

I had a tricky time with this one, which is often the case with the more riddle-like JS puzzles. Clues 1, 11, 13 and 14 are alliterative, and it seemed like that might point to a pattern of some answers being similar (e.g. CASSIUS CLAY for answer 1, or FIRST FOLIO for answer 8, since both *Romeo and Juliet* and *Macbeth* were printed in it). That led me down the wrong path for a while. I then assumed that answer 3 was ARTIFICIAL INTELLIGENCE, and that the pattern that other answers would follow would be two words, one beginning with "A", since that seemed to match many of the other answers. That didn't pan out either.

It took me several days of mulling to catch the "A...I" pattern properly. After that, AFGHANI, ALUMNI and particularly ANNO DOMINI eluded me for a while (A POSTERIORI ("from the later") as an oblique way to refer to the future (i.e. the present) in the past seemed like an outside possibility for answer 13; The clue is pretty vague). 

Once I had the majority of the answers, it seemed very likely that the 2-15 numbering in the final clue corresponded to the lengths of the answer strings. I figured out that the answers needed to sorted by length and mapped to the final clue entries as letters, but I tried to do so by using either the initial clue position or the number of moves to the sorted position indexed into the full alphabet rather than the answer word. Exploring variations on those approaches doesn't yield anything comprehensible, and I put the puzzle aside for a while. I only thought to attempt the correct method after looking over previous puzzles. One step of the August 2025 [Dogs Playing Poker solution](https://www.janestreet.com/puzzles/dogs-playing-poker-solution/) involves indexing characters in words, and that was enough to put me on the right track.
