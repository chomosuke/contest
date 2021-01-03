
import java.util.*;

class Solution2 {
    public static void main (String[] args) {
        Scanner scanner = new Scanner(System.in);

        int t = scanner.nextInt();

        for (int i = 0; i < t; i++) {
            int n = scanner.nextInt();
            int q = scanner.nextInt();

            int[] d = new int[n-1];
            for (int i2 = 0; i2 < n-1; i2++) {
                d[i2] = scanner.nextInt();
            }

            hm = new HashMap<>();

            System.out.print("Case #" + (i + 1) + ":");
            for (int i2 = 0; i2 < q; i2++) {
                System.out.print(" " +
                        query(d, scanner.nextInt(), scanner.nextInt()));
            }
            System.out.print("\n");
        }
    }

    private static HashMap<Integer, State> hm;
    private static int query(int[] d, int s, int k) {
        s--;
        State state;
        if (hm.containsKey(s)) {
            state = hm.get(s);
        } else {
            state = new State(d, s);
            hm.put(s, state);
        }
        return state.compute(k) + 1;
    }
}

class State {
    private int l, r;
    private int[] d;
    private ArrayList<Integer> roomVisited = new ArrayList<>();
    public State(int[] d, int s) {
        l = s-1;
        r = s;
        this.d = d;
        roomVisited.add(s);
    }

    public int compute(int k) {
        while (k > roomVisited.size()) {
            if (l < 0) {
                r++;
                roomVisited.add(r);
            } else if (r >= d.length) {
                l--;
                roomVisited.add(l+1);
            } else if (d[l] < d[r]) {
                l--;
                roomVisited.add(l+1);
            } else {
                r++;
                roomVisited.add(r);
            }
        }
        return roomVisited.get(k - 1);
    }
}