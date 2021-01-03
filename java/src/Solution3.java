public class Solution3 {

    public static void main(String[] args) {
        char[][] board = new char[][]{
                {'.', '.', '9', '7', '4', '8', '.', '.', '.'},
                {'7', '.', '.', '.', '.', '.', '.', '.', '.'},
                {'.', '2', '.', '1', '.', '9', '.', '.', '.'},
                {'.', '.', '7', '.', '.', '.', '2', '4', '.'},
                {'.', '6', '4', '.', '1', '.', '5', '9', '.'},
                {'.', '9', '8', '.', '.', '.', '3', '.', '.'},
                {'.', '.', '.', '8', '.', '3', '.', '2', '.'},
                {'.', '.', '.', '.', '.', '.', '.', '.', '6'},
                {'.', '.', '.', '2', '7', '5', '9', '.', '.'}
        };
        solveSudoku(board);
        for (char[] chars : board) System.out.println(chars);
    }

    public static void solveSudoku(char[][] board) {
        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[i].length; j++) {
                if (board[i][j] == '.')
                    board[i][j] = (char) -1;
                else
                    board[i][j] -= '1';
            }
        }

        solveSudoku(board, 0);

        for (int i = 0; i < board.length; i++) {
            for (int j = 0; j < board[i].length; j++) {
                board[i][j] += '1';
            }
        }

    }

    // true if solved
    private static boolean solveSudoku(char[][] board, int index) {

        int i = index / 9;
        int j = index % 9;
        while (board[i][j] != (char) -1) {
            index++;
            i = index / 9;
            j = index % 9;
            if (i >= 9) {
                return true;
            }
        }

        int si = i / 3 * 3;
        int sj = j / 3 * 3;

        boolean[] contain = new boolean[9]; // initialized to false
        for (int k = 0; k < 9; k++) {
            if (board[i][k] != (char) -1)
                contain[board[i][k]] = true;
            if (board[k][j] != (char) -1)
                contain[board[k][j]] = true;
            if (board[si + (k % 3)][sj + (k / 3)] != (char) -1)
                contain[board[si + (k % 3)][sj + (k / 3)]] = true;
        }

        for (int k = 0; k < 9; k++) {
            if (!contain[k]) {
                board[i][j] = (char) k;
                if (index >= 80 || solveSudoku(board, index + 1))
                    return true;
            }
        }

        board[i][j] = (char) -1;

        return false;
    }
}
