<?hh // strict

function test_function1(darray<string, mixed> $arr): void {
  // OK
  $arr['herp'] = 7;
}


function test_function2((int, int) $arr): void {
  // OK
  $arr[0] = 7;
}


function test_function3(shape('x' => int) $arr): void {
  // OK
  $arr['x'] = 7;
}
