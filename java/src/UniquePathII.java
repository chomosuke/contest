import java.util.ArrayList;
// a potentially faster solution to UniquePathII
//class UniquePathII {
//    public int uniquePathsWithObstacles(int[][] obstacleGrid) {
//
//        int m = obstacleGrid.length; // corrispond to i in Index2D
//        int n = obstacleGrid[0].length; // corispond to j in Index2D
//
//        ArrayList<Index2D> obstacles = new ArrayList<>();
//        // find all obstacle
//        for (int i = 0; i < obstacleGrid.length; i++) {
//            for (int j = 0; j < obstacleGrid[i].length; j++) {
//                if (obstacleGrid[i][j] == 0) {
//                    obstacles.add(new Index2D(i, j));
//                }
//            }
//        }
//
//        // note that obstacles are sorted by i
//
//
//
//    }
//
//    private int passThrough(ArrayList<Index2D> obstacles, int m, int n) {
//
//    }
//
//    private int uniquePaths(int m, int n) {
//        if (m <= 0 || n <= 0) {
//            return 0;
//        }
//        m--;
//        n--;
//        int total = m + n;
//        int min = Math.min(m, n);
//        long result = 1;
//        for (int i = 0; i < min; i++) {
//            result *= total - i;
//            result /= i+1;
//        }
//        return (int)result;
//    }
//}
//
//class Index2D {
//    public Index2D (int i, int j) {
//        this.i = i;
//        this.j = j;
//    }
//    public int i;
//    public int j;
//}
