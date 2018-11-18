# Jazz Virtual Machine
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/playXE/Jazz/blob/master/LICENSE)


Jazz is a register-based virtual machine and lighweight programming language

Jazz is heavily inspired by [Gravity](https://marcobambini.github.io/gravity/#/) language


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
