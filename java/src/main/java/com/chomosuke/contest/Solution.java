package com.chomosuke.contest;

import java.io.*;
import java.math.*;
import java.security.*;
import java.text.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.regex.*;
import java.util.stream.*;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.toList;

class Result {

    private static int findRep(int[] parent, int i) {
        while (parent[i] != i) {
            i = parent[i];
        }
        return i;
    }

    /*
     * Complete the 'getTheGroups' function below.
     *
     * The function is expected to return an INTEGER_ARRAY.
     * The function accepts following parameters:
     * 1. INTEGER n
     * 2. STRING_ARRAY queryType
     * 3. INTEGER_ARRAY students1
     * 4. INTEGER_ARRAY students2
     */

    public static List<Integer> getTheGroups(int n, List<String> queryType, List<Integer> students1,
            List<Integer> students2) {

        // creating union find
        int[] parent = new int[n];
        int[] sizes = new int[n]; // index must be representatives
        for (int i = 0; i < n; i++) {
            parent[i] = i; // at first everyone is in their own group
            sizes[i] = 1;
        }
        int[] sArr1 = new int[students1.size()];
        int[] sArr2 = new int[students2.size()];
        for (int i = 0; i < sArr1.length; i++) {
            sArr1[i] = students1.get(i) - 1; // zero based index
            sArr2[i] = students2.get(i) - 1;
        }
        ArrayList<Integer> results = new ArrayList<>();

        for (int j = 0; j < queryType.size(); j++) {
            int rep1 = findRep(parent, sArr1[j]);
            int rep2 = findRep(parent, sArr2[j]);
            if (queryType.get(j).equals("Friend")) {
                if (rep1 != rep2) {
                    if (sizes[rep1] < sizes[rep2]) {
                        // rep1 smaller than rep2
                        // point rep1 to rep2
                        parent[rep1] = rep2;
                        sizes[rep2] += sizes[rep1];
                    } else {
                        parent[rep2] = rep1;
                        sizes[rep1] += sizes[rep2];
                    }
                }
            } else {
                if (rep1 == rep2) {
                    results.add(sizes[rep1]);
                } else {
                    results.add(sizes[rep1] + sizes[rep2]);
                }
            }
        }

        return results;
    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bufferedWriter = new BufferedWriter(new FileWriter(System.getenv("OUTPUT_PATH")));

        int n = Integer.parseInt(bufferedReader.readLine().trim());

        int queryTypeCount = Integer.parseInt(bufferedReader.readLine().trim());

        List<String> queryType = IntStream.range(0, queryTypeCount).mapToObj(i -> {
            try {
                return bufferedReader.readLine();
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        })
                .collect(toList());

        int students1Count = Integer.parseInt(bufferedReader.readLine().trim());

        List<Integer> students1 = IntStream.range(0, students1Count).mapToObj(i -> {
            try {
                return bufferedReader.readLine().replaceAll("\\s+$", "");
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        })
                .map(String::trim)
                .map(Integer::parseInt)
                .collect(toList());

        int students2Count = Integer.parseInt(bufferedReader.readLine().trim());

        List<Integer> students2 = IntStream.range(0, students2Count).mapToObj(i -> {
            try {
                return bufferedReader.readLine().replaceAll("\\s+$", "");
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        })
                .map(String::trim)
                .map(Integer::parseInt)
                .collect(toList());

        List<Integer> result = Result.getTheGroups(n, queryType, students1, students2);

        bufferedWriter.write(
                result.stream()
                        .map(Object::toString)
                        .collect(joining("\n"))
                        + "\n");

        bufferedReader.close();
        bufferedWriter.close();
    }
}
