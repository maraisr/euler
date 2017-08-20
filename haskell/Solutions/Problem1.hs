module Solutions.Problem1 where

threeOrFive :: Integer -> Bool
threeOrFive x = x `mod` 3 == 0 || x `mod` 5 == 0

sol :: Integer -> Integer
sol = sum . filter threeOrFive . enumFromTo 1 . subtract 1

solution1 = sol 1000
