public class TrappingRainWater {
    public int trap(int[] height) {

        if (height.length == 0) {
            return 0;
        }

        int l = 1;
        int r = height.length - 2;
        int heightL = height[l-1];
        int heightR = height[r+1];
        int volume = 0;
        while(l <= r) {
            while (heightL <= heightR && l <= r) {
                // make l bigger while counting volume to find the next height

                int newVolume = heightL - height[l];
                if (newVolume > 0) {
                    volume += newVolume;
                } else {
                    // new heightL
                    heightL = height[l];
                }
                l++;
            }

            while (heightL >= heightR && l <= r) {
                // make r smaller while counting volume to find the next height

                int newVolume = heightR - height[r];
                if (newVolume > 0) {
                    volume += newVolume;
                } else {
                    // new heightR
                    heightR = height[r];
                }
                r--;
            }

        }


        return volume;
    }


}
