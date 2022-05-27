module Main where
import Control.Monad (replicateM)

getInts :: IO [Int]
getInts = fmap getInts getLine
  where
    isNum c = c `elem` "1234567890"
    getInts s =
      let h = read (takeWhile isNum s) :: Int
          t = getInts (dropWhile (not . isNum) (dropWhile isNum s)) :: [Int]
      in h:t

getInt = fmap head getInts

main :: IO ()
main = do
  n <- getInt
  hs <- replicateM n getInt
  print (solve hs 0)

solve :: [Int] -> Int -> Int
solve [h] currentHeight = abs (h - currentHeight) + 1
solve (h:hs) currentHeight = abs (h - currentHeight) + 2 + solve hs h
solve [] _ = 0
