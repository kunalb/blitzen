# Blitzen
```                                         
                                            (         )
                                             \(     )/
                                              \(   )/
                                            (\ )---( /)
                                              / a c \
                                              (  o  )
                                               \ ‿ /
```

A helpful reindeer to fetch inputs from — and submit answers to — [Advent of Code](https://adventofcode.com/) quickly. A minimal example:
```
> bzn | ./solution | bzn submit
```

## Installation
```
cargo install --git https://github.com/kunalb/blitzen --branch main
```

## Usage

### Fetching inputs
Simply specify the year and day for your problem, and Blitzen will print all the inputs to stdout. You'll probably want to pipe the input to your solution.
```
> bzn -y 2020 -d 1 fetch
1935
1956
1991
1425
(...snipped...)
```
(`fetch` is also the default and can be omitted.)

### Submitting results
`bzn submit` reads from stdin and quickly flies it to the North pole: you'll need to specify one more argument here for the level (`-l 1` or `-l 2`); it defaults to 1.
```
echo "solution" | bzn -y 2020 -d 2 submit -l 1
```

### Defaults
The reindeer is fairly smart, and will default to the latest problem during the advent: and ask you for help after. Run sometime after midnight, eastern time on December 3rd, it defaults to `-y 2020 -d 3` for both fetch and submit.
```
> bzn
.....#....#...#.#..........#...
...#.................#.........
......#...#.#.#.....#..#.....#.
.........#.#.............##....
(...snipped...)
```

### Initializing
The very first time you ask Blitzen for help you'll need to pass in your session key, which will then get cached for all future use.
```
bzn -y 2020 -d 4 -s <paste your key here>
```

You can look up your key by inspecting the cookies after logging into AoC: copy the value of the `session` cookie [from web inspector](https://developers.google.com/web/tools/chrome-devtools/storage/cookies).

If you want to keep the key out of your shell history, you can also directly create a file at `~/.cache/bzn/session.conf` and paste in the key.
 
### Config & Cache
The session key is saved at `~/.cache/bzn/session.conf`:
```
> ls ~/.config/bzn
session.conf
```

All (successfully fetched) inputs are cached at `~/.cache/bzn/` with a faily complex naming scheme. The cache is *not* reset if the session key is changed, you'll have to do it manually if you have multiple accounts.
```
> ls ~/.cache/bzn
2015_4  2016_4  2019_1  2019_2  2019_3  2020_1  2020_2  2020_3
```


