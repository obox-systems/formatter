
f1( ){} 


# input
f1(){} 
# expected
f1(){}

# input
let x:int=3 
# expected
let x : int = 3

# input
f1( x ),f2(y) , f3()
# expected
f1( x ) , f2( y ) , f3()

# input
[1,b,3]
# expected
[ 1, b, 3 ]

# input
{a:1,b:2}
# expected
{ a : 1, b : 3 }