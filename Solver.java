import java.util.concurrent.ConcurrentHashMap;
import java.util.ArrayList;

/**
 * Solver
 */
public class Solver {
    ConcurrentHashMap<ArrayList<Long>, Long> map;

    public static void main(final String[] args) {
        final ArrayList<Long> arr = new ArrayList<Long>();
        for (int i = 0; i < args.length; i++) {
            arr.add(Long.parseLong(args[i]));
        }
        final Solver s = new Solver();
        final double before = System.nanoTime();
        final long res = s.solve(arr);
        final double after = System.nanoTime();
        System.out.println("Took: " + (after - before) / 1000000 + "ms");
        System.out.println("Result: " + res);
    }

    public Solver() {
        map = new ConcurrentHashMap<ArrayList<Long>, Long>();
    }

    public long solve(final ArrayList<Long> arr) {
        // System.out.println(arr);
        if (map.containsKey(arr)) {
            // System.out.println("Cached!");
            return map.get(arr);
        }
        final ArrayList<Long> b = new ArrayList<Long>();
        b.add((long) 1);
        if (arr.equals(b)) {
            // System.out.println("End");
            return 1;
        }
        long out = 0;
        for (int i = arr.size() - 1; i >= 0; i--) {
            if (arr.get(i) != 0) {
                final ArrayList<Long> copy = new ArrayList<Long>(arr);
                long diff;
                copy.set(i, copy.get(i) - 1);
                while (i == copy.size() - 1 && copy.get(i) == (long) 0) {
                    copy.remove(i);
                }
                diff = i == arr.size() - 1 ? arr.get(i) : arr.get(i) - arr.get(i + 1);
                // System.out.println(diff);
                if (diff > 0) {
                    out += diff * this.solve(copy);
                }
            } else {
                System.out.println("Wrong Format!");
                return 0;
            }
        }
        /*
         * map.put(arr, out); System.out.println("Got: " + out);
         */
        return out;
    }
}
