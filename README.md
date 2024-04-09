# Numeric Integrator in vanilla rust

This is a custom made numeric integrator for differential equations of one variable in any form (I think).

## Running

Use the examples as a reference.
To plot the `data.dat` simply use matplotlib or using gnuplot:

```bash
cargo run --example integration
gnuplot -p -e "plot 'data.dat' every 500 with lines"
```

## Usage

You have to be very careful with your input and your starting parameter. Otherwise you will get trash out.