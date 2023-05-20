module Variants exposing (..)

type E1 = E1A
type U1 = U1Int | U1Float
type alias Empty =
  {   }

type V1 = V1A | V1BU1 | V1CE1 | V1DString | V1EEmpty | V1F | V1GInt
type Casts1 = Casts1AInt | Casts1BFloat
type Casts2 = Casts2AFloat | Casts2BFloat
type Casts3 = Casts3AFloat | Casts3BInt
type Casts4 = Casts4AInt | Casts4BInt
type Casts5 = Casts5AFloat | Casts5BInt
type Casts6 = Casts6A(Float, Int) | Casts6B(Int, Int)
type MyErrno = MyErrnoBad1 | MyErrnoBad2
type alias IsClone =
  { 

  v1: V1
  }



e1Arg : E1 -> ()
e1Arg x = ()
        

e1Result : E1
e1Result  = ()
        

u1Arg : U1 -> ()
u1Arg x = ()
        

u1Result : U1
u1Result  = ()
        

v1Arg : V1 -> ()
v1Arg x = ()
        

v1Result : V1
v1Result  = ()
        

boolArg : Bool -> ()
boolArg x = ()
        

boolResult : Bool
boolResult  = ()
        

optionArg : (Maybe Bool) -> (Maybe ()) -> (Maybe Int) -> (Maybe E1) -> (Maybe Float) -> (Maybe U1) -> (Maybe (Maybe Bool)) -> ()
optionArg a b c d e f g = ()
        

optionResult : { t0: (Maybe Bool) , t1: (Maybe ()) , t2: (Maybe Int) , t3: (Maybe E1) , t4: (Maybe Float) , t5: (Maybe U1) , t6: (Maybe (Maybe Bool)) }
optionResult  = ()
        

casts : Casts1 -> Casts2 -> Casts3 -> Casts4 -> Casts5 -> Casts6 -> { t0: Casts1 , t1: Casts2 , t2: Casts3 , t3: Casts4 , t4: Casts5 , t5: Casts6 }
casts a b c d e f = ()
        

resultArg : (Result () ()) -> (Result () E1) -> (Result E1 ()) -> (Result () ()) -> (Result Int V1) -> (Result String (List Int)) -> ()
resultArg a b c d e f = ()
        

resultResult : { t0: (Result () ()) , t1: (Result () E1) , t2: (Result E1 ()) , t3: (Result () ()) , t4: (Result Int V1) , t5: (Result String (List Int)) }
resultResult  = ()
        

returnResultSugar : (Result Int MyErrno)
returnResultSugar  = ()
        

returnResultSugar2 : (Result () MyErrno)
returnResultSugar2  = ()
        

returnResultSugar3 : (Result MyErrno MyErrno)
returnResultSugar3  = ()
        

returnResultSugar4 : (Result (Int, Int) MyErrno)
returnResultSugar4  = ()
        

returnOptionSugar : (Maybe Int)
returnOptionSugar  = ()
        

returnOptionSugar2 : (Maybe MyErrno)
returnOptionSugar2  = ()
        

resultSimple : (Result Int Int)
resultSimple  = ()
        

isCloneArg : IsClone -> ()
isCloneArg a = ()
        

isCloneReturn : IsClone
isCloneReturn  = ()
        

returnNamedOption : (Maybe Int)
returnNamedOption  = ()
        

returnNamedResult : (Result Int MyErrno)
returnNamedResult  = ()
        