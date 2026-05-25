// 变量类型注解
const x: number = 42;
let name: string = "hello";

// 函数参数类型
function add(a: number, b: number) {
  return a + b;
}

// interface 声明(应被跳过)
interface User {
  name: string;
  age: number;
}

// type 声明(应被跳过) 
type Point = {
  x: number;
  y: number;
};

// enum 声明(应被跳过)
enum Color {
  Red = "red",
  Green = "green"
}

// 类型断言
const el = document.getElementById("app");
