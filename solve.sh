if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    echo "Usage: $0 YEAR DAY PART"
    echo "Submits solution for specified Advent of Code year/day/part"
    echo
    echo "Arguments:"
    echo "  YEAR    Year of puzzle (e.g. 2023)"
    echo "  DAY     Day of puzzle (1-25)"
    echo "  PART    Part of puzzle (1 or 2)"
    exit 1
fi

YEAR=$1
DAY=$2
PART=$3
aoc s -d $DAY -y $YEAR $PART $(cargo run -- $YEAR $DAY $PART)