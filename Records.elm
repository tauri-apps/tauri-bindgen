module Records exposing (..)

type alias Empty =
  {   }
{-| A record containing two scalar fields
that both have the same type
-}
type alias Scalars =
  { 
{-| A record containing two scalar fields
that both have the same type
-}
  a: Int
  , 
{-| A record containing two scalar fields
that both have the same type
-}
  b: Int
  }
{-| A record that is really just flags
All of the fields are bool
-}
type alias ReallyFlags =
  { 
{-| A record that is really just flags
All of the fields are bool
-}
  a: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  b: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  c: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  d: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  e: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  f: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  g: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  h: Bool
  , 
{-| A record that is really just flags
All of the fields are bool
-}
  i: Bool
  }

type alias Aggregates =
  { 

  a: Scalars
  , 

  b: Int
  , 

  c: Empty
  , 

  d: String
  , 

  e: ReallyFlags
  }

type alias IntTypedef = Int

type alias TupleTypedef2 = (IntTypedef)



tupleArg : (String, Int) -> ()
tupleArg x = ()
        

tupleResult : (String, Int)
tupleResult  = ()
        

emptyArg : Empty -> ()
emptyArg x = ()
        

emptyResult : Empty
emptyResult  = ()
        

scalarArg : Scalars -> ()
scalarArg x = ()
        

scalarResult : Scalars
scalarResult  = ()
        

flagsArg : ReallyFlags -> ()
flagsArg x = ()
        

flagsResult : ReallyFlags
flagsResult  = ()
        

aggregateArg : Aggregates -> ()
aggregateArg x = ()
        

aggregateResult : Aggregates
aggregateResult  = ()
        

typedefInout : TupleTypedef2 -> Int
typedefInout e = ()
        