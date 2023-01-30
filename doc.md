# Output format 
Explanation about the main output files from pan-sv. Some additional files are not listed here.


## Bubble stats (prefix.bubble.stats)

| Col | Type  | Description                  |
|-----|-------|------------------------------|
| 1   | int   | Bubble identifier            |
| 2   | int   | Start node                   |
| 3   | int   | End node                     |
| 4   | int   | Minimal length of the bubble |
| 5   | int   | Maximum length of the bubble |
| 6   | float | Mean length of the bubble    |
| 7   | int   | Number of traversals         | 

Tags: 
- NL: Nestedness level (depth in a bubble)



## Bed output
Since traversals are specific to each bubble, identifier are always starting by 0. 
| Col | Type   | Description     |
|-----|--------|-----------------|
| 1   | String | Genome name     |
| 2   | int    | Start position  |
| 3   | int    | End position    |
| 4   | int    | Bubble id       |
| 5   | int    | Traversal id    |