package day02;

import java.util.Arrays;
import java.util.Collections;

public class GiftBox {
    int l;
    int w;
    int h;

    GiftBox(int l, int w, int h) {
        this.l = l;
        this.w = w;
        this.h = h;
    }

    static GiftBox parse(String s) {
        String[] stringDimensions = s.trim().split("x");
        GiftBox g = new GiftBox(
                Integer.parseInt(stringDimensions[0]),
                Integer.parseInt(stringDimensions[1]),
                Integer.parseInt(stringDimensions[2]));
        return g;
    }

    int wrappingPaperRequired() {
        int a = (l * w);
        int b = (l * h);
        int c = (w * h);
        int surfaceArea = 2 * a + 2 * b + 2 * c;
        int smallestSide = Math.min(a, Math.min(b, c));
        return surfaceArea + smallestSide;
    }

    int ribbonRequired() {
        int[] sides = new int[] { l, w, h };
        Arrays.sort(sides);

        int ribbonLengthAroundBox = 2 * sides[0] + 2 * sides[1];
        int ribbonBow = l * w * h;
        return ribbonLengthAroundBox + ribbonBow;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }

        if (obj == this) {
            return true;
        }

        GiftBox other = (GiftBox) obj;

        return this.l == other.l && this.w == other.w && this.h == other.h;

    }
}
