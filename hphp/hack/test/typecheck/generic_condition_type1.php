<?hh // partial

<<__Pure, __AtMostRxAsArgs>>
function varray_map<Tv1, Tv2>(
  <<__AtMostRxAsFunc>>(function(Tv1): Tv2) $value_func,
  <<__OnlyRxIfImpl(\HH\Rx\Traversable::class)>>Traversable<Tv1> $traversable,
): varray<Tv2> {
  $result = varray[];
  foreach ($traversable as $value) {
    $result[] = $value_func($value);
  }
  return $result;
}

<<__Pure>>
function f(varray<int> $x) {
  // OK
  varray_map($v ==> $v, $x);
}
