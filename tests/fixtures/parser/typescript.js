// 变量类型注解
const x: number = 42;
let name: string = "hello";
const arr: Array<number> = [1, 2, 3];

// 函数参数和返回值类型
function add(a: number, b: number): number {
  return a + b;
}

// 联合类型
function format(value: string | number): string {
  return String(value);
}

// 可选参数
function greet(name: string, greeting?: string): string {
  return greeting ? greeting + " " + name : "Hello " + name;
}

// interface 声明(应被跳过)
interface User {
  name: string;
  age: number;
  email?: string;
}

// type 声明(应被跳过)
type Point = {
  x: number;
  y: number;
};

type ID = string | number;

// enum 声明(应被跳过)
enum Color {
  Red = "red",
  Green = "green",
  Blue = "blue"
}

// namespace 声明(应被跳过)
namespace Utils {
  export function log(msg: string): void {
    console.log(msg);
  }
}

// declare 声明(应被跳过)
declare global {
  interface Window {
    myCustomProp: string;
  }
}

// 交叉类型
type WithId = { id: string } & { created: Date };

// 泛型函数
function identity<T>(arg: T): T {
  return arg;
}

// 类型断言
const input = document.getElementById("app");
const div = input as HTMLDivElement;

// 复杂泛型
const map: Map<string, Array<number>> = new Map();
