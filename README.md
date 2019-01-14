# Jazz (Unmaintaned, please see https://github.com/jazz-lang/
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/playXE/Jazz/blob/master/LICENSE)


Jazz is a register-based virtual machine and lightweight programming language

Jazz is heavily inspired by [Gravity](https://marcobambini.github.io/gravity/#/) language

## Goals
* **Clear and simple syntax**
* **Integration with Rust**
* **Interfacing with other languages than Rust**
* **Make VM suitable for object oriented programming**

## Non-goals
* Write simple book for learning Jazz programming language
* JIT compilation
* Generating bytecode files


# Example code

```swift
func factorial(num) {
    if num == 0 {
        return 1;
    } else {
        return num * factorial(num - 1);
    }
}

func main() {
    print(factorial(5));
    return 0;
}
```

```swift
class Vector2 {
    var x;
    var y;
    func init(a,b) {
        this.x = a;
        this.y = b;
        return this;
    }

    func toString() {
        return concat("(",this.x,";",this.y,")");
    }
}

func main() {
    var vector = Vector2(2,-2);
    print(vector.toString());
}

```
