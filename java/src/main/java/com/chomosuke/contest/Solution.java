package com.chomosuke.contest;

// BufferedWriter bufferedWriter = new BufferedWriter(new FileWriter(System.getenv("OUTPUT_PATH")));

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
     * Complete the 'countTeams' function below.
     *
     * The function is expected to return an INTEGER_ARRAY.
     * The function accepts following parameters:
     * 1. INTEGER_ARRAY rating
     * 2. 2D_INTEGER_ARRAY queries
     */

    public static List<Integer> countTeams(List<Integer> ratings, List<List<Integer>> queries) {
        ArrayList<Integer> groupCounts = new ArrayList<>();
        for (List<Integer> query : queries) {
            HashMap<Integer, Integer> ratingCounts = new HashMap<>();
            for (int i = query.get(0) - 1; i < query.get(1); i++) {
                ratingCounts.put(ratings.get(i), ratingCounts.getOrDefault(ratings.get(i), 0) + 1);
            }
            Integer groupCount = 0;
            for (Integer groups : ratingCounts.values()) {
                groupCount += groups / 2;
            }
            groupCounts.add(groupCount);
        }
        return groupCounts;
    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bufferedWriter = new BufferedWriter(new FileWriter(System.getenv("OUTPUT_PATH")));

        int ratingCount = Integer.parseInt(bufferedReader.readLine().trim());

        List<Integer> rating = IntStream.range(0, ratingCount).mapToObj(i -> {
            try {
                return bufferedReader.readLine().replaceAll("\\s+$", "");
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        })
                .map(String::trim)
                .map(Integer::parseInt)
                .collect(toList());

        int queriesRows = Integer.parseInt(bufferedReader.readLine().trim());
        int queriesColumns = Integer.parseInt(bufferedReader.readLine().trim());

        List<List<Integer>> queries = new ArrayList<>();

        IntStream.range(0, queriesRows).forEach(i -> {
            try {
                queries.add(
                        Stream.of(bufferedReader.readLine().replaceAll("\\s+$", "").split(" "))
                                .map(Integer::parseInt)
                                .collect(toList()));
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        });

        List<Integer> result = Result.countTeams(rating, queries);

        bufferedWriter.write(
                result.stream()
                        .map(Object::toString)
                        .collect(joining("\n"))
                        + "\n");

        bufferedReader.close();
        bufferedWriter.close();
    }
}
