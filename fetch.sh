if [ -z "$1" ] || [ -z "$2" ] || [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
    echo "Usage: $0 YEAR DAY"
    echo "Downloads Advent of Code input and problem description for specified year and day"
    echo
    echo "Arguments:"
    echo "  YEAR    Year of puzzle (e.g. 2023)"
    echo "  DAY     Day of puzzle (1-25)"
    exit 1
fi

YEAR=$1
DAY=$2

mkdir -p src/input/y$YEAR
aoc download -y $YEAR -d $DAY -i src/input/y$YEAR/day$(printf "%02d" $DAY).txt -p src/input/y$YEAR/day$(printf "%02d" $DAY).md