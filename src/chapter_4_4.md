# 结构体

结构体
struct 或 structure ，自定义数据类型，允许包装和命名多个相关值，形成一个组合。
Rust 包含三种结构体： 命名结构体、元组结构体、单元结构体。
结构体与元组之间差异
* 结构体灵活，可以任意访问不需要索引
* 结构体需要命名各部分数据表明其含义

命名结构体
定义结构体 

例子1
```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
使用结构体，通过为结构体内每个字段指定具体的值来创建这个结构体，创建一个结构体 需要以结构体名称开头接着花括号内写 key: value 类型内容 key 结构体字段，value 结构体字段值，实例中定义字段值可以与定义中的顺序不同
创建结构体实例
例子2
```rust
let user1 = User {
  active: true,
  username: "lalla".to_string(),
  email: "lalla@test.com".to_string(),
  sign_in_count: 62789346,
}
```
查看结构体所有内容和部分内容，为了从结构体中获取某个特定的值，可以使用点号
例子3
```rust
#[derive(Debug)]   // 想要查看结构体所有内容必须要添加该内容
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
      active: true,
      username: "lalla".to_string(),
      email: "lalla@test.com".to_string(),
      sign_in_count: 62789346,
    };
    println!("{}", user1.username);
    println!("{}", user1.active);
    println!("{:?}", user1);
}
```

改变结构体字段值
如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。
例子4 
```rust
let mut user1 = User {
  active: true,
  username: "lalla".to_string(),
  email: "lalla@test.com".to_string(),
  sign_in_count: 62789346,
};
println!("{}", user1.active);
user1.active = false;
println!("{}", user1.active);
```
整个实例必须是可变的，Rust 允许只将某个字段定义为可变，同其他表达式一样，我们可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。
例子5
```rust
// struct User  省略
fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: email.to_string(),
        email: username.to_string(),
        sign_in_count: 1,
    }
}

fn main() {
    let user2 = build_user("sdf@sd.f","sdf");
    println!("{:?}",user2);
}

// 结果
User { active: true, username: "sdf@sd.f", email: "sdf", sign_in_count: 1 }
```

上面代码中形参与结构体字段命名相同，函数返回结构体重新声明了结构体字段有些重复，有简便写法。（函数形参与结构体字段名称相同，类型相同时如果要返回结构体可以直接写字段名）
例子6
```rust
// struct User  省略
fn build_user(email: &str, username: String) -> User {
    let email = email.to_string();
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

fn main() {
    let user2 = build_user("sdf@sd.f","sdf");
    println!("{:?}",user2);
}

// 结果
// User { active: true, username: "sdf@sd.f", email: "sdf", sign_in_count: 1 }
```

结构体更新语法
使用旧实例的值改变其他部分创建出一个新的实例，可以通过结构体更新语法实现。
例子7： 在例子4基础上 通过修改部分 user1 实例创建一个新的 user3
```rust
let user3 = User {
  active: user1.active,
  email: "uoi@oo.we",
  username: "uoi",
  sign_in_count: "892334",
};
```
推荐写法：(当结构体增加字段值时，此段代码不用修改)
例子8
```rust
let user4 = User {
  email: "uoi@oo.we",
  username: "uoi",
  sign_in_count: "892334",
  ..user1
};
```
注意：
结构更新语法就像带有 = 的赋值，因为它移动了数据，就像我们在“变量与数据交互的方式（一）：移动”部分讲到的一样。在这个例子中，总体上说我们在创建 user4 后不能就再使用 user1 了，因为 user1 的 username 字段中的 String 被移到 user2 中。如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么 user1 在创建 user2 后仍然有效。active 和 sign_in_count 的类型是实现 Copy trait 的类型，所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用。
元组结构体
元组结构体有结构体名称提供的含义，没有具体字段名，只有字段类型。
使用场景：想给元组命名，区分其他元组
例子9
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
  let back = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}
```
注意 black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例。你定义的每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型。例如，一个获取 Color 类型参数的函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。在其他方面，元组结构体实例类似于元组，你可以将它们解构为单独的部分，也可以使用 . 后跟索引来访问单独的值，等等.
访问元组结构体字段
例子10.1
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main(){
  let index = 0;
  let back = Color(0, 1, 0);
  let origin = Point(0, 0, 0);
  println!("back value is {} {} {}",back.0, back.1, back.2);
  println!("back value is {}",back.index);
}
```

结构元组结构体
例子10.2
```rust
struct Point(i32, i32, i32);

fn main(){
  let (x, y, z)= Point(0, 9, 1);
  println!("back value is {} {} {}",x, y, z);
}
```

单元结构体
可以定义一个没有任何单元的结构体，类似 unit 类型。
使用场景：在某个类型上实现 trait 但不需要在类型中存储数据。
例子11
```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

结构体数据所有权
例子1 中使用 String 没有使用 &str ,是有意为之的选择，我们想要结构体拥有它所有数据，为此只要整个结构体有效的话其数据也是有效的。
如果要结构体使用其他对象的引用需要用声明周期
方法 method
方法与函数类似，都使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时执行的代码。不过方法与函数时不同的，方法在结构体的上下文中被定义（或者是枚举或者trait 对象上下文）并且第一个参数 是 self，代表调用该结构体的方法。
例子 12
```rust
struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
  // &self 实际上是 self: &Self 的缩写。
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

fn main() {
    let r = Rectangle {
        width: 5.6,
        length: 9.0,
    };
    println!("area is {}", r.area());
}
// 结果
area is 50.4
```
注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我们在 rectangle: &Rectangle 中做的那样。方法可以选择获得 self 的所有权，或者像我们这里一样不可变地借用 self，或者可变地借用 self，就跟其他参数一样。

这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。
请注意，我们可以选择将方法的名称与结构中的一个字段相同。例如，我们可以在 Rectangle 上定义一个方法，并命名为 width：
例子13
```rust
impl Rectangle {
  // &self 实际上是 self: &Self 的缩写。
    fn width(&self) -> f64 {
        self.width
    }
}

fn main() {
    let r = Rectangle {
        width: 5.6,
        length: 9.0,
    };
    println!("width is {}", r.width());
}
// 结果
width is 5.6
```
更多参数的方法
我们让一个 Rectangle 的实例获取另一个 Rectangle 实例，如果 self （第一个 Rectangle）能完全包含第二个长方形则返回 true；否则返回 false。
例子14
```rust
impl Rectangle {
    fn can_hold(&self,r: &Rectangle)-> bool{
        self.width > r.width && self.length > r.length 
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 5.6,
        length: 9.0,
    };
    let rect2 = Rectangle {
        width: 5.3,
        length: 8.0,
    };
    let rect3 = Rectangle {
        width: 9.0,
        length: 8.0,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
// 结果
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

关联函数
所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数：在 String 类型上定义的 String::from 函数。
不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。这些函数的名称通常为 new ，但 new 并不是一个关键字。例如我们可以提供一个叫做 square 关联函数，它接受一个维度参数并且同时作为宽和高，这样可以更轻松的创建一个正方形 Rectangle 而不必指定两次同样的值：
例子15
```rust
impl Rectangle {
    fn square(size: f64) -> Self {
        Self {
            width: size,
            length: size,
        } 
    }
}

println!("square is {:?}", Rectangle::square(12.0));
// 结果
square is Rectangle { width: 12.0, length: 12.0 }
```

多个 impl 块
每个结构体允许有多个 impl 块
例子
```rust
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle1 = Rectangle::new(12, 10);
    let rectangle2 = Rectangle::new(19, 20);
    let rectangle3 = Rectangle {
        width: 19,
        height: 20,
    };
    println!("rectangle1 area is {}", rectangle1.area());
    println!(
        "can rectangle1 hold rectangle2: {}",
        rectangle1.can_hold(&rectangle2)
    );
    println!("rectangle3 is {:?}", rectangle3);
}
// 结果
rectangle1 area is 120
can rectangle1 hold rectangle2: false
rectangle3 is Rectangle { width: 19, height: 20 }
```
