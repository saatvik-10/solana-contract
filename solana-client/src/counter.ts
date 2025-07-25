import * as borsh from 'borsh';

export class Counter {
  //Matches Rust struct; holds your shared data
  value: number;
  constructor(props: { value: number }) {
    this.value = props.value;
  }
}

export const CounterSchema = new Map([
  //Ensures Rust and TS read/write bytes identically
  [
    Counter,
    {
      kind: 'struct',
      fields: [['value', 'u32']],
    },
  ],
]);
