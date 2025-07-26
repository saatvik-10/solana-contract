export class Counter {
  //Matches Rust struct; holds your shared data
  value: number;
  constructor(props: { value: number }) {
    this.value = props.value;
  }
}

// Updated schema for newer Borsh versions
export const CounterSchema = {
  struct: {
    value: 'u32',
  },
};
