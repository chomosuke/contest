public class SpiralMatrixII {

    public static void main(String[] args) {
        int[][] mat = new SpiralMatrixII().generateMatrix(3);
        for (int[] line : mat) {
            for (int num : line) {
                System.out.print(num + " ");
            }
            System.out.print('\n');
        }
    }

    public int[][] generateMatrix(int n) {
        int[][] mat = new int[n][];
        for (int i = 0; i < n; i++) {
            mat[i] = new int[n]; // auto init to zero
        }
        int count = 1;
        mat[0][0] = count;
        int sq = n*n;
        int i = 0, j = 0;
        int dir = 0; // 0 left, 1 down, 2 right, 3 up
        while (count < n*n) {
            if (dir == 0) {
                if (j+1 < n && mat[i][j+1] == 0) {
                    j++;
                    count++;
                    mat[i][j] = count;
                } else {
                    dir++;
                }
            }
            if (dir == 1) {
                if (i+1 < n && mat[i+1][j] == 0) {
                    i++;
                    count++;
                    mat[i][j] = count;
                } else {
                    dir++;
                }
            }
            if (dir == 2) {
                if (j-1 >= 0 && mat[i][j-1] == 0) {
                    j--;
                    count++;
                    mat[i][j] = count;
                } else {
                    dir++;
                }
            }
            if (dir == 3) {
                if (i-1 >= 0 && mat[i-1][j] == 0) {
                    i--;
                    count++;
                    mat[i][j] = count;
                } else {
                    dir = 0;
                }
            }
        }
        return mat;
    }
}
