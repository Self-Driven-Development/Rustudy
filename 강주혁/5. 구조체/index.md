# 5. 구조체

> 구조체는 서로 관련된 데이터를 묶어서 사용할 때 유용하다.

## 5.1 구조체의 정의 및 인스턴스화

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

정의한 구조체 사용하려면 인스턴스화 해야한다.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

가변 인스턴스라면 특정 필드의 값을 변경할 수 있다. 이때 가변성은 해당 인스턴스의 전체가 지니게 된다.

### 필드 초기화 축약법(Field Init Shorthand)

`email: email` 대신 `email`만 써도 된다.

### 구조체 업데이트 문법(Struct Update Syntax)

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
```

이 때 `user2`를 생성한 이후에는 `user1`을 더이상 사용할 수 없다. `user1`의 모든 필드를 사용했기 때문이다. 특정 값 몇 개만 사용한다면 `user1`이 유효하다.

### 튜플 구조체(Tuple Structs)

구조체 자체에는 이름을 지어 의미를 주지만, 이를 구성하는 필드에는 이름을 붙이지 않고 `타입만 적어 넣은 형태`

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Color 와 Point는 서로 다른 타입이다.(필드 구성이 같더라도)

### 유사 유닛 구조체(Unit-Like Structs)

필드를 가지지 않는 구조체

```rust
struct UnitLikeStruct;

fn main() {
    let unit_like_struct = UnitLikeStruct;
}
```

#### 구조체 데이터의 소유권

구조체가 참조자를 이용해 소유권을 갖지 않는 데이터도 저장할 수 있지만, `라이프타임`을 활용해야 한다. 라이프타임을 사용하면 구조체가 존재하는 동안에 구조체 내 참조자가 가리키는 데이터의 `유효함`을 보장받을 수 있기 때문이다.

## 5.2 구조체를 사용한 예제 프로그램

`width`, `height` 이 두 매개변수의 값이 연관되어 있다는 것 명확하게 표현하기 위해 구조체를 사용한다.

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

하지만 이렇게 하면 인덱스에 어떤 값이 들어가야 하는지 알아야 한다. 구조체로 리팩토링하여 코드에 더 많은 의미를 담자.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### 트레이트 파생으로 유용한 기능 추가하기

`#[derive(Debug)]` 어노테이션을 붙이면 `println!` 매크로에 `{:?}` 포맷 문자열을 사용해 출력할 수 있다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

`{:#?}` 포맷 문자열을 사용하면 좀 더 보기 좋게 출력된다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}
```

`dbg!` 매크로를 사용하면 좀 더 편하게 디버깅할 수 있다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rect1);
}
```

## 5.3 메서드 문법

함수와 비슷하지만, 구조체 컨텍스트에 정의되고(열거형, 트레이트 객체 안에 정의되기도 함), 첫 번째 매개변수는 항상 `self`이다.
`self` 매개변수는 메서드를 호출하고 있는 구조체 인스턴스를 나타낸다.

### 연관 함수

`impl` 블록 내에 구현된 모든 함수를 연관 함수(associated function)라고 부른다.

- 구조체의 새 인스턴스를 반환하는 생성자로 자주 활용됨 ex) new

여러개의 impl 블록으로 나눌 수도 있다.
