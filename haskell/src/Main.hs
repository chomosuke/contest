module Main where
import Control.Monad (replicateM)
import Data.List (sort)

-- helper for reading numbers
getInts :: IO [Int]
getInts = fmap getInts getLine
  where
    isNum c = c `elem` ['0'..'9']
    getInts [] = []
    getInts s =
      let h = read (takeWhile isNum s) :: Int
          t = getInts (dropWhile (not . isNum) (dropWhile isNum s)) :: [Int]
      in h:t
getInt :: IO Int
getInt = fmap head getInts
linesBy :: (a -> Bool) -> [a] -> [[a]]
linesBy predicate = foldr f []
  where
    f e (firstList:rest) = if predicate e
      then []:firstList:rest
      else (e:firstList):rest
    f e [] = if predicate e
      then [[]]
      else [[e]]
wordsBy :: (a -> Bool) -> [a] -> [[a]]
wordsBy predicate xs = filter (not . null) (linesBy predicate xs)
join :: [a] -> [[a]] -> [a]
join delimiter [x] = x
join delimiter (x:xs) = x ++ delimiter ++ join delimiter xs
join delimiter [] = []
allSame :: Eq a => [a] -> Bool
allSame (x1:x2:xs) = x1 == x2 && allSame (x2:xs)
allSame _ = True

main :: IO ()
main = do
  n <- getInt
  as <- getInts
  putStrLn (join " " (map show (solve n as)))

solve :: Int -> [Int] -> [Int]
solve n as = if allSame sorted then [-1] else sorted
  where sorted = sort as
