<?hh // strict
interface Rx {}

class A {

  public static function f(): int {
    return 1;
  }
}

class B extends A implements Rx {

  public function g(): int {
    // should be OK
    return self::f();
  }
}
