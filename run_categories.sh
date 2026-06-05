#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

# Define color codes for beautiful output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}===========================================================${NC}"
echo -e "${BLUE}=== Splitting Benchmark Graphs into Coherent Categories ===${NC}"
echo -e "${BLUE}===========================================================${NC}"

# Default arguments
INPUT_CSV="output.csv"
THEME="dark"

# Parse arguments (supports positional: [csv_file] [theme], and flags: --theme=light)
for arg in "$@"; do
    if [[ "$arg" == "dark" || "$arg" == "light" ]]; then
        THEME="$arg"
    elif [[ "$arg" == --theme=* ]]; then
        THEME="${arg#*=}"
    elif [[ "$arg" == -t=* ]]; then
        THEME="${arg#*=}"
    elif [[ "$arg" == *.csv ]]; then
        INPUT_CSV="$arg"
    else
        if [ -f "$arg" ]; then
            INPUT_CSV="$arg"
        fi
    fi
done

if [ ! -f "$INPUT_CSV" ]; then
    echo -e "${YELLOW}[WARNING] '$INPUT_CSV' not found. Checking root directory...${NC}"
    INPUT_CSV="output.csv"
fi

if [ ! -f "$INPUT_CSV" ]; then
    echo -e "\033[31;1m[ERROR] CSV file '$INPUT_CSV' not found! Please run the benchmarks first to generate it.\033[0m"
    exit 1
fi

echo -e "${GREEN}Using input file: $INPUT_CSV${NC}"
echo -e "${GREEN}Using theme:      $THEME${NC}"

# Clean up previous category runs to ensure fresh charts
rm -rf benchmark_charts/categories

# 1. Bitsets
echo -e "\n${BLUE}--> 1. Generating charts for: Bitsets (Bit-level representations)${NC}"
python3 stat/main.py "$INPUT_CSV" --output-dir benchmark_charts/categories/bitsets --theme "$THEME" --whitelist \
    ByteArraySet \
    SimdBitSet \
    CustomBitSet \
    LibBitSet \
    LibBitVec \
    LibFixedBitSet \
    LibIdlset

# 2. Hash Sets
echo -e "\n${BLUE}--> 2. Generating charts for: Hash Sets (Hash-based O(1) structures)${NC}"
python3 stat/main.py "$INPUT_CSV" --output-dir benchmark_charts/categories/hashsets --theme "$THEME" --whitelist \
    StdHashSet \
    StdHashSetDefaultFunc \
    StdHashSetNoHasher \
    LibFxHashSet \
    LibFxHashSetDefaultFunc

# 3. Trees
echo -e "\n${BLUE}--> 3. Generating charts for: Trees / Sorted Sets (Hierarchical tree structures)${NC}"
python3 stat/main.py "$INPUT_CSV" --output-dir benchmark_charts/categories/trees --theme "$THEME" --whitelist \
    StdTreeSet \
    StdTreeSetDefaultFunc \
    LibAvlTree \
    LibRBTree \
    BinarySearchTree \
    BitTreeSet

# 4. Intervals & Roaring
echo -e "\n${BLUE}--> 4. Generating charts for: Intervals & Roaring (Compressed & Segment sets)${NC}"
python3 stat/main.py "$INPUT_CSV" --output-dir benchmark_charts/categories/intervals_roaring --theme "$THEME" --whitelist \
    IntervalSet \
    LibInterval \
    LibRangeSetBlaze \
    IntervalResourceSet \
    LibRoaring \
    LibCRoaring

# 5. Sequences / Vectors
echo -e "\n${BLUE}--> 5. Generating charts for: Sequences & Vectors (Linear sequential structures)${NC}"
python3 stat/main.py "$INPUT_CSV" --output-dir benchmark_charts/categories/vectors_lists --theme "$THEME" --whitelist \
    StdVec \
    StdVecDicotomie \
    StdLinkedList

echo -e "\n${GREEN}========================================================================${NC}"
echo -e "${GREEN}✓ Success! All categories generated under 'benchmark_charts/categories/'${NC}"
echo -e "${GREEN}========================================================================${NC}"
