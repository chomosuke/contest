module Main where
import Control.Monad (replicateM, when)

-- helper for reading numbers
getInts :: IO [Int]
getInts = fmap getInts getLine
  where
    isNum c = c `elem` ['0'..'9']
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

main :: IO ()
main = do
  str <- getLine
  print (solve str)

solve :: String -> Int
solve str = maximum (map ((1+) . length) (linesBy isVowels str))

isVowels :: Char -> Bool
isVowels = (`elem` "AEIOUY")
