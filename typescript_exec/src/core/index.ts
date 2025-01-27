import {add} from "../../rust/pkg/rust_lib";

export const main = async () => {

  const value1:number = 1;
  const value2:number = 2;

  const result:number=add(value1, value2);
  //const result:number=value1+value2;

  console.log(result);
};


main();
