echo "  Testing Rust (9 9)"
cargo build -r >&-
./target/release/ComplexCombinations 9 9
echo "  With 9 9 9 9 9"
./target/release/ComplexCombinations 9 9 9 9 9

echo "  Testing C (9 9)"
gcc hashmap.o Solver.c
./a.out 9 9

echo "  Testing Java (9 9)"
java Solver.java 9 9

echo "  Testing JavaScript (9 9)"
node Solver.js 9 9
echo "  With 9 9 9 9 9"
node Solver.js 9 9 9 9 9

echo "  Testing Python (9 9)"
source .venv/bin/activate
python Solver.py 9 9
echo "  With 9 9 9 9 9"
python Solver.py 9 9 9 9 9
deactivate
