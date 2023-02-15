import java.util.ArrayList;

class Solution {
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
        Cell[][] cells = new Cell[9][9];
        for (int i = 0; i < cells.length; i++) {
            for (int j = 0; j < cells[i].length; j++) {
                cells[i][j] = new Cell();
            }
        }

        Group group;
        for (int i = 0; i < 9; i++) {
            group = new Group();
            for (int j = 0; j < 9; j++) {
                group.addCell(cells[i][j]);
            }
            group = new Group();
            for (int j = 0; j < 9; j++) {
                group.addCell(cells[j][i]);
            }
        }

        for (int i = 0; i < 9; i++) {
            int x = i * 3 % 9;
            int y = i / 3 * 3;
            group = new Group();
            for (int j = 0; j < 9; j++) {
                group.addCell(cells[x+j%3][y+j/3]);
            }
        }

        for (int i = 0; i < cells.length; i++) {
            for (int j = 0; j < cells[i].length; j++) {
                cells[i][j].setCell(board[i][j]);
            }
        }

        for (int i = 0; i < cells.length; i++) {
            for (int j = 0; j < cells[i].length; j++) {
                if (cells[i][j].isSolved()) {
                    System.out.print(cells[i][j].getSolved() + ", ");
                    board[i][j] = cells[i][j].getSolved();
                }
                else {
                    System.out.print(" , ");
                }
            }
            System.out.print('\n');
        }
    }
}

class Cell {
    private final boolean[] possibility = new boolean[] {true, true, true, true, true, true, true, true, true};
    private Character solved = null;
    public boolean isSolved() {
        return solved != null;
    }

    public Character getSolved() { return solved; }

    public void solve(char c) {
        solved = c;
        for (Group group : groups)
            group.notifySolved(this);
    }
    public void removePossibility(char c) {
        if (isSolved())
            return;

        possibility[c - '1'] = false;
        for (Group group : groups)
            group.removePossibility(this, c);

        checkSolved();
    }

    public void checkSolved() {
        int count = 0;
        for (boolean possible : possibility)
            if (possible)
                count++;

        if (count == 1)
            for (int i = 0; i < possibility.length; i++)
                if (possibility[i]) {
                    solve((char)(i + '1'));
                    return;
                }
    }

    public void setCell(char c) {
        if (c != '.') {
            for (int i = 0; i < possibility.length; i++) {
                if (i != c-'1')
                    possibility[i] = false;
            }
            solve(c);
        }
    }

    private final ArrayList<Group> groups = new ArrayList<>();
    public void addGroup(Group g) {
        groups.add(g);
    }
}

class Group {
    public Group() {
        for (int i = 0; i < 9; i++)
            possibilities.add(new ArrayList<>());
    }

    public void notifySolved(Cell c) {
        for (Cell cell : cells) {
            if (c != cell)
                cell.removePossibility(c.getSolved());
        }
    }

    public void removePossibility(Cell cell, char c) {
        int i = c - '1';
        if (possibilities.get(i).remove(cell))
            checkSolved(i);
    }
    public void checkSolved(int i) {
        if (possibilities.get(i).size() == 1) {
            possibilities.get(i).get(0).setCell((char)(i + '1'));
        }
    }
    private final ArrayList<ArrayList<Cell>> possibilities = new ArrayList<>();

    private final ArrayList<Cell> cells = new ArrayList<>();

    public void addCell(Cell c) {
        cells.add(c);
        c.addGroup(this);
        for (ArrayList<Cell> possibility : possibilities) {
            possibility.add(c);
        }
    }
}