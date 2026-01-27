export default function hello() {
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");
  console.log("hi");

  const foo = () => {
    console.log("hi");
  };

  const obj = {
    foo: () => {},
    bar() {},
    baz: function () {},
  };

  function test(x: number) {
    if (x > 10 && x < 20) {
      for (let i = 0; i < x; i++) {
        if (i % 2 === 0) console.log(i);
      }
    } else {
      return x > 0 ? "positive" : "negative";
    }
  }
}
