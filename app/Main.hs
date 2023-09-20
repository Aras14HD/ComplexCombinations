main = do
  print (combs [ 9, 9 ])

mapIndex f l = zipWith f l [0..]

combs :: [Int] -> Int
combs [1] = 1
combs [] = 0
combs arr = sum (combMap arr) 

combMap :: [Int] -> [Int] 
combMap arr = filter (>0) (mapIndex (\x i -> combFilter x i (arr ++ [0])) arr)

combFilter:: Int -> Int -> [Int] -> Int
combFilter x i arr = (x - (arr !! (i+1))) * combs (filter (>0) (take i arr ++ ((x-1): drop (i+1) arr)))