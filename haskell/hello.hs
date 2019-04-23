hello x = "Hello " ++ x

add :: Int -> Int -> Int
add x y = x + y

addTuple :: (Int, Int) -> Int
addTuple (x,y) = x + y

fact :: Integer -> Integer
fact n | n == 0 = 1
       | n /= 0 = n * fact (n-1)
        