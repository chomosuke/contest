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

    /*
     * Complete the 'minimize' function below.
     *
     * The function is expected to return an INTEGER.
     * The function accepts following parameters:
     * 1. INTEGER_ARRAY point
     * 2. INTEGER k
     */

    public static int minimize(List<Integer> points, int k) {
        Collections.sort(points);
        Integer[] ps = points.toArray(new Integer[0]);
        for (int i = 0; i < ps.length; i++) {
            ps[i] -= k;
        }
        int max = ps[ps.length - 1];
        int minDiff = max - ps[0];
        for (int i = 0; i < ps.length - 1; i++) {
            max = Math.max(ps[i] + 2 * k, max);
            int min = Math.min(ps[0] + 2 * k, ps[i + 1]);
            minDiff = Math.min(minDiff, max - min);
        }
        return minDiff;
    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bufferedWriter = new BufferedWriter(new FileWriter(System.getenv("OUTPUT_PATH")));

        int pointCount = Integer.parseInt(bufferedReader.readLine().trim());

        List<Integer> point = IntStream.range(0, pointCount).mapToObj(i -> {
            try {
                return bufferedReader.readLine().replaceAll("\\s+$", "");
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        })
                .map(String::trim)
                .map(Integer::parseInt)
                .collect(toList());

        int k = Integer.parseInt(bufferedReader.readLine().trim());

        int result = Result.minimize(point, k);

        bufferedWriter.write(String.valueOf(result));
        bufferedWriter.newLine();

        bufferedReader.close();
        bufferedWriter.close();
    }
}
