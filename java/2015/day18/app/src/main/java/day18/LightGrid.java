package day18;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.Arrays;

enum GridPos {
    A,
    B
}

public class LightGrid {
    private final boolean[][] gridA;
    private final boolean[][] gridB;

    GridPos gridPos = GridPos.A;

    LightGrid(int size){
        this.gridA = new boolean[size][size];
        this.gridB = new boolean[size][size];
    }

    LightGrid(int size, boolean[][] grid){
        this.gridA = grid;
        this.gridB = new boolean[grid.length][grid.length];

        for (int y=0; y<grid.length; y++){
            this.gridB[y] = Arrays.copyOf(grid[y], grid.length);
        }
    }

    public static LightGrid parse(InputStream input) throws IOException {

        BufferedReader br = new BufferedReader(new InputStreamReader(input));

        boolean[][] grid = null;

        int y = 0;
        String line = br.readLine();
        while (line != null) {

            byte[] lineBytes = line.getBytes();

            if(grid == null){
                grid = new boolean[lineBytes.length][lineBytes.length];
            }

            for (int x=0; x<lineBytes.length; x++){
                switch (lineBytes[x]){
                    case '#':
                        grid[y][x] = true;
                    break;
                    default:
                        grid[y][x] = false;
                    break;
                }
            }

            y++;
            line = br.readLine();
        }

        return new LightGrid(grid.length, grid);
    }

    public void iterate() {
        boolean[][] displayGrid;
        boolean[][] workingGrid;
        if (gridPos == GridPos.A) {
            displayGrid = gridB;
            workingGrid = gridA;
            gridPos = GridPos.B;
        } else {
            displayGrid = gridA;
            workingGrid = gridB;
            gridPos = GridPos.A;
        }

        for (int y=0; y < displayGrid.length; y++){
            for (int x=0; x < displayGrid.length; x++){
                int nn = getNeighborNum(workingGrid, x, y);
                if (workingGrid[y][x]){
                    if (nn == 2 || nn==3){
                        displayGrid[y][x] = true;
                    } else {
                        displayGrid[y][x] = false;
                    }
                } else {
                    if (nn == 3){
                        displayGrid[y][x] = true;
                    } else {
                        displayGrid[y][x] = false;
                    }
                }

            }
        }
    }

    public void lightCorners(){
        boolean[][] grid = gridA;
        if (gridPos == GridPos.B) {
            grid = gridB;
        }

        grid[0][0] = true;
        grid[grid.length-1][0] = true;
        grid[0][grid.length-1] = true;
        grid[grid.length-1][grid.length-1] = true;
    }

    private int getNeighborNum(boolean[][] grid, int x, int y){

        int nn=0;

        for(int yy=y-1; yy<=y+1; yy++){
            for(int xx=x-1; xx<=x+1; xx++){
                if (xx<0 || yy<0){
                    continue;
                }

                if (xx >= grid.length || yy >= grid.length){
                    continue;
                }

                if(xx == x && yy == y){
                    continue;
                }
                
                if (grid[yy][xx]){
                    nn++;
                }
            }
        } 

        return nn;
    }

    public int getCount() {

        boolean[][] grid = gridA;
        if (this.gridPos == GridPos.B){
            grid = gridB;
        }

        int count = 0;
        for (boolean[] row: grid){
            for (boolean light: row){
                if (light){
                    count++;
                }
            }
        }
        return count;
    }

    public String getOutput(){
        boolean[][] currentGrid = gridA;
        if (gridPos == GridPos.B){
            currentGrid = gridB;
        } 

        StringBuilder sb = new StringBuilder();
        for(boolean[] gridRow: currentGrid){
            for(boolean light: gridRow){
                if (light){
                    sb.append('#');
                } else {
                    sb.append('.');
                }
            }
            sb.append("\n");
        }

        return sb.toString();
    }
}

