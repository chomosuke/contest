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
     * Complete the 'getGreatestElements' function below.
     *
     * The function is expected to return an INTEGER_ARRAY.
     * The function accepts following parameters:
     * 1. INTEGER_ARRAY arr
     * 2. INTEGER k
     */

    public static List<Integer> getGreatestElements(List<Integer> list, int k) {
        ArrayList<Integer> kGreatests = new ArrayList<>();
        int[] nums = list.stream()
                .mapToInt(Integer::intValue)
                .toArray();

        // reverse index of arr
        int[] indexOfNum = new int[nums.length + 1];
        for (int i = 0; i < nums.length; i++) {
            indexOfNum[nums[i]] = i;
        }

        // point towards the next greatest in arr
        // -1 mean null
        int[] nextGreatestIndex = new int[nums.length];
        int[] prevGreatestIndex = new int[nums.length];
        // populate the index for the whole array
        for (int num = 1; num < nums.length + 1; num++) {
            if (num - 1 > 0) {
                nextGreatestIndex[indexOfNum[num]] = indexOfNum[num - 1];
            } else {
                nextGreatestIndex[indexOfNum[num]] = -1;
            }
            if (num + 1 < indexOfNum.length) {
                prevGreatestIndex[indexOfNum[num]] = indexOfNum[num + 1];
            } else {
                prevGreatestIndex[indexOfNum[num]] = -1;
            }
        }

        int kGreatestI = indexOfNum[nums.length - k + 1];
        for (int i = nums.length - 1; i >= k - 1; i--) {
            kGreatests.add(nums[kGreatestI]);
            // take away the ith num
            int numIndex = indexOfNum[nums[i]];
            if (prevGreatestIndex[numIndex] != -1)
                nextGreatestIndex[prevGreatestIndex[numIndex]] = nextGreatestIndex[numIndex];
            if (nextGreatestIndex[numIndex] != -1)
                prevGreatestIndex[nextGreatestIndex[numIndex]] = prevGreatestIndex[numIndex];
            // update kGreatest
            if (nums[i] >= nums[kGreatestI]) {
                kGreatestI = nextGreatestIndex[kGreatestI];
            }
        }
        return kGreatests;
    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bufferedWriter = new BufferedWriter(new OutputStreamWriter(System.out));

        int arrCount = Integer.parseInt(bufferedReader.readLine().trim());

        List<Integer> arr = IntStream.range(0, arrCount).mapToObj(i -> {
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

        List<Integer> result = Result.getGreatestElements(arr, k);

        bufferedWriter.write(
                result.stream()
                        .map(Object::toString)
                        .collect(joining("\n"))
                        + "\n");

        bufferedReader.close();
        bufferedWriter.close();
    }
}
