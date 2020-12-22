import java.util.*;

public class CombinationSum2 {
    public List<List<Integer>> combinationSum2(int[] candidates, int target) {
        HashMap<Integer, Integer> countsMap = new HashMap<>();
        for (int candidate : candidates) {
            countsMap.put(candidate, countsMap.getOrDefault(candidate, 0) + 1);
        }

        // copy over for sorting
        ArrayList<Count> counts = new ArrayList<>();
        for (Integer key : countsMap.keySet()) {
            counts.add(new Count(key, countsMap.get(key)));
        }
        counts.sort(new SortByValue());

        ArrayList<List<Integer>> results = new ArrayList<>();

        Deque<Combination> combinations = new ArrayDeque<>();

        // put all candidates into maybeResults
        for (Count count : counts) {
            Combination combination = new Combination();
            combination.add(count.value);
            combinations.addLast(combination);
        }

        while (!combinations.isEmpty()) {
            Combination combination = combinations.pollFirst();

            // see if this combination is a result
            if (combination.getSum() == target) {
                results.add(combination.getList());
            } else if (combination.getSum() < target) {
                for (int i = bSearch(counts, combination.getMax()); // with sorted counts, to ensure unique combination
                     i < counts.size(); i++) {
                    // get some new combinations
                    Count count = counts.get(i);
                    if (count.count > combination.getCount(count.value)) {
                        // can make new combinations
                        Combination newCombination = combination.copy();
                        newCombination.add(count.value);
                        combinations.addLast(newCombination);
                    }
                }
            }
        }

        return results;
    }

    // -1 means not found
    private int bSearch(ArrayList<Count> counts, Integer target) {
        int start = 0;
        int end = counts.size();
        while(start < end) {
            int mid = (end + start) / 2;
            if (counts.get(mid).value < target) {
                start = mid + 1;
            } else if (counts.get(mid).value > target) {
                end = mid;
            } else {
                return mid;
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] candidates = {14,6,25,9,30,20,33,34,28,30,16,12,31,9,9,12,34,16,25,32,8,7,30,12,33,20,21,29,24,17,27,34,11,17,30,6,32,21,27,17,16,8,24,12,12,28,11,33,10,32,22,13,34,18,12};
        System.out.print(new CombinationSum2().combinationSum2(candidates, 27));
    }
}

class Count {
    public Integer value;
    public Integer count; // bad naming ik but i can't think of any better one
    public Count(Integer value, Integer count) {
        this.value = value;
        this.count = count;
    }
}

class Combination {
    private HashMap<Integer, Integer> countsMap = new HashMap<>();
    public void add(Integer num) {
        if (!arrayList.isEmpty() && num < getMax()) { // comment out when submitting
            throw new RuntimeException("oops");
        }
        countsMap.put(num, getCount(num) + 1);
        sum += num;
        arrayList.add(num);
    }

    private ArrayList<Integer> arrayList = new ArrayList<>();
    public List<Integer> getList() {
        return arrayList;
    }

    private int sum = 0;
    public int getSum() {
        return sum;
    }

    public Integer getCount(Integer num) {
        return countsMap.getOrDefault(num, 0);
    }

    public Combination copy() {
        Combination newCombination = new Combination();

        for (Integer num : arrayList) {
            newCombination.add(num);
        }

        return newCombination;
    }

    public int getMax() {
        return arrayList.get(arrayList.size() - 1);
    }
}

class SortByValue implements Comparator<Count> {
    // Used for sorting in ascending order of value
    public int compare(Count a, Count b) {
        return a.value - b.value;
    }
}