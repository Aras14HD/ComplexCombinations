import java.util.Arrays;
import java.util.HashMap;
import java.util.ArrayList;

/**
 * Solver
 */
public class Solver {
    HashMap<ArrayList<Long>, Long> map;

    public static void main(String[] args) {
        ArrayList<Long> arr = new ArrayList<Long>();
        for (int i = 0; i < args.length; i++) {
            arr.add(Long.parseLong(args[i]));
        }
        double before = System.nanoTime();
        Solver s = new Solver();
        long res = s.solve(arr);
        double after = System.nanoTime();
        System.out.println("Took: " + (after - before) / 1000000 + "ms");
        System.out.println(res);
    }

    public Solver() {
        map = new HashMap<ArrayList<Long>, Long>();
    }

    public long solve(ArrayList<Long> arr) {
        System.out.println(arr);
        if (map.containsKey(arr)) {
            System.out.println("Cached!");
            return map.get(arr);
        }
        ArrayList<Long> b = new ArrayList<Long>();
        b.add((long) 1);
        if (arr.equals(b)) {
            System.out.println("End");
            return 1;
        }
        long out = 0;
        for (int i = arr.size() - 1; i >= 0; i--) {
            if (arr.get(i) != 0) {
                ArrayList<Long> copy = new ArrayList<Long>(arr);
                long diff;
                copy.set(i, copy.get(i) - 1);
                if (i == copy.size() - 1 && copy.get(i) == (long) 0) {
                    copy.remove(i);
                }
                diff = i == arr.size() - 1 ? arr.get(i) : arr.get(i) - arr.get(i + 1);
                System.out.println(diff);
                if (diff > 0) {
                    out += diff * this.solve(copy);
                }
            }
        }
        map.put(arr, out);
        System.out.println("Got: " + out);
        return out;
    }
}
